use std::sync::{Arc, Mutex};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample, SampleFormat, SizedSample,
};
use serde::{Deserialize, Serialize};
use yapper_audio_core::{AudioSourceKind, CaptureRuntime};
use yapper_domain::DetectedAppKind;

const TARGET_SAMPLE_RATE: u32 = 16_000;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WindowsCapturePlan {
    pub supports_microphone: bool,
    pub supports_full_system_loopback: bool,
    pub supports_process_loopback: bool,
    pub targeted_apps: Vec<DetectedAppKind>,
}

#[derive(Clone, Debug)]
pub struct CapturedAudio {
    pub sample_rate_hz: u32,
    pub samples: Vec<f32>,
}

#[derive(Default)]
pub struct WindowsAudioRuntime;

pub struct MicrophoneCapture {
    stream: cpal::Stream,
    shared: Arc<Mutex<CaptureState>>,
    input_sample_rate_hz: u32,
}

#[derive(Default)]
struct CaptureState {
    samples: Vec<f32>,
}

impl WindowsAudioRuntime {
    pub fn plan() -> WindowsCapturePlan {
        WindowsCapturePlan {
            supports_microphone: true,
            supports_full_system_loopback: true,
            supports_process_loopback: false,
            targeted_apps: vec![
                DetectedAppKind::Teams,
                DetectedAppKind::Slack,
                DetectedAppKind::Zoom,
            ],
        }
    }

    pub fn start_microphone_capture(
        preferred_device_name: Option<&str>,
        level_callback: Option<Arc<dyn Fn(f32) + Send + Sync + 'static>>,
    ) -> Result<MicrophoneCapture, String> {
        let host = cpal::default_host();
        let device = pick_input_device(&host, preferred_device_name)?;
        let config = device
            .default_input_config()
            .map_err(|error| format!("failed to read microphone config: {error}"))?;
        let channels = config.channels() as usize;
        let sample_rate_hz = config.sample_rate().0;
        let shared = Arc::new(Mutex::new(CaptureState::default()));

        let stream = match config.sample_format() {
            SampleFormat::F32 => {
                build_stream::<f32>(&device, &config, channels, shared.clone(), level_callback)?
            }
            SampleFormat::I8 => {
                build_stream::<i8>(&device, &config, channels, shared.clone(), level_callback)?
            }
            SampleFormat::I16 => {
                build_stream::<i16>(&device, &config, channels, shared.clone(), level_callback)?
            }
            SampleFormat::I32 => {
                build_stream::<i32>(&device, &config, channels, shared.clone(), level_callback)?
            }
            SampleFormat::U8 => {
                build_stream::<u8>(&device, &config, channels, shared.clone(), level_callback)?
            }
            other => return Err(format!("unsupported microphone sample format: {other:?}")),
        };

        stream
            .play()
            .map_err(|error| format!("failed to start microphone stream: {error}"))?;

        Ok(MicrophoneCapture {
            stream,
            shared,
            input_sample_rate_hz: sample_rate_hz,
        })
    }
}

impl MicrophoneCapture {
    pub fn stop(self) -> Result<CapturedAudio, String> {
        drop(self.stream);

        let raw = self
            .shared
            .lock()
            .map_err(|_| "failed to stop microphone capture cleanly".to_string())?
            .samples
            .clone();

        let samples = if self.input_sample_rate_hz == TARGET_SAMPLE_RATE {
            raw
        } else {
            resample_linear(&raw, self.input_sample_rate_hz, TARGET_SAMPLE_RATE)
        };

        Ok(CapturedAudio {
            sample_rate_hz: TARGET_SAMPLE_RATE,
            samples,
        })
    }
}

impl CaptureRuntime for WindowsAudioRuntime {
    fn capture_sources(&self) -> Vec<AudioSourceKind> {
        vec![
            AudioSourceKind::Microphone,
            AudioSourceKind::SystemLoopback,
            AudioSourceKind::TargetedLoopback,
        ]
    }

    fn supports_process_targeting(&self) -> bool {
        false
    }
}

fn pick_input_device(
    host: &cpal::Host,
    preferred_device_name: Option<&str>,
) -> Result<cpal::Device, String> {
    if let Some(preferred_name) = preferred_device_name.filter(|value| !value.trim().is_empty()) {
        for device in host
            .input_devices()
            .map_err(|error| format!("failed to enumerate microphones: {error}"))?
        {
            if device
                .name()
                .map(|name| name == preferred_name)
                .unwrap_or(false)
            {
                return Ok(device);
            }
        }
    }

    host.default_input_device()
        .ok_or_else(|| "no microphone input device is available".to_string())
}

fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::SupportedStreamConfig,
    channels: usize,
    shared: Arc<Mutex<CaptureState>>,
    level_callback: Option<Arc<dyn Fn(f32) + Send + Sync + 'static>>,
) -> Result<cpal::Stream, String>
where
    T: Sample + SizedSample + Send + 'static,
    f32: cpal::FromSample<T>,
{
    device
        .build_input_stream(
            &config.clone().into(),
            move |data: &[T], _| {
                let mono = fold_to_mono::<T>(data, channels);
                let level = rms_level(&mono);

                if let Ok(mut state) = shared.lock() {
                    state.samples.extend_from_slice(&mono);
                }

                if let Some(callback) = &level_callback {
                    callback(level);
                }
            },
            move |error| {
                log::error!("microphone stream error: {error}");
            },
            None,
        )
        .map_err(|error| format!("failed to build microphone stream: {error}"))
}

fn fold_to_mono<T>(data: &[T], channels: usize) -> Vec<f32>
where
    T: Sample,
    f32: cpal::FromSample<T>,
{
    if channels <= 1 {
        return data
            .iter()
            .map(|sample| sample.to_sample::<f32>())
            .collect();
    }

    data.chunks_exact(channels)
        .map(|frame| {
            frame
                .iter()
                .map(|sample| sample.to_sample::<f32>())
                .sum::<f32>()
                / channels as f32
        })
        .collect()
}

fn rms_level(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }

    let mean_square =
        samples.iter().map(|sample| sample * sample).sum::<f32>() / samples.len() as f32;
    (mean_square.sqrt() * 3.0).clamp(0.0, 1.0)
}

fn resample_linear(samples: &[f32], input_rate_hz: u32, output_rate_hz: u32) -> Vec<f32> {
    if samples.is_empty() || input_rate_hz == output_rate_hz {
        return samples.to_vec();
    }

    let output_len =
        ((samples.len() as f64 * output_rate_hz as f64) / input_rate_hz as f64).round() as usize;
    let mut output = Vec::with_capacity(output_len.max(1));

    for index in 0..output_len {
        let position = index as f64 * input_rate_hz as f64 / output_rate_hz as f64;
        let left = position.floor() as usize;
        let right = (left + 1).min(samples.len().saturating_sub(1));
        let fraction = (position - left as f64) as f32;
        let left_sample = samples[left];
        let right_sample = samples[right];
        output.push(left_sample + ((right_sample - left_sample) * fraction));
    }

    output
}
