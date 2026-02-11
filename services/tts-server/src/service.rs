use std::{collections::HashMap, sync::Arc};

use reqwest::Client;
use serde_json::json;
use tiktoken_rs::cl100k_base;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::model::{HealthResponse, SynthesizeRequest, SynthesizeResponse, VoiceModel};

const OPENAI_ENDPOINT_DEFAULT: &str = "https://api.openai.com/v1/audio/speech";
const ELEVENLABS_ENDPOINT_DEFAULT: &str = "https://api.elevenlabs.io/v1/text-to-speech";
const OPENAI_DEFAULT_MODEL: &str = "tts-1-hd";
const ELEVENLABS_DEFAULT_MODEL: &str = "eleven_turbo_v2_5";
const OPENAI_SAMPLE_RATE_HZ: u32 = 24_000;
const ELEVENLABS_SAMPLE_RATE_HZ: u32 = 24_000;
const OPENAI_MAX_CHARS: usize = 4_096;
const ELEVENLABS_MAX_CHARS: usize = 5_000;
const MAX_TOKENS_PER_CHUNK: usize = 500;
const ELEVENLABS_OUTPUT_FORMAT: &str = "pcm_24000";

#[derive(Clone)]
pub struct AppState {
    pub models: Arc<Vec<VoiceModel>>,
    pub audio_store: Arc<RwLock<HashMap<String, StoredAudio>>>,
    http_client: Client,
    hosted_models: Arc<HashMap<String, HostedModel>>,
}

#[derive(Clone)]
pub struct StoredAudio {
    pub data: Vec<u8>,
    pub content_type: String,
}

#[derive(Clone)]
struct HostedModel {
    public_model: VoiceModel,
    provider: ProviderConfig,
}

#[derive(Clone)]
enum ProviderConfig {
    OpenAi(OpenAiConfig),
    ElevenLabs(ElevenLabsConfig),
}

#[derive(Clone)]
struct OpenAiConfig {
    api_key: String,
    endpoint: String,
    model: String,
    voice: String,
    sample_rate_hz: u32,
}

#[derive(Clone)]
struct ElevenLabsConfig {
    api_key: String,
    endpoint: String,
    voice_id: String,
    model: String,
    stability: f32,
    similarity_boost: f32,
    output_format: String,
    sample_rate_hz: u32,
}

#[derive(Debug)]
pub enum SynthesizeError {
    InvalidRequest(String),
    UnknownModel(String),
    MissingConfiguration(String),
    Upstream(String),
    Internal(String),
}

