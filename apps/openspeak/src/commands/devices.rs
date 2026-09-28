use anyhow::{Context, Result};

use crate::{
    cli::{DevicesArgs, DevicesCommand},
    player::devices,
};

/// List the output devices requested by the CLI.
///
/// # Errors
///
/// Returns an error if the operating system cannot enumerate audio devices.
pub async fn execute(args: DevicesArgs) -> Result<()> {
    match args.command {
        DevicesCommand::List => {
            let devices = tokio::task::spawn_blocking(devices::list)
                .await
                .context("The audio device discovery task failed.")??;
            if devices.is_empty() {
                println!("No output audio devices found.");
            }
            for (index, device) in devices.iter().enumerate() {
                let default = if device.is_default { " (default)" } else { "" };
                println!("{index}: {}{default}", device.name);
            }
            Ok(())
        }
    }
}
