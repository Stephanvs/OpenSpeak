use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AudioSourceKind {
    Microphone,
    SystemLoopback,
    TargetedLoopback,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AudioFrameSummary {
    pub source: AudioSourceKind,
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub buffered_samples: usize,
}

pub trait CaptureRuntime: Send + Sync {
    fn capture_sources(&self) -> Vec<AudioSourceKind>;
    fn supports_process_targeting(&self) -> bool;
}
