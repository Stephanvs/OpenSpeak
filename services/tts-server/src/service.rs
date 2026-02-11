use std::{collections::HashMap, f32::consts::PI, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::model::{HealthResponse, SynthesizeRequest, SynthesizeResponse, VoiceModel};

#[derive(Clone)]
pub struct AppState {
    pub models: Arc<Vec<VoiceModel>>,
    pub audio_store: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl AppState {
    pub fn new() -> Self {
        let models = vec![
            VoiceModel { id: "piper-en-us-amy".into(), name: "Amy (US)".into(), language: "en-US".into(), backend: "piper".into(), sample_rate_hz: 22_050 },
            VoiceModel { id: "kokoro-en-us".into(), name: "Kokoro (US)".into(), language: "en-US".into(), backend: "kokoro".into(), sample_rate_hz: 24_000 },
            VoiceModel { id: "macos-avspeech".into(), name: "System Voice (macOS)".into(), language: "en-US".into(), backend: "avspeech".into(), sample_rate_hz: 22_050 },
        ];

        Self { models: Arc::new(models), audio_store: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn health(&self) -> HealthResponse {
        HealthResponse { status: "ok".into(), models_loaded: self.models.len() }
    }

    pub async fn synthesize(&self, req: SynthesizeRequest) -> Result<SynthesizeResponse, String> {
        let model = self.models.iter().find(|m| m.id == req.model_id).cloned().ok_or_else(|| format!("Unknown model_id: {}", req.model_id))?;

        let wav = generate_stub_wav(model.sample_rate_hz, &req.text);
        let request_id = Uuid::new_v4().to_string();
        self.audio_store.write().await.insert(request_id.clone(), wav);

        Ok(SynthesizeResponse {
            request_id: request_id.clone(),
            stream_url: format!("/v1/audio/{request_id}"),
            format: req.format,
            sample_rate_hz: model.sample_rate_hz,
        })
    }
}

fn generate_stub_wav(sample_rate: u32, text: &str) -> Vec<u8> {
    let duration_s = (0.25_f32 + (text.len() as f32 * 0.01)).min(4.0);
    let samples = (sample_rate as f32 * duration_s) as usize;
    let mut pcm = Vec::with_capacity(samples * 2);
    for i in 0..samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (0.2 * (2.0 * PI * 440.0 * t).sin() * i16::MAX as f32) as i16;
        pcm.extend_from_slice(&sample.to_le_bytes());
    }

    wav_from_pcm_mono_i16(sample_rate, &pcm)
}

fn wav_from_pcm_mono_i16(sample_rate: u32, pcm: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(44 + pcm.len());
    let byte_rate = sample_rate * 2;
    let block_align = 2u16;
    let subchunk2_size = pcm.len() as u32;
    let chunk_size = 36 + subchunk2_size;

    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&chunk_size.to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&block_align.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&subchunk2_size.to_le_bytes());
    out.extend_from_slice(pcm);
    out
}
