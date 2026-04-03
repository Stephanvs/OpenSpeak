use std::{path::Path, process::Command};

use serde::{Deserialize, Serialize};
use specta::Type;
use yapper_domain::ModelInfo;

const DEFAULT_MODEL_ID: &str = "windows-speech-en-us";

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModelRegistrySummary {
    pub default_model_id: String,
    pub supports_cpu_baseline: bool,
    pub supports_direct_ml: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSegment {
    pub start_seconds: f32,
    pub end_seconds: f32,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecognitionResult {
    text: String,
    audio_position: f32,
    audio_duration: f32,
}

pub struct ModelRegistry;

impl ModelRegistry {
    pub fn builtins(_model_dir: &Path, active_model_id: &str) -> Vec<ModelInfo> {
        vec![ModelInfo {
            id: DEFAULT_MODEL_ID.to_string(),
            name: "Windows Speech Recognition".to_string(),
            description: "Local Windows speech recognizer used by the current desktop build."
                .to_string(),
            size_mb: 0.0,
            installed: true,
            active: active_model_id == DEFAULT_MODEL_ID,
        }]
    }

    pub fn summary() -> ModelRegistrySummary {
        ModelRegistrySummary {
            default_model_id: DEFAULT_MODEL_ID.to_string(),
            supports_cpu_baseline: true,
            supports_direct_ml: false,
        }
    }

    pub fn default_model_id() -> &'static str {
        DEFAULT_MODEL_ID
    }

    pub fn download_model(
        model_id: &str,
        _model_dir: &Path,
        mut on_progress: impl FnMut(f32, &str),
    ) -> Result<(), String> {
        if model_id != DEFAULT_MODEL_ID {
            return Err(format!("unsupported model id: {model_id}"));
        }

        on_progress(100.0, "Windows speech recognition is ready");
        Ok(())
    }

    pub fn transcribe_wav(
        _model_dir: &Path,
        model_id: &str,
        wav_path: &Path,
    ) -> Result<Vec<TranscriptSegment>, String> {
        if model_id != DEFAULT_MODEL_ID {
            return Err(format!("unsupported model id: {model_id}"));
        }

        let wav = wav_path
            .to_str()
            .ok_or_else(|| "audio path contains invalid UTF-8".to_string())?
            .replace('\'', "''");

        let script = format!(
			concat!(
				"$ErrorActionPreference='Stop';",
				"Add-Type -AssemblyName System.Speech;",
				"$engine = New-Object System.Speech.Recognition.SpeechRecognitionEngine([System.Globalization.CultureInfo]::GetCultureInfo('en-US'));",
				"$engine.LoadGrammar((New-Object System.Speech.Recognition.DictationGrammar));",
				"$engine.SetInputToWaveFile('{}');",
				"$results = New-Object System.Collections.Generic.List[object];",
				"while ($true) {{",
				"  try {{ $result = $engine.Recognize(); }} catch [System.InvalidOperationException] {{ break; }}",
				"  if ($null -eq $result) {{ break; }}",
				"  $results.Add([ordered]@{{ text = $result.Text; audioPosition = [float]$result.Audio.AudioPosition.TotalSeconds; audioDuration = [float]$result.Audio.Duration.TotalSeconds }});",
				"}};",
				"$results | ConvertTo-Json -Compress"
			),
			wav
		);

        let output = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .output()
            .map_err(|error| format!("failed to launch Windows speech recognition: {error}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(if stderr.is_empty() {
                "Windows speech recognition failed".to_string()
            } else {
                stderr
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if stdout.is_empty() || stdout == "null" {
            return Ok(Vec::new());
        }

        let parsed = if stdout.starts_with('[') {
            serde_json::from_str::<Vec<RecognitionResult>>(&stdout)
                .map_err(|error| format!("failed to parse recognizer output: {error}"))?
        } else {
            vec![serde_json::from_str::<RecognitionResult>(&stdout)
                .map_err(|error| format!("failed to parse recognizer output: {error}"))?]
        };

        Ok(parsed
            .into_iter()
            .filter_map(|result| {
                let text = result.text.trim().to_string();
                if text.is_empty() {
                    return None;
                }

                Some(TranscriptSegment {
                    start_seconds: result.audio_position,
                    end_seconds: result.audio_position + result.audio_duration,
                    text,
                })
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs, process::Command};

    use super::{ModelRegistry, DEFAULT_MODEL_ID};

    #[test]
    #[cfg(target_os = "windows")]
    fn transcribes_synthesized_speech() {
        let wav_path = env::temp_dir().join("yapper-speech-test.wav");
        let wav = wav_path.to_str().unwrap().replace('\'', "''");
        let script = format!(
			"Add-Type -AssemblyName System.Speech; $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; $synth.SetOutputToWaveFile('{}'); $synth.Speak('meeting note test from yapper'); $synth.SetOutputToNull(); $synth.Dispose();",
			wav
		);

        let status = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .status()
            .expect("speech synthesis should run");
        assert!(status.success());

        let segments =
            ModelRegistry::transcribe_wav(std::path::Path::new("."), DEFAULT_MODEL_ID, &wav_path)
                .expect("transcription should succeed");
        assert!(!segments.is_empty());
        assert!(segments
            .iter()
            .any(|segment| !segment.text.trim().is_empty()));

        let _ = fs::remove_file(wav_path);
    }
}
