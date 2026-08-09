use anyhow::{Context, Result, anyhow, bail};
use async_trait::async_trait;
use reqwest::{Client, Url, header};
use serde::Serialize;

use crate::{
    config::ElevenLabsConfig,
    providers::{SpeechCodec, SpeechResult, TextToSpeechProvider, TtsSpeechRequest},
};

const DEFAULT_MP3_SAMPLE_RATE: u32 = 44_100;
const DEFAULT_MP3_BIT_RATE: u32 = 128_000;
const DEFAULT_SAMPLE_RATE: u32 = 24_000;
const TELEPHONY_SAMPLE_RATE: u32 = 8_000;

pub struct ElevenLabsProvider {
    client: Client,
    config: ElevenLabsConfig,
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
    model_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice_settings: Option<VoiceSettings>,
}

#[derive(Debug, Serialize)]
struct VoiceSettings {
    speed: f64,
}

impl ElevenLabsProvider {
    #[must_use]
    pub fn new(client: Client, config: ElevenLabsConfig) -> Self {
        Self { client, config }
    }

    fn resolve_output_format(&self, request: &TtsSpeechRequest) -> Result<String> {
        let Some(requested) = request.output else {
            return Ok(non_empty(&self.config.default_output_format)
                .unwrap_or("mp3_44100_128")
                .to_owned());
        };

        let configured = parse_output_format_token(&self.config.default_output_format);
        let codec = requested
            .codec
            .or(configured.map(|format| format.codec))
            .unwrap_or(SpeechCodec::Mp3);
        let sample_rate = requested.sample_rate.or_else(|| {
            configured
                .filter(|format| format.codec == codec)
                .and_then(|format| format.sample_rate)
        });
        let bit_rate = requested.bit_rate.or_else(|| {
            configured
                .filter(|format| format.codec == codec && requested.sample_rate.is_none())
                .and_then(|format| format.bit_rate)
        });

        match codec {
            SpeechCodec::Mp3 => resolve_mp3_format(
                sample_rate.unwrap_or(DEFAULT_MP3_SAMPLE_RATE),
                bit_rate.unwrap_or(DEFAULT_MP3_BIT_RATE),
            )
            .map(str::to_owned),
            SpeechCodec::Wav => {
                resolve_linear_format("wav", sample_rate.unwrap_or(DEFAULT_SAMPLE_RATE), bit_rate)
            }
            SpeechCodec::Pcm => {
                resolve_linear_format("pcm", sample_rate.unwrap_or(DEFAULT_SAMPLE_RATE), bit_rate)
            }
            SpeechCodec::Mulaw => resolve_telephony_format("ulaw_8000", sample_rate, bit_rate),
            SpeechCodec::Alaw => resolve_telephony_format("alaw_8000", sample_rate, bit_rate),
        }
    }
}