impl std::fmt::Display for SynthesizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest(message)
            | Self::UnknownModel(message)
            | Self::MissingConfiguration(message)
            | Self::Upstream(message)
            | Self::Internal(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for SynthesizeError {}

impl AppState {
    pub fn new() -> Self {
        let hosted_models_vec = build_hosted_models();
        let models: Vec<VoiceModel> = hosted_models_vec
            .iter()
            .map(|model| model.public_model.clone())
            .collect();
        let hosted_models = hosted_models_vec
            .into_iter()
            .map(|model| (model.public_model.id.clone(), model))
            .collect();

        Self {
            models: Arc::new(models),
            hosted_models: Arc::new(hosted_models),
            audio_store: Arc::new(RwLock::new(HashMap::new())),
            http_client: Client::new(),
        }
    }

    pub fn health(&self) -> HealthResponse {
        HealthResponse {
            status: "ok".into(),
            models_loaded: self.models.len(),
        }
    }

    pub async fn synthesize(
        &self,
        req: SynthesizeRequest,
    ) -> Result<SynthesizeResponse, SynthesizeError> {
        if req.text.trim().is_empty() {
            return Err(SynthesizeError::InvalidRequest(
                "text cannot be empty".into(),
            ));
        }

        if self.hosted_models.is_empty() {
            return Err(SynthesizeError::MissingConfiguration(
                "No models configured. Set OPENAI_API_KEY and/or ELEVENLABS_API_KEY (+ ELEVENLABS_VOICE_ID)."
                    .into(),
            ));
        }

        if req.format != "wav" && req.format != "pcm_s16le" {
            return Err(SynthesizeError::InvalidRequest(
                "format must be either 'wav' or 'pcm_s16le'".into(),
            ));
        }

        let hosted_model = self
            .hosted_models
            .get(&req.model_id)
            .cloned()
            .ok_or_else(|| {
                SynthesizeError::UnknownModel(format!("Unknown model_id: {}", req.model_id))
            })?;

        let chunks = chunk_text(
            &req.text,
            MAX_TOKENS_PER_CHUNK,
            hosted_model.provider.max_chars(),
        )?;
        let mut pcm = Vec::new();
        for chunk in chunks {
            let bytes = synthesize_chunk(&self.http_client, &hosted_model.provider, &chunk).await?;
            pcm.extend_from_slice(&bytes);
        }

        if pcm.is_empty() {
            return Err(SynthesizeError::Upstream(
                "Provider returned no audio data".into(),
            ));
        }

        let sample_rate_hz = hosted_model.provider.sample_rate_hz();
        let format = req.format;
        let (audio, content_type) = if format == "wav" {
            (
                wav_from_pcm_mono_i16(sample_rate_hz, &pcm),
                "audio/wav".to_string(),
            )
        } else {
            (pcm, format!("audio/L16; rate={sample_rate_hz}; channels=1"))
        };

        let request_id = Uuid::new_v4().to_string();
        self.audio_store.write().await.insert(
            request_id.clone(),
            StoredAudio {
                data: audio,
                content_type,
            },
        );

        Ok(SynthesizeResponse {
            request_id: request_id.clone(),
            stream_url: format!("/v1/audio/{request_id}"),
            format,
            sample_rate_hz,
        })
    }
}

impl ProviderConfig {
    fn max_chars(&self) -> usize {
        match self {
            Self::OpenAi(_) => OPENAI_MAX_CHARS,
            Self::ElevenLabs(_) => ELEVENLABS_MAX_CHARS,
        }
    }

    fn sample_rate_hz(&self) -> u32 {
        match self {
            Self::OpenAi(config) => config.sample_rate_hz,
            Self::ElevenLabs(config) => config.sample_rate_hz,
        }
    }
}

fn build_hosted_models() -> Vec<HostedModel> {
    let mut models = Vec::new();

    if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
        let endpoint = std::env::var("OPENAI_TTS_ENDPOINT")
            .unwrap_or_else(|_| OPENAI_ENDPOINT_DEFAULT.to_string());
        let model =
            std::env::var("OPENAI_TTS_MODEL").unwrap_or_else(|_| OPENAI_DEFAULT_MODEL.to_string());
        for (voice_id, display_name) in openai_voices() {
            models.push(HostedModel {
                public_model: VoiceModel {
                    id: format!("openai-{model}-{voice_id}"),
                    name: format!("OpenAI {display_name}"),
                    language: "en-US".into(),
                    backend: "openai".into(),
                    sample_rate_hz: OPENAI_SAMPLE_RATE_HZ,
                },
                provider: ProviderConfig::OpenAi(OpenAiConfig {
                    api_key: api_key.clone(),
                    endpoint: endpoint.clone(),
                    model: model.clone(),
                    voice: voice_id.to_string(),
                    sample_rate_hz: OPENAI_SAMPLE_RATE_HZ,
                }),
            });
        }
    }

    let elevenlabs_api_key = std::env::var("ELEVENLABS_API_KEY").ok();
    let elevenlabs_voice_id = std::env::var("ELEVENLABS_VOICE_ID").ok();
    if let (Some(api_key), Some(voice_id)) = (elevenlabs_api_key, elevenlabs_voice_id) {
        let endpoint = std::env::var("ELEVENLABS_TTS_ENDPOINT")
            .unwrap_or_else(|_| ELEVENLABS_ENDPOINT_DEFAULT.to_string());
        let model = std::env::var("ELEVENLABS_MODEL_ID")
            .unwrap_or_else(|_| ELEVENLABS_DEFAULT_MODEL.to_string());
        let voice_name = std::env::var("ELEVENLABS_VOICE_NAME")
            .unwrap_or_else(|_| "Configured Voice".to_string());
        let stability = std::env::var("ELEVENLABS_STABILITY")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(0.5)
            .clamp(0.0, 1.0);
        let similarity_boost = std::env::var("ELEVENLABS_SIMILARITY")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(0.75)
            .clamp(0.0, 1.0);

        models.push(HostedModel {
            public_model: VoiceModel {
                id: format!("elevenlabs-{voice_id}"),
                name: format!("ElevenLabs {voice_name}"),
                language: "en-US".into(),
                backend: "elevenlabs".into(),
                sample_rate_hz: ELEVENLABS_SAMPLE_RATE_HZ,
            },
            provider: ProviderConfig::ElevenLabs(ElevenLabsConfig {
                api_key,
                endpoint,
                voice_id,
                model,
                stability,
                similarity_boost,
                output_format: ELEVENLABS_OUTPUT_FORMAT.to_string(),
                sample_rate_hz: ELEVENLABS_SAMPLE_RATE_HZ,
            }),
        });
    }

    models
}

