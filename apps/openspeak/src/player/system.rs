use std::io::Cursor;

use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use cpal::traits::HostTrait;
use rodio::DeviceSinkBuilder;

use crate::{player::Player, providers::SpeechResult};

pub struct SystemPlayer;

#[async_trait]
impl Player for SystemPlayer {
    async fn play(&self, result: &SpeechResult, device_id: Option<&str>) -> Result<()> {
        let audio = result.audio_bytes.clone();
        let device_id = device_id.map(str::to_owned);
        tokio::task::spawn_blocking(move || play_blocking(audio, device_id))
            .await
            .context("The audio playback task failed.")?
    }
}

fn play_blocking(audio: Vec<u8>, selected_id: Option<String>) -> Result<()> {
    let mut stream = if let Some(selected_id) = selected_id {
        let id: cpal::DeviceId = selected_id.parse().context("Invalid output device ID.")?;
        let device = cpal::default_host().device_by_id(&id).with_context(
            || "The selected output device disconnected. Run `devices list` again.",
        )?;
        DeviceSinkBuilder::from_device(device)
            .context("Could not configure the selected output device.")?
            .open_sink_or_fallback()
            .context("Could not open the selected output device.")?
    } else {
        DeviceSinkBuilder::from_default_device()
            .context("No default output audio device is available.")?
            .open_sink_or_fallback()
            .context("Could not open the default output audio device.")?
    };

    if audio.is_empty() {
        bail!("The speech provider returned empty audio.");
    }
    stream.log_on_drop(false);
    let player = rodio::play(stream.mixer(), Cursor::new(audio))
        .context("Could not decode the generated audio for playback.")?;
    player.sleep_until_end();
    Ok(())
}
