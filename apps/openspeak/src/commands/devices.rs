#[cfg(not(any(target_os = "linux", windows)))]
use anyhow::bail;
use anyhow::{Context, Result};

use crate::cli::{DevicesArgs, DevicesCommand};

/// List the output devices requested by the CLI.
///
/// # Errors
///
/// Returns an error if the operating system cannot enumerate audio devices.
pub async fn execute(args: DevicesArgs) -> Result<()> {
    match args.command {
        DevicesCommand::List => {
            let devices = tokio::task::spawn_blocking(list_output_devices)
                .await
                .context("The audio device discovery task failed.")??;
            if devices.is_empty() {
                println!("No output audio devices found.");
            } else {
                for device in devices {
                    println!("{device}");
                }
            }
            Ok(())
        }
    }
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn list_output_devices() -> Result<Vec<String>> {
    use anyhow::anyhow;
    use windows::Win32::{
        Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
        Media::Audio::{DEVICE_STATE_ACTIVE, IMMDeviceEnumerator, MMDeviceEnumerator, eRender},
        System::{
            Com::{
                CLSCTX_ALL, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize,
                STGM_READ, StructuredStorage::PropVariantClear,
            },
            Variant::VT_LPWSTR,
        },
    };

    // SAFETY: This call runs on a dedicated blocking thread. Every successful
    // initialization is paired with CoUninitialize below on that same thread.
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok()? };
    let result = (|| {
        // SAFETY: COM is initialized and these are the Core Audio class and interface types.
        let enumerator: IMMDeviceEnumerator =
            unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)? };
        // SAFETY: The enumerator and collection are valid COM interfaces.
        let collection = unsafe { enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)? };
        let count = unsafe { collection.GetCount()? };
        let mut devices = Vec::new();
        for index in 0..count {
            // SAFETY: The index is within the collection and the property store is read-only.
            let endpoint = unsafe { collection.Item(index)? };
            let store = unsafe { endpoint.OpenPropertyStore(STGM_READ)? };
            let mut value = unsafe { store.GetValue(&PKEY_Device_FriendlyName)? };
            // SAFETY: GetValue returns an initialized PROPVARIANT. Read the
            // null-terminated name before clearing its owned storage.
            let name = unsafe {
                if value.Anonymous.Anonymous.vt == VT_LPWSTR {
                    let pointer = value.Anonymous.Anonymous.Anonymous.pwszVal;
                    if pointer.is_null() {
                        Err(anyhow!("Core Audio provided a null device friendly name."))
                    } else {
                        Ok(pointer.to_string()?)
                    }
                } else {
                    Err(anyhow!(
                        "Core Audio did not provide a device friendly name."
                    ))
                }
            };
            unsafe { PropVariantClear(&raw mut value)? };
            devices.push(format!("{index}: {}", name?));
        }
        Ok(devices)
    })();
    // SAFETY: CoInitializeEx succeeded on this thread.
    unsafe { CoUninitialize() };
    result
}

#[cfg(target_os = "linux")]
fn list_output_devices() -> Result<Vec<String>> {
    use std::{fs, process::Command};

    if let Ok(output) = Command::new("pactl")
        .args(["list", "sinks"])
        .env("LC_ALL", "C")
        .output()
    {
        if output.status.success() {
            let devices = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter_map(|line| line.trim().strip_prefix("Description: "))
                .map(str::to_owned)
                .collect();
            return Ok(devices);
        }
    }

    let pcm = fs::read_to_string("/proc/asound/pcm")
        .context("Could not query audio sinks or read /proc/asound/pcm.")?;
    Ok(pcm
        .lines()
        .filter(|line| line.contains("playback"))
        .map(|line| line.split(" : playback").next().unwrap_or(line).to_owned())
        .collect())
}

#[cfg(target_os = "macos")]
fn list_output_devices() -> Result<Vec<String>> {
    use std::process::Command;

    let output = Command::new("system_profiler")
        .arg("SPAudioDataType")
        .output()
        .context("Could not start macOS audio device discovery.")?;
    if !output.status.success() {
        bail!(
            "macOS audio device discovery failed with {}.",
            output.status
        );
    }
    let report = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();
    let mut name: Option<&str> = None;
    let mut has_output = false;
    for line in report.lines() {
        let trimmed = line.trim();
        if line.starts_with("        ") && !line.starts_with("          ") && trimmed.ends_with(':')
        {
            if has_output {
                if let Some(previous) = name {
                    devices.push(previous.to_owned());
                }
            }
            name = Some(trimmed.trim_end_matches(':'));
            has_output = false;
        } else if trimmed.starts_with("Output Channels:")
            || trimmed.starts_with("Default Output Device:")
        {
            has_output = true;
        }
    }
    if has_output {
        if let Some(previous) = name {
            devices.push(previous.to_owned());
        }
    }
    Ok(devices)
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
fn list_output_devices() -> Result<Vec<String>> {
    bail!("Audio device discovery is not supported on this operating system.")
}