async fn synthesize_chunk(
    client: &Client,
    provider: &ProviderConfig,
    text: &str,
) -> Result<Vec<u8>, SynthesizeError> {
    match provider {
        ProviderConfig::OpenAi(config) => synthesize_openai_chunk(client, config, text).await,
        ProviderConfig::ElevenLabs(config) => {
            synthesize_elevenlabs_chunk(client, config, text).await
        }
    }
}

async fn synthesize_openai_chunk(
    client: &Client,
    config: &OpenAiConfig,
    text: &str,
) -> Result<Vec<u8>, SynthesizeError> {
    let response = client
        .post(&config.endpoint)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": config.model,
            "voice": config.voice,
            "input": text,
            "response_format": "pcm"
        }))
        .send()
        .await
        .map_err(|error| SynthesizeError::Upstream(format!("OpenAI request failed: {error}")))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "<unable to read body>".to_string());
        return Err(SynthesizeError::Upstream(format!(
            "OpenAI request failed ({status}): {}",
            shorten_for_error(&body, 280)
        )));
    }

    response
        .bytes()
        .await
        .map(|bytes| bytes.to_vec())
        .map_err(|error| SynthesizeError::Upstream(format!("OpenAI audio read failed: {error}")))
}

async fn synthesize_elevenlabs_chunk(
    client: &Client,
    config: &ElevenLabsConfig,
    text: &str,
) -> Result<Vec<u8>, SynthesizeError> {
    let endpoint = elevenlabs_endpoint_with_voice_id(&config.endpoint, &config.voice_id);
    let response = client
        .post(endpoint)
        .header("xi-api-key", &config.api_key)
        .header("Content-Type", "application/json")
        .query(&[("output_format", config.output_format.as_str())])
        .json(&json!({
            "text": text,
            "model_id": config.model,
            "voice_settings": {
                "stability": config.stability,
                "similarity_boost": config.similarity_boost,
            }
        }))
        .send()
        .await
        .map_err(|error| {
            SynthesizeError::Upstream(format!("ElevenLabs request failed: {error}"))
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "<unable to read body>".to_string());
        return Err(SynthesizeError::Upstream(format!(
            "ElevenLabs request failed ({status}): {}",
            shorten_for_error(&body, 280)
        )));
    }

    response
        .bytes()
        .await
        .map(|bytes| bytes.to_vec())
        .map_err(|error| {
            SynthesizeError::Upstream(format!("ElevenLabs audio read failed: {error}"))
        })
}

