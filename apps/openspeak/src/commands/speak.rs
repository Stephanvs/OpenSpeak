use anyhow::{Result, bail};

use crate::{cli::SpeechArgs, player::Player, providers::TextToSpeechProviderResolver};

/// Synthesize and immediately play one speech request.
///
/// # Errors
///
/// Returns an error when the text or provider is invalid, synthesis fails, or
/// the system player cannot play the generated audio.
pub async fn execute(
    provider_resolver: &TextToSpeechProviderResolver,
    player: &dyn Player,
    args: &SpeechArgs,
) -> Result<()> {
    if args.text.trim().is_empty() {
        bail!("No text supplied for speak.");
    }

    let provider = provider_resolver.resolve(args.provider.as_deref())?;
    let result = provider.speak(&args.request()).await?;
    player.play(&result).await?;

    println!(
        "[{}] played {} bytes ({}) on the default audio device",
        provider.name(),
        result.audio_bytes.len(),
        result.content_type
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    use anyhow::Result;
    use async_trait::async_trait;

    use super::execute;
    use crate::{
        cli::SpeechArgs,
        player::Player,
        providers::{
            SpeechResult, TextToSpeechProvider, TextToSpeechProviderResolver, TtsSpeechRequest,
        },
    };

    struct Provider;

    #[async_trait]
    impl TextToSpeechProvider for Provider {
        fn name(&self) -> &'static str {
            "test"
        }

        async fn speak(&self, _request: &TtsSpeechRequest) -> Result<SpeechResult> {
            Ok(SpeechResult {
                audio_bytes: vec![1, 2, 3],
                content_type: "audio/mpeg".to_owned(),
                file_extension: ".mp3".to_owned(),
            })
        }
    }

    struct RecordingPlayer(AtomicBool);

    #[async_trait]
    impl Player for RecordingPlayer {
        async fn play(&self, result: &SpeechResult) -> Result<()> {
            assert_eq!(result.audio_bytes, [1, 2, 3]);
            self.0.store(true, Ordering::Relaxed);
            Ok(())
        }
    }

    #[tokio::test]
    async fn synthesizes_then_plays_audio() {
        let resolver = TextToSpeechProviderResolver::new(
            [Arc::new(Provider) as Arc<dyn TextToSpeechProvider>],
            "test",
        );
        let player = RecordingPlayer(AtomicBool::new(false));
        let args = SpeechArgs {
            text: "hello".to_owned(),
            provider: None,
            voice: None,
            model: None,
            codec: None,
            speed: None,
            sample_rate: None,
            bit_rate: None,
            language: None,
        };

        execute(&resolver, &player, &args).await.unwrap();

        assert!(player.0.load(Ordering::Relaxed));
    }
}
