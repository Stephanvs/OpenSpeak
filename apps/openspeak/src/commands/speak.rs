use anyhow::{Context, Result, bail};

use crate::{
    cli::{CliSpeechCodec, SpeakArgs},
    player::{Player, devices},
    providers::TextToSpeechProviderResolver,
};

/// Synthesize and immediately play one speech request.
///
/// # Errors
///
/// Returns an error when the text or provider is invalid, synthesis fails, or
/// the system player cannot play the generated audio.
pub async fn execute(
    provider_resolver: &TextToSpeechProviderResolver,
    player: &dyn Player,
    args: &SpeakArgs,
) -> Result<()> {
    if args.speech.text.trim().is_empty() {
        bail!("No text supplied for speak.");
    }
    if matches!(
        args.speech.codec,
        Some(CliSpeechCodec::Pcm | CliSpeechCodec::Mulaw | CliSpeechCodec::Alaw)
    ) {
        bail!("Immediate playback supports MP3 and WAV. Use `synthesize` for raw audio codecs.");
    }

    let selected_id = if let Some(index) = args.device {
        let listed = tokio::task::spawn_blocking(devices::list)
            .await
            .context("The audio device discovery task failed.")??;
        Some(
            listed
                .get(index)
                .with_context(|| {
                    format!("Output device {index} is unavailable. Run `devices list` again.")
                })?
                .id
                .clone(),
        )
    } else {
        None
    };

    let provider = provider_resolver.resolve(args.speech.provider.as_deref())?;
    let result = provider.speak(&args.speech.request()).await?;
    if !matches!(
        result.file_extension.to_ascii_lowercase().as_str(),
        ".mp3" | ".wav"
    ) {
        bail!(
            "Immediate playback supports MP3 and WAV; the provider returned {}.",
            result.content_type
        );
    }
    player.play(&result, selected_id.as_deref()).await?;

    println!(
        "[{}] played {} bytes ({}) on audio device {}",
        provider.name(),
        result.audio_bytes.len(),
        result.content_type,
        args.device
            .map_or_else(|| "default".to_owned(), |index| index.to_string())
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
        cli::{SpeakArgs, SpeechArgs},
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
        async fn play(&self, result: &SpeechResult, device_id: Option<&str>) -> Result<()> {
            assert_eq!(result.audio_bytes, [1, 2, 3]);
            assert_eq!(device_id, None);
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
        let args = SpeakArgs {
            speech: SpeechArgs {
                text: "hello".to_owned(),
                provider: None,
                voice: None,
                model: None,
                codec: None,
                speed: None,
                sample_rate: None,
                bit_rate: None,
                language: None,
            },
            device: None,
        };

        execute(&resolver, &player, &args).await.unwrap();

        assert!(player.0.load(Ordering::Relaxed));
    }
}
