use std::{env, path::PathBuf};

use anyhow::{Context, Result, bail};
use chrono::Utc;
use uuid::Uuid;

use crate::{cli::SynthesizeArgs, providers::TextToSpeechProviderResolver};

/// Synthesize one speech request and save it to an audio file.
///
/// # Errors
///
/// Returns an error when the request is invalid, synthesis fails, the output
/// directory cannot be resolved, or the generated file cannot be written.
pub async fn execute(
    provider_resolver: &TextToSpeechProviderResolver,
    args: &SynthesizeArgs,
) -> Result<()> {
    if args.speech.text.trim().is_empty() {
        bail!("No text supplied for synthesize.");
    }

    let provider = provider_resolver.resolve(args.speech.provider.as_deref())?;
    let result = provider.speak(&args.speech.request()).await?;
    let output_path = resolve_output_path(args.output.as_ref(), &result.file_extension)?;
    tokio::fs::write(&output_path, &result.audio_bytes)
        .await
        .with_context(|| format!("Could not write audio to {}.", output_path.display()))?;

    println!(
        "[{}] wrote {} bytes to {}",
        provider.name(),
        result.audio_bytes.len(),
        output_path.display()
    );
    Ok(())
}

fn resolve_output_path(
    configured_directory: Option<&PathBuf>,
    file_extension: &str,
) -> Result<PathBuf> {
    let current_directory =
        env::current_dir().context("Could not determine the current directory.")?;
    let output_directory = match configured_directory {
        Some(directory) if directory.is_absolute() => directory.clone(),
        Some(directory) => current_directory.join(directory),
        None => current_directory,
    };
    let extension = if file_extension.starts_with('.') {
        file_extension.to_owned()
    } else {
        format!(".{file_extension}")
    };
    let file_name = format!(
        "openspeak-{}-{}{}",
        Utc::now().format("%Y%m%d%H%M%S"),
        Uuid::new_v4().simple(),
        extension
    );

    Ok(output_directory.join(file_name))
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use anyhow::Result;
    use async_trait::async_trait;
    use tempfile::tempdir;

    use super::{execute, resolve_output_path};
    use crate::{
        cli::{SpeechArgs, SynthesizeArgs},
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
                content_type: "audio/wav".to_owned(),
                file_extension: ".wav".to_owned(),
            })
        }
    }

    #[test]
    fn output_path_uses_the_requested_directory_and_extension() {
        let directory = PathBuf::from("audio");
        let path = resolve_output_path(Some(&directory), ".wav").unwrap();

        assert!(path.parent().unwrap().ends_with("audio"));
        assert_eq!(path.extension().unwrap(), "wav");
        assert!(
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("openspeak-")
        );
    }

    #[tokio::test]
    async fn writes_synthesized_audio_to_the_output_directory() {
        let resolver = TextToSpeechProviderResolver::new(
            [Arc::new(Provider) as Arc<dyn TextToSpeechProvider>],
            "test",
        );
        let output_directory = tempdir().unwrap();
        let args = SynthesizeArgs {
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
            output: Some(output_directory.path().to_owned()),
        };

        execute(&resolver, &args).await.unwrap();

        let files: Vec<_> = std::fs::read_dir(output_directory.path())
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        assert_eq!(files.len(), 1);
        assert_eq!(std::fs::read(&files[0]).unwrap(), [1, 2, 3]);
        assert_eq!(files[0].extension().unwrap(), "wav");
    }
}
