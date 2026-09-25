use std::io::Write;

#[cfg(not(windows))]
use anyhow::bail;
use anyhow::{Context, Result};
use async_trait::async_trait;
use tempfile::Builder;

use crate::{player::Player, providers::SpeechResult};

pub struct SystemPlayer;

#[async_trait]
impl Player for SystemPlayer {
    async fn play(&self, result: &SpeechResult) -> Result<()> {
        let extension = normalize_extension(&result.file_extension);
        let mut file = Builder::new()
            .prefix("openspeak-play-")
            .suffix(&extension)
            .tempfile()
            .context("Could not create a temporary audio file.")?;
        file.write_all(&result.audio_bytes)
            .context("Could not write the temporary audio file.")?;
        file.flush()
            .context("Could not flush the temporary audio file.")?;

        play_path(file.path(), &extension).await
    }
}

fn normalize_extension(file_extension: &str) -> String {
    let extension = file_extension.trim();
    if extension.is_empty() {
        ".bin".to_owned()
    } else if extension.starts_with('.') {
        extension.to_owned()
    } else {
        format!(".{extension}")
    }
}

#[cfg(target_os = "macos")]
async fn play_path(path: &std::path::Path, _extension: &str) -> Result<()> {
    play_command("afplay", &[], path).await
}

#[cfg(target_os = "linux")]
async fn play_path(path: &std::path::Path, extension: &str) -> Result<()> {
    if extension.eq_ignore_ascii_case(".mp3") {
        play_command("mpg123", &["-q"], path).await
    } else {
        play_command("aplay", &["-q"], path).await
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
async fn play_command(command: &str, arguments: &[&str], path: &std::path::Path) -> Result<()> {
    let status = tokio::process::Command::new(command)
        .args(arguments)
        .arg(path)
        .kill_on_drop(true)
        .status()
        .await
        .with_context(|| format!("Could not start the system audio player '{command}'."))?;
    if !status.success() {
        bail!("The system audio player '{command}' exited with {status}.");
    }
    Ok(())
}

#[cfg(windows)]
async fn play_path(path: &std::path::Path, _extension: &str) -> Result<()> {
    let path = path.to_owned();
    tokio::task::spawn_blocking(move || windows::play_blocking(&path))
        .await
        .context("The Windows audio player task failed.")?
}

#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
async fn play_path(_path: &std::path::Path, _extension: &str) -> Result<()> {
    bail!("Audio playback is not supported on this operating system.")
}

#[cfg(windows)]
#[allow(unsafe_code)]
mod windows {
    use std::{ffi::OsStr, os::windows::ffi::OsStrExt, path::Path, ptr::null_mut};

    use anyhow::{Result, bail};
    use uuid::Uuid;
    use windows_sys::Win32::Media::Multimedia::{mciGetErrorStringW, mciSendStringW};

    pub fn play_blocking(path: &Path) -> Result<()> {
        let alias = format!("openspeak{}", Uuid::new_v4().simple());
        send(&format!("open \"{}\" alias {alias}", path.display()))?;
        let play_result = send(&format!("play {alias} wait"));
        let close_result = send(&format!("close {alias}"));
        play_result.and(close_result)
    }

    fn send(command: &str) -> Result<()> {
        const MESSAGE_CAPACITY: u32 = 256;
        let command: Vec<u16> = OsStr::new(command).encode_wide().chain(Some(0)).collect();
        // SAFETY: `command` is a valid null-terminated UTF-16 string. No return
        // buffer or callback window is supplied, as permitted by mciSendStringW.
        let error = unsafe { mciSendStringW(command.as_ptr(), null_mut(), 0, null_mut()) };
        if error == 0 {
            return Ok(());
        }

        let mut message = [0_u16; MESSAGE_CAPACITY as usize];
        // SAFETY: `message` is a writable buffer of the exact length provided.
        let has_message =
            unsafe { mciGetErrorStringW(error, message.as_mut_ptr(), MESSAGE_CAPACITY) } != 0;
        if has_message {
            let length = message
                .iter()
                .position(|character| *character == 0)
                .unwrap_or(0);
            bail!(
                "Windows audio playback failed ({error}): {}",
                String::from_utf16_lossy(&message[..length])
            );
        }
        bail!("Windows audio playback failed with MCI error {error}.")
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_extension;

    #[test]
    fn normalizes_file_extensions() {
        assert_eq!(normalize_extension("mp3"), ".mp3");
        assert_eq!(normalize_extension(".wav"), ".wav");
        assert_eq!(normalize_extension(" "), ".bin");
    }
}
