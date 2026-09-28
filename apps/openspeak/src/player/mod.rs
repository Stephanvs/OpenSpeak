pub mod devices;
mod system;

use anyhow::Result;
use async_trait::async_trait;

use crate::providers::SpeechResult;

pub use system::SystemPlayer;

#[async_trait]
pub trait Player: Send + Sync {
    async fn play(&self, result: &SpeechResult, device_id: Option<&str>) -> Result<()>;
}