fn chunk_text(
    text: &str,
    max_tokens_per_chunk: usize,
    max_chars_per_chunk: usize,
) -> Result<Vec<String>, SynthesizeError> {
    let tokenizer = cl100k_base().map_err(|error| {
        SynthesizeError::Internal(format!("Failed to initialize tokenizer: {error}"))
    })?;

    let mut chunks = Vec::new();
    let mut current_chunk = String::new();

    for word in text.split_whitespace() {
        let candidate = if current_chunk.is_empty() {
            word.to_string()
        } else {
            format!("{current_chunk} {word}")
        };

        let candidate_tokens = tokenizer.encode_ordinary(&candidate).len();
        let candidate_chars = candidate.chars().count();

        if candidate_tokens <= max_tokens_per_chunk && candidate_chars <= max_chars_per_chunk {
            current_chunk = candidate;
            continue;
        }

        if current_chunk.is_empty() {
            return Err(SynthesizeError::InvalidRequest(format!(
                "Input contains a segment that exceeds provider limits (max {max_tokens_per_chunk} tokens, {max_chars_per_chunk} chars per chunk)"
            )));
        }

        chunks.push(std::mem::take(&mut current_chunk));
        current_chunk = word.to_string();

        let current_tokens = tokenizer.encode_ordinary(&current_chunk).len();
        let current_chars = current_chunk.chars().count();
        if current_tokens > max_tokens_per_chunk || current_chars > max_chars_per_chunk {
            return Err(SynthesizeError::InvalidRequest(format!(
                "Input contains a segment that exceeds provider limits (max {max_tokens_per_chunk} tokens, {max_chars_per_chunk} chars per chunk)"
            )));
        }
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    if chunks.is_empty() {
        return Err(SynthesizeError::InvalidRequest(
            "text cannot be empty".into(),
        ));
    }

    Ok(chunks)
}

fn openai_voices() -> [(&'static str, &'static str); 9] {
    [
        ("alloy", "Alloy"),
        ("echo", "Echo"),
        ("fable", "Fable"),
        ("onyx", "Onyx"),
        ("nova", "Nova"),
        ("shimmer", "Shimmer"),
        ("ash", "Ash"),
        ("coral", "Coral"),
        ("sage", "Sage"),
    ]
}

fn elevenlabs_endpoint_with_voice_id(endpoint: &str, voice_id: &str) -> String {
    if endpoint.contains("{voice_id}") {
        endpoint.replace("{voice_id}", voice_id)
    } else {
        format!("{}/{}", endpoint.trim_end_matches('/'), voice_id)
    }
}

fn shorten_for_error(value: &str, max_chars: usize) -> String {
    let shortened: String = value.chars().take(max_chars).collect();
    if value.chars().count() > max_chars {
        format!("{shortened}...")
    } else {
        shortened
    }
}

fn wav_from_pcm_mono_i16(sample_rate: u32, pcm: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(44 + pcm.len());
    let byte_rate = sample_rate * 2;
    let block_align = 2u16;
    let subchunk2_size = pcm.len() as u32;
    let chunk_size = 36 + subchunk2_size;

    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&chunk_size.to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&block_align.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&subchunk2_size.to_le_bytes());
    out.extend_from_slice(pcm);
    out
}

#[cfg(test)]
mod tests {
    use super::{
        chunk_text, elevenlabs_endpoint_with_voice_id, shorten_for_error, wav_from_pcm_mono_i16,
    };

    #[test]
    fn chunk_text_splits_long_input() {
        let text = "hello ".repeat(600);
        let chunks = chunk_text(&text, 200, 1_000).expect("chunking should succeed");
        assert!(chunks.len() > 1);
    }

    #[test]
    fn chunk_text_rejects_empty_input() {
        let error = chunk_text("   ", 200, 1_000).expect_err("empty input should fail");
        assert_eq!(error.to_string(), "text cannot be empty");
    }

    #[test]
    fn chunk_text_rejects_oversized_segment() {
        let oversized_word = "x".repeat(1_001);
        let error = chunk_text(&oversized_word, 200, 1_000)
            .expect_err("oversized segment should fail chunking");
        assert!(error
            .to_string()
            .contains("Input contains a segment that exceeds provider limits"));
    }

    #[test]
    fn elevenlabs_endpoint_supports_placeholder() {
        let endpoint = elevenlabs_endpoint_with_voice_id(
            "https://example.test/v1/text-to-speech/{voice_id}",
            "voice123",
        );
        assert_eq!(endpoint, "https://example.test/v1/text-to-speech/voice123");
    }

    #[test]
    fn elevenlabs_endpoint_appends_voice_without_placeholder() {
        let endpoint =
            elevenlabs_endpoint_with_voice_id("https://example.test/v1/text-to-speech", "voice123");
        assert_eq!(endpoint, "https://example.test/v1/text-to-speech/voice123");
    }

    #[test]
    fn shorten_for_error_adds_ellipsis_when_truncated() {
        let value = "abcdefghijklmnopqrstuvwxyz";
        let shortened = shorten_for_error(value, 10);
        assert_eq!(shortened, "abcdefghij...");
    }

    #[test]
    fn wav_builder_emits_riff_header() {
        let wav = wav_from_pcm_mono_i16(24_000, &[0, 0, 1, 0]);
        assert!(wav.starts_with(b"RIFF"));
        assert_eq!(&wav[8..12], b"WAVE");
    }
}
