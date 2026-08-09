use anyhow::{Context, Result, anyhow, bail};
use async_trait::async_trait;
use reqwest::{Client, Url, header};
use serde::Serialize;

use crate::{
    config::GrokConfig,
    providers::{SpeechCodec, SpeechResult, TextToSpeechProvider, TtsSpeechRequest},
};

const FALLBACK_BIT_RATE: u32 = 128_000;
const SUPPORTED_SAMPLE_RATES: &[u32] = &[8_000, 16_000, 22_050, 24_000, 44_100, 48_000];
const SUPPORTED_MP3_BIT_RATES: &[u32] = &[32_000, 64_000, 96_000, 128_000, 192_000];

pub struct GrokProvider {
    client: Client,
    config: GrokConfig,
}

#[derive(Clone, Copy, Debug)]
struct ParsedOutputFormat {
    codec: SpeechCodec,
    sample_rate: Option<u32>,
    bit_rate: Option<u32>,
}

#[derive(Debug, Serialize)]
struct SpeechPayload<'a> {
    text: &'a str,
    voice_id: &'a str,
    output_format: OutputFormat,
    language: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    speed: Option<f64>,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct OutputFormat {
    codec: SpeechCodec,
    sample_rate: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    bit_rate: Option<u32>,
}

impl GrokProvider {
    #[must_use]
    pub fn new(client: Client, config: GrokConfig) -> Self {
        Self { client, config }
    }

    fn resolve_output_format(&self, request: &TtsSpeechRequest) -> Result<OutputFormat> {
        let requested = request.output.unwrap_or_default();
        let configured = parse_output_format_token(&self.config.default_output_format)?;
        let codec = requested
            .codec
            .or(configured.map(|format| format.codec))
            .unwrap_or(SpeechCodec::Mp3);
        let sample_rate = requested
            .sample_rate
            .or_else(|| {
                configured
                    .filter(|format| format.codec == codec)
                    .and_then(|format| format.sample_rate)
            })
            .unwrap_or(self.config.default_sample_rate);

        if !SUPPORTED_SAMPLE_RATES.contains(&sample_rate) {
            bail!(
                "Unsupported xAI sample rate '{sample_rate}'. Allowed sample rates: 8000, 16000, 22050, 24000, 44100, 48000."
            );
        }

        let bit_rate = if codec == SpeechCodec::Mp3 {
            let bit_rate = requested
                .bit_rate
                .or_else(|| {
                    configured
                        .filter(|format| format.codec == codec && requested.sample_rate.is_none())
                        .and_then(|format| format.bit_rate)
                })
                .or(self.config.default_bit_rate)
                .unwrap_or(FALLBACK_BIT_RATE);
            if !SUPPORTED_MP3_BIT_RATES.contains(&bit_rate) {
                bail!(
                    "Unsupported xAI MP3 bit rate '{bit_rate}'. Allowed bit rates: 32000, 64000, 96000, 128000, 192000."
                );
            }
            Some(bit_rate)
        } else {
            if requested.bit_rate.is_some() {
                bail!("xAI codec '{}' does not support bit rate.", codec.as_str());
            }
            None
        };

        Ok(OutputFormat {
            codec,
            sample_rate,
            bit_rate,
        })
    }
}

#[async_trait]
impl TextToSpeechProvider for GrokProvider {
    fn name(&self) -> &'static str {
        "grok"
    }

    async fn speak(&self, request: &TtsSpeechRequest) -> Result<SpeechResult> {
        let api_key = non_empty(&self.config.api_key)
            .ok_or_else(|| anyhow!("Missing XAI_API_KEY environment variable."))?;
        if request.model.as_deref().and_then(non_empty).is_some() {
            bail!("xAI text-to-speech does not support --model.");
        }
        if request
            .speed
            .is_some_and(|speed| !(0.7..=1.5).contains(&speed))
        {
            bail!("xAI speed must be between 0.7 and 1.5.");
        }

        let voice_id = request
            .voice
            .as_deref()
            .and_then(non_empty)
            .or_else(|| non_empty(&self.config.default_voice_id))
            .unwrap_or("eve");
        let language = request
            .language_code
            .as_deref()
            .and_then(non_empty)
            .or_else(|| non_empty(&self.config.default_language))
            .unwrap_or("auto");
        let output_format = self.resolve_output_format(request)?;
        let payload = SpeechPayload {
            text: &request.text,
            voice_id,
            output_format,
            language,
            speed: request.speed,
        };
        let url = Url::parse(&format!(
            "{}/v1/tts",
            self.config.api_base_url.trim_end_matches('/')
        ))
        .context("Invalid XAI_API_BASE_URL.")?;

        let response = self
            .client
            .post(url)
            .bearer_auth(api_key)
            .header(header::ACCEPT, "application/octet-stream")
            .json(&payload)
            .send()
            .await
            .context("Could not send the xAI request.")?;
        let status = response.status();
        if !status.is_success() {
            let error = response.text().await.unwrap_or_default();
            bail!("Grok Voice request failed ({}): {error}", status.as_u16());
        }

        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split(';').next())
            .unwrap_or("application/octet-stream")
            .to_owned();
        let audio_bytes = response
            .bytes()
            .await
            .context("Could not read the xAI response body.")?
            .to_vec();

        Ok(SpeechResult {
            audio_bytes,
            file_extension: guess_file_extension(output_format.codec, &content_type).to_owned(),
            content_type,
        })
    }
}

