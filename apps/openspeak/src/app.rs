use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;

use crate::{
    cli::Command,
    commands,
    config::Config,
    player::{Player, SystemPlayer},
    providers::{
        ElevenLabsProvider, GrokProvider, TextToSpeechProvider, TextToSpeechProviderResolver,
    },
};

pub struct App {
    provider_resolver: TextToSpeechProviderResolver,
    player: Arc<dyn Player>,
}

impl App {
    #[must_use]
    pub fn new(config: Config) -> Self {
        let client = Client::new();
        let providers: Vec<Arc<dyn TextToSpeechProvider>> = vec![
            Arc::new(ElevenLabsProvider::new(client.clone(), config.elevenlabs)),
            Arc::new(GrokProvider::new(client, config.grok)),
        ];

        Self {
            provider_resolver: TextToSpeechProviderResolver::new(
                providers,
                config.default_provider,
            ),
            player: Arc::new(SystemPlayer),
        }
    }

    pub async fn execute(&self, command: Command) -> Result<()> {
        match command {
            Command::Speak(args) => {
                commands::speak::execute(&self.provider_resolver, self.player.as_ref(), &args).await
            }
            Command::Synthesize(args) => {
                commands::synthesize::execute(&self.provider_resolver, &args).await
            }
            Command::Serve => commands::serve::execute(),
        }
    }
}