#[async_trait]
impl TextToSpeechProvider for ElevenLabsProvider {
    fn name(&self) -> &'static str {
        "elevenlabs"
    }

    async fn speak(&self, request: &TtsSpeechRequest) -> Result<SpeechResult> {
        let api_key = non_empty(&self.config.api_key)
            .ok_or_else(|| anyhow!("Missing ELEVENLABS_API_KEY environment variable."))?;
        let voice_id = request
            .voice
            .as_deref()
            .and_then(non_empty)
            .or_else(|| non_empty(&self.config.default_voice_id))
            .ok_or_else(|| {
                anyhow!(
                    "No ElevenLabs voice configured. Set ELEVENLABS_DEFAULT_VOICE_ID or pass --voice."
                )
            })?;

        if request
            .speed
            .is_some_and(|speed| !(0.7..=1.2).contains(&speed))
        {
            bail!("ElevenLabs speed must be between 0.7 and 1.2.");
        }

        let output_format = self.resolve_output_format(request)?;
        let model_id = request
            .model
            .as_deref()
            .and_then(non_empty)
            .or_else(|| non_empty(&self.config.default_model_id))
            .unwrap_or("eleven_multilingual_v2");
        let language_code = request.language_code.as_deref().and_then(non_empty);
        let payload = SpeechPayload {
            text: &request.text,
            model_id,
            language_code,
            voice_settings: request.speed.map(|speed| VoiceSettings { speed }),
        };

        let mut url = Url::parse(&format!(
            "{}/v1/text-to-speech",
            self.config.api_base_url.trim_end_matches('/')
        ))
        .context("Invalid ELEVENLABS_API_BASE_URL.")?;
        url.path_segments_mut()
            .map_err(|()| anyhow!("ELEVENLABS_API_BASE_URL cannot be used as an API base URL."))?
            .push(voice_id);
        url.query_pairs_mut()
            .append_pair("output_format", &output_format);

        let response = self
            .client
            .post(url)
            .header("xi-api-key", api_key)
            .header(header::ACCEPT, "application/octet-stream")
            .json(&payload)
            .send()
            .await
            .context("Could not send the ElevenLabs request.")?;
        let status = response.status();
        if !status.is_success() {
            let error = response.text().await.unwrap_or_default();
            bail!("ElevenLabs request failed ({}): {error}", status.as_u16());
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
            .context("Could not read the ElevenLabs response body.")?
            .to_vec();

        Ok(SpeechResult {
            audio_bytes,
            file_extension: guess_file_extension(&output_format, &content_type).to_owned(),
            content_type,
        })
    }
}

fn non_empty(value: &str) -> Option<&str> {
    let value = value.trim();
    (!value.is_empty()).then_some(value)
}

fn parse_output_format_token(raw_value: &str) -> Option<ParsedOutputFormat> {
    let parts: Vec<_> = raw_value
        .split('_')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect();
    if parts.is_empty() || parts.len() > 4 {
        return None;
    }

    let codec = match parts[0].to_ascii_lowercase().as_str() {
        "mp3" => SpeechCodec::Mp3,
        "wav" => SpeechCodec::Wav,
        "pcm" => SpeechCodec::Pcm,
        "mulaw" | "ulaw" => SpeechCodec::Mulaw,
        "alaw" => SpeechCodec::Alaw,
        _ => return None,
    };

    if matches!(codec, SpeechCodec::Mulaw | SpeechCodec::Alaw)
        && parts.len() == 2
        && parts[1] == "8000"
    {
        return Some(ParsedOutputFormat {
            codec,
            sample_rate: Some(TELEPHONY_SAMPLE_RATE),
            bit_rate: None,
        });
    }

    if !matches!(
        codec,
        SpeechCodec::Mp3 | SpeechCodec::Pcm | SpeechCodec::Wav
    ) {
        return None;
    }

    let sample_rate = parts.get(1).and_then(|value| value.parse().ok());
    let bit_rate = parts
        .get(2)
        .and_then(|value| value.parse::<u32>().ok())
        .map(|value| if value < 1_000 { value * 1_000 } else { value });
    Some(ParsedOutputFormat {
        codec,
        sample_rate,
        bit_rate,
    })
}

fn resolve_mp3_format(sample_rate: u32, bit_rate: u32) -> Result<&'static str> {
    match (sample_rate, bit_rate) {
        (22_050, 32_000) => Ok("mp3_22050_32"),
        (24_000, 48_000) => Ok("mp3_24000_48"),
        (44_100, 32_000) => Ok("mp3_44100_32"),
        (44_100, 64_000) => Ok("mp3_44100_64"),
        (44_100, 96_000) => Ok("mp3_44100_96"),
        (44_100, 128_000) => Ok("mp3_44100_128"),
        (44_100, 192_000) => Ok("mp3_44100_192"),
        _ => bail!("Unsupported ElevenLabs MP3 format {sample_rate} Hz / {bit_rate} bps."),
    }
}

