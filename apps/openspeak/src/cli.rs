use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::providers::{SpeechCodec, SpeechOutputSettings, TtsSpeechRequest};

#[derive(Debug, Parser)]
#[command(name = "openspeak", about = "openspeak - text-to-speech")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Convert text to speech and play immediately.
    #[command(alias = "say")]
    Speak(SpeechArgs),

    /// Generate an audio file from text.
    Synthesize(SynthesizeArgs),

    /// Start a local server (placeholder).
    Serve,
}

#[derive(Clone, Debug, Args)]
pub struct SpeechArgs {
    /// Text to speak or synthesize.
    pub text: String,

    /// TTS provider to use, such as elevenlabs or grok.
    #[arg(short, long)]
    pub provider: Option<String>,

    /// Provider-specific voice or voice ID.
    #[arg(short, long)]
    pub voice: Option<String>,

    /// Provider-specific model override.
    #[arg(short, long)]
    pub model: Option<String>,

    /// Output codec.
    #[arg(short, long, value_enum)]
    pub codec: Option<CliSpeechCodec>,

    /// Speech speed override.
    #[arg(short, long)]
    pub speed: Option<f64>,

    /// Output sample rate in Hz when the provider supports it.
    #[arg(long)]
    pub sample_rate: Option<u32>,

    /// Output bit rate in bits per second when the provider supports it.
    #[arg(long)]
    pub bit_rate: Option<u32>,

    /// Language code when the provider supports it.
    #[arg(long)]
    pub language: Option<String>,
}

#[derive(Debug, Args)]
pub struct SynthesizeArgs {
    #[command(flatten)]
    pub speech: SpeechArgs,

    /// Directory where the generated audio file is saved.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CliSpeechCodec {
    Mp3,
    Wav,
    Pcm,
    Mulaw,
    Alaw,
}

impl SpeechArgs {
    pub fn request(&self) -> TtsSpeechRequest {
        let output =
            if self.codec.is_none() && self.sample_rate.is_none() && self.bit_rate.is_none() {
                None
            } else {
                Some(SpeechOutputSettings {
                    codec: self.codec.map(Into::into),
                    sample_rate: self.sample_rate,
                    bit_rate: self.bit_rate,
                })
            };

        TtsSpeechRequest {
            text: self.text.clone(),
            voice: self.voice.clone(),
            model: self.model.clone(),
            output,
            speed: self.speed,
            language_code: self.language.clone(),
        }
    }
}

impl From<CliSpeechCodec> for SpeechCodec {
    fn from(value: CliSpeechCodec) -> Self {
        match value {
            CliSpeechCodec::Mp3 => Self::Mp3,
            CliSpeechCodec::Wav => Self::Wav,
            CliSpeechCodec::Pcm => Self::Pcm,
            CliSpeechCodec::Mulaw => Self::Mulaw,
            CliSpeechCodec::Alaw => Self::Alaw,
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{Cli, Command};
    use crate::providers::SpeechCodec;

    #[test]
    fn say_is_an_alias_for_speak() {
        let cli = Cli::try_parse_from(["openspeak", "say", "hello"]).unwrap();
        assert!(matches!(cli.command, Command::Speak(_)));
    }

    #[test]
    fn speech_options_map_to_the_shared_request() {
        let cli = Cli::try_parse_from([
            "openspeak",
            "synthesize",
            "hello",
            "--codec",
            "wav",
            "--sample-rate",
            "48000",
            "--language",
            "en",
        ])
        .unwrap();

        let Command::Synthesize(args) = cli.command else {
            panic!("expected synthesize command");
        };
        let request = args.speech.request();
        let output = request.output.unwrap();
        assert_eq!(output.codec, Some(SpeechCodec::Wav));
        assert_eq!(output.sample_rate, Some(48_000));
        assert_eq!(request.language_code.as_deref(), Some("en"));
    }
}
