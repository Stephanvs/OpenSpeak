mod elevenlabs;
mod grok;

use std::{collections::HashMap, sync::Arc};

use anyhow::{Result, bail};
use async_trait::async_trait;
use serde::Serialize;

pub use elevenlabs::ElevenLabsProvider;
pub use grok::GrokProvider;

#[derive(Clone, Debug)]
pub struct TtsSpeechRequest {
    pub text: String,
    pub voice: Option<String>,
    pub model: Option<String>,
    pub output: Option<SpeechOutputSettings>,
    pub speed: Option<f64>,
    pub language_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpeechOutputSettings {
    pub codec: Option<SpeechCodec>,
    pub sample_rate: Option<u32>,
    pub bit_rate: Option<u32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SpeechCodec {
    Mp3,
    Wav,
    Pcm,
    Mulaw,
    Alaw,
}

impl SpeechCodec {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mp3 => "mp3",
            Self::Wav => "wav",
            Self::Pcm => "pcm",
            Self::Mulaw => "mulaw",
            Self::Alaw => "alaw",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechResult {
    pub audio_bytes: Vec<u8>,
    pub content_type: String,
    pub file_extension: String,
}

#[async_trait]
pub trait TextToSpeechProvider: Send + Sync {
    fn name(&self) -> &'static str;

    async fn speak(&self, request: &TtsSpeechRequest) -> Result<SpeechResult>;
}

pub struct TextToSpeechProviderResolver {
    providers: HashMap<String, Arc<dyn TextToSpeechProvider>>,
    default_provider: String,
}

impl TextToSpeechProviderResolver {
    #[must_use]
    pub fn new(
        providers: impl IntoIterator<Item = Arc<dyn TextToSpeechProvider>>,
        default_provider: impl Into<String>,
    ) -> Self {
        let providers = providers
            .into_iter()
            .map(|provider| (provider.name().to_ascii_lowercase(), provider))
            .collect();

        Self {
            providers,
            default_provider: default_provider.into(),
        }
    }

    /// Resolve an explicit provider name or the configured default.
    ///
    /// # Errors
    ///
    /// Returns an error when the requested or default provider is not
    /// registered.
    pub fn resolve(&self, requested_name: Option<&str>) -> Result<&dyn TextToSpeechProvider> {
        let name = requested_name
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .unwrap_or(&self.default_provider)
            .to_ascii_lowercase();

        if let Some(provider) = self.providers.get(&name) {
            return Ok(provider.as_ref());
        }

        let mut names: Vec<_> = self.providers.keys().map(String::as_str).collect();
        names.sort_unstable();
        bail!(
            "Unknown TTS provider '{name}'. Available providers: {}.",
            names.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use async_trait::async_trait;

    use super::{
        SpeechResult, TextToSpeechProvider, TextToSpeechProviderResolver, TtsSpeechRequest,
    };

    struct TestProvider(&'static str);

    #[async_trait]
    impl TextToSpeechProvider for TestProvider {
        fn name(&self) -> &'static str {
            self.0
        }

        async fn speak(&self, _request: &TtsSpeechRequest) -> Result<SpeechResult> {
            unreachable!()
        }
    }

    #[test]
    fn resolver_is_case_insensitive_and_uses_the_default() {
        let resolver = TextToSpeechProviderResolver::new(
            [Arc::new(TestProvider("grok")) as Arc<dyn TextToSpeechProvider>],
            "GROK",
        );

        assert_eq!(resolver.resolve(None).unwrap().name(), "grok");
        assert_eq!(resolver.resolve(Some("GrOk")).unwrap().name(), "grok");
    }

    #[test]
    fn resolver_lists_available_providers_in_errors() {
        let resolver = TextToSpeechProviderResolver::new(
            [
                Arc::new(TestProvider("grok")) as Arc<dyn TextToSpeechProvider>,
                Arc::new(TestProvider("elevenlabs")) as Arc<dyn TextToSpeechProvider>,
            ],
            "grok",
        );

        let error = match resolver.resolve(Some("missing")) {
            Ok(_) => panic!("expected an unknown-provider error"),
            Err(error) => error.to_string(),
        };
        assert!(error.contains("elevenlabs, grok"));
    }
}
