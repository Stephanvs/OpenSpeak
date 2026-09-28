use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputDevice {
    pub name: String,
    pub id: String,
    pub is_default: bool,
}

/// Enumerate output devices in stable identifier order.
///
/// # Errors
///
/// Returns an error if the audio host cannot enumerate or identify a device.
pub fn list() -> Result<Vec<OutputDevice>> {
    let host = cpal::default_host();
    let default_id = host
        .default_output_device()
        .and_then(|device| device.id().ok());
    let mut devices = host
        .output_devices()
        .context("Could not enumerate output audio devices.")?
        .map(|device| {
            let id = device
                .id()
                .context("Could not identify an output audio device.")?;
            let name = device.description().map_or_else(
                |_| "Unknown output device".to_owned(),
                |description| {
                    description
                        .extended()
                        .iter()
                        .find(|line| line.starts_with(description.name()))
                        .cloned()
                        .unwrap_or_else(|| description.to_string())
                },
            );
            Ok(OutputDevice {
                name,
                id: id.to_string(),
                is_default: default_id.as_ref() == Some(&id),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    sort_by_id(&mut devices);
    Ok(devices)
}

fn sort_by_id(devices: &mut [OutputDevice]) {
    devices.sort_by(|a, b| a.id.cmp(&b.id));
}

#[cfg(test)]
mod tests {
    use super::{OutputDevice, sort_by_id};

    #[test]
    fn indexes_follow_stable_ids_even_when_enumeration_changes() {
        let a = OutputDevice {
            name: "Same name".into(),
            id: "host:b".into(),
            is_default: false,
        };
        let b = OutputDevice {
            name: "Same name".into(),
            id: "host:a".into(),
            is_default: true,
        };
        let mut first = vec![a.clone(), b.clone()];
        let mut second = vec![b, a];
        sort_by_id(&mut first);
        sort_by_id(&mut second);
        assert_eq!(first, second);
        assert_eq!(first[0].id, "host:a");
    }
}