fn resolve_linear_format(codec: &str, sample_rate: u32, bit_rate: Option<u32>) -> Result<String> {
    if bit_rate.is_some() {
        bail!("ElevenLabs codec '{codec}' does not support bit rate.");
    }

    match sample_rate {
        8_000 | 16_000 | 22_050 | 24_000 | 32_000 | 44_100 | 48_000 => {
            Ok(format!("{codec}_{sample_rate}"))
        }
        _ => bail!("Unsupported ElevenLabs {codec} sample rate '{sample_rate}'."),
    }
}

fn resolve_telephony_format(
    format: &'static str,
    sample_rate: Option<u32>,
    bit_rate: Option<u32>,
) -> Result<String> {
    let codec = format.split_once('_').map_or(format, |(codec, _)| codec);
    if bit_rate.is_some() {
        bail!("ElevenLabs codec '{codec}' does not support bit rate.");
    }
    if sample_rate.unwrap_or(TELEPHONY_SAMPLE_RATE) != TELEPHONY_SAMPLE_RATE {
        bail!("ElevenLabs telephony codecs require sample rate {TELEPHONY_SAMPLE_RATE}.");
    }

    Ok(format.to_owned())
}

fn guess_file_extension(output_format: &str, content_type: &str) -> &'static str {
    let output_format = output_format.to_ascii_lowercase();
    let content_type = content_type.to_ascii_lowercase();
    if output_format.starts_with("mp3") || content_type.contains("mpeg") {
        ".mp3"
    } else if output_format.starts_with("wav") || content_type.contains("wav") {
        ".wav"
    } else if output_format.starts_with("pcm") {
        ".pcm"
    } else if output_format.starts_with("ulaw")
        || output_format.starts_with("mulaw")
        || content_type.contains("basic")
    {
        ".mulaw"
    } else if output_format.starts_with("alaw") || content_type.contains("alaw") {
        ".alaw"
    } else if output_format.starts_with("opus") || content_type.contains("opus") {
        ".opus"
    } else {
        ".bin"
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, header, method, path, query_param},
    };

    use super::ElevenLabsProvider;
    use crate::{
        config::ElevenLabsConfig,
        providers::{SpeechCodec, SpeechOutputSettings, TextToSpeechProvider, TtsSpeechRequest},
    };

    fn config(api_base_url: String) -> ElevenLabsConfig {
        ElevenLabsConfig {
            api_key: "test-key".to_owned(),
            api_base_url,
            default_voice_id: "voice-id".to_owned(),
            default_model_id: "eleven_multilingual_v2".to_owned(),
            default_output_format: "mp3_44100_128".to_owned(),
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
    fn maps_generic_output_settings_to_elevenlabs_tokens() {
        let provider = ElevenLabsProvider::new(Client::new(), config(String::new()));
        let mut request = request();
        request.output = Some(SpeechOutputSettings {
            codec: Some(SpeechCodec::Mp3),
            sample_rate: Some(44_100),
            bit_rate: Some(192_000),
        });

        assert_eq!(
            provider.resolve_output_format(&request).unwrap(),
            "mp3_44100_192"
        );
    }

    #[tokio::test]
    async fn sends_the_expected_request_and_returns_audio() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/text-to-speech/voice-id"))
            .and(query_param("output_format", "mp3_44100_128"))
            .and(header("xi-api-key", "test-key"))
            .and(body_json(serde_json::json!({
                "text": "hello",
                "model_id": "eleven_multilingual_v2"
            })))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "audio/mpeg")
                    .set_body_bytes([1, 2, 3]),
            )
            .mount(&server)
            .await;

        let provider = ElevenLabsProvider::new(Client::new(), config(server.uri()));
        let result = provider.speak(&request()).await.unwrap();

        assert_eq!(result.audio_bytes, [1, 2, 3]);
        assert_eq!(result.file_extension, ".mp3");
    }
}
