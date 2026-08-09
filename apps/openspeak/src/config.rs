use std::env;

use anyhow::{Context, Result, bail};

#[derive(Clone, Debug)]
pub struct Config {
    pub default_provider: String,
    pub elevenlabs: ElevenLabsConfig,
    pub grok: GrokConfig,
}

#[derive(Clone, Debug)]
pub struct ElevenLabsConfig {
    pub api_key: String,
    pub api_base_url: String,
    pub default_voice_id: String,
    pub default_model_id: String,
    pub default_output_format: String,
}

#[derive(Clone, Debug)]
pub struct GrokConfig {
    pub api_key: String,
    pub api_base_url: String,
    pub default_voice_id: String,
    pub default_output_format: String,
    pub default_sample_rate: u32,
    pub default_bit_rate: Option<u32>,
    pub default_language: String,
}

impl Config {
    /// Load `OpenSpeak` and provider settings from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error when a configured numeric value is not a positive
    /// integer.
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            default_provider: string_or("OPENSPEAK_DEFAULT_PROVIDER", "grok"),
            elevenlabs: ElevenLabsConfig {
                api_key: optional_string("ELEVENLABS_API_KEY").unwrap_or_default(),
                api_base_url: string_or("ELEVENLABS_API_BASE_URL", "https://api.elevenlabs.io"),
                default_voice_id: optional_string("ELEVENLABS_DEFAULT_VOICE_ID")
                    .unwrap_or_default(),
                default_model_id: string_or(
                    "ELEVENLABS_DEFAULT_MODEL_ID",
                    "eleven_multilingual_v2",
                ),
                default_output_format: string_or(
                    "ELEVENLABS_DEFAULT_OUTPUT_FORMAT",
                    "mp3_44100_128",
                ),
            },
            grok: GrokConfig {
                api_key: optional_string("XAI_API_KEY").unwrap_or_default(),
                api_base_url: string_or("XAI_API_BASE_URL", "https://api.x.ai"),
                default_voice_id: string_or("XAI_DEFAULT_VOICE_ID", "eve"),
                default_output_format: string_or("XAI_DEFAULT_OUTPUT_FORMAT", "mp3"),
                default_sample_rate: positive_u32_or("XAI_DEFAULT_SAMPLE_RATE", 24_000)?,
                default_bit_rate: optional_positive_u32("XAI_DEFAULT_BIT_RATE")?.or(Some(128_000)),
                default_language: string_or("XAI_DEFAULT_LANGUAGE", "auto"),
            },
        })
    }
}

fn optional_string(name: &str) -> Option<String> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn string_or(name: &str, default: &str) -> String {
    optional_string(name).unwrap_or_else(|| default.to_owned())
}

fn positive_u32_or(name: &str, default: u32) -> Result<u32> {
    optional_positive_u32(name).map(|value| value.unwrap_or(default))
}

fn optional_positive_u32(name: &str) -> Result<Option<u32>> {
    let Some(raw_value) = optional_string(name) else {
        return Ok(None);
    };

    let value = raw_value
        .parse::<u32>()
        .with_context(|| format!("Invalid positive integer in {name}."))?;
    if value == 0 {
        bail!("Invalid positive integer in {name}.");
    }

    Ok(Some(value))
}