fn non_empty(value: &str) -> Option<&str> {
    let value = value.trim();
    (!value.is_empty()).then_some(value)
}

fn parse_output_format_token(raw_value: &str) -> Result<Option<ParsedOutputFormat>> {
    let Some(raw_value) = non_empty(raw_value) else {
        return Ok(None);
    };
    let parts: Vec<_> = raw_value
        .split('_')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect();
    if parts.is_empty() || parts.len() > 3 {
        bail!(
            "Unsupported xAI output format '{raw_value}'. Expected codec, codec_sampleRate, or codec_sampleRate_bitRate."
        );
    }

    let codec = match parts[0].to_ascii_lowercase().as_str() {
        "mp3" => SpeechCodec::Mp3,
        "wav" => SpeechCodec::Wav,
        "pcm" => SpeechCodec::Pcm,
        "mulaw" | "ulaw" => SpeechCodec::Mulaw,
        "alaw" => SpeechCodec::Alaw,
        _ => bail!(
            "Unsupported xAI codec '{}' in output format '{raw_value}'. Allowed codecs: mp3, wav, pcm, mulaw, alaw.",
            parts[0]
        ),
    };
    let sample_rate = parts
        .get(1)
        .map(|value| parse_positive_u32(value, "sample rate", raw_value))
        .transpose()?;
    let bit_rate = parts
        .get(2)
        .map(|value| parse_positive_u32(value, "bit rate", raw_value))
        .transpose()?
        .map(|value| if value < 1_000 { value * 1_000 } else { value });
    if bit_rate.is_some() && codec != SpeechCodec::Mp3 {
        bail!("xAI codec '{}' does not support bit rate.", codec.as_str());
    }

    Ok(Some(ParsedOutputFormat {
        codec,
        sample_rate,
        bit_rate,
    }))
}

fn parse_positive_u32(raw_value: &str, label: &str, original_value: &str) -> Result<u32> {
    let value = raw_value
        .parse::<u32>()
        .with_context(|| format!("Invalid {label} in xAI output format '{original_value}'."))?;
    if value == 0 {
        bail!("Invalid {label} in xAI output format '{original_value}'.");
    }
    Ok(value)
}

fn guess_file_extension(codec: SpeechCodec, content_type: &str) -> &'static str {
    let content_type = content_type.to_ascii_lowercase();
    if codec == SpeechCodec::Mp3 || content_type.contains("mpeg") {
        ".mp3"
    } else if codec == SpeechCodec::Wav || content_type.contains("wav") {
        ".wav"
    } else if codec == SpeechCodec::Pcm {
        ".pcm"
    } else if codec == SpeechCodec::Mulaw || content_type.contains("basic") {
        ".mulaw"
    } else if codec == SpeechCodec::Alaw || content_type.contains("alaw") {
        ".alaw"
    } else {
        ".bin"
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, header, method, path},
    };

    use super::GrokProvider;
    use crate::{
        config::GrokConfig,
        providers::{SpeechCodec, SpeechOutputSettings, TextToSpeechProvider, TtsSpeechRequest},
    };

    fn config(api_base_url: String) -> GrokConfig {
        GrokConfig {
            api_key: "test-key".to_owned(),
            api_base_url,
            default_voice_id: "eve".to_owned(),
            default_output_format: "mp3".to_owned(),
            default_sample_rate: 24_000,
            default_bit_rate: Some(128_000),
            default_language: "auto".to_owned(),
        }
    }

    fn request() -> TtsSpeechRequest {
        TtsSpeechRequest {
            text: "hello".to_owned(),
            voice: None,
            model: None,
            output: None,
            speed: None,
            language_code: None,
        }
    }

    #[test]
    fn non_mp3_codecs_do_not_inherit_the_default_mp3_bit_rate() {
        let provider = GrokProvider::new(Client::new(), config(String::new()));
        let mut request = request();
        request.output = Some(SpeechOutputSettings {
            codec: Some(SpeechCodec::Wav),
            sample_rate: Some(48_000),
            bit_rate: None,
        });

        let output = provider.resolve_output_format(&request).unwrap();
        assert_eq!(output.codec, SpeechCodec::Wav);
        assert_eq!(output.sample_rate, 48_000);
        assert_eq!(output.bit_rate, None);
    }

    #[test]
    fn respects_low_configured_sample_rates() {
        let mut config = config(String::new());
        config.default_sample_rate = 8_000;
        let provider = GrokProvider::new(Client::new(), config);

        let output = provider.resolve_output_format(&request()).unwrap();

        assert_eq!(output.sample_rate, 8_000);
    }

    #[tokio::test]
    async fn supports_custom_voices_language_and_speed() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/tts"))
            .and(header("authorization", "Bearer test-key"))
            .and(body_json(serde_json::json!({
                "text": "hello",
                "voice_id": "custom01",
                "output_format": {
                    "codec": "mp3",
                    "sample_rate": 24000,
                    "bit_rate": 128_000
                },
                "language": "en",
                "speed": 1.2
            })))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "audio/mpeg")
                    .set_body_bytes([1, 2, 3]),
            )
            .mount(&server)
            .await;

        let provider = GrokProvider::new(Client::new(), config(server.uri()));
        let mut request = request();
        request.voice = Some("custom01".to_owned());
        request.language_code = Some("en".to_owned());
        request.speed = Some(1.2);
        let result = provider.speak(&request).await.unwrap();

        assert_eq!(result.audio_bytes, [1, 2, 3]);
        assert_eq!(result.file_extension, ".mp3");
    }
}
