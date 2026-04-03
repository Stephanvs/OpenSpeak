use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

use chrono::Local;
use serde::Serialize;
use tauri::{Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;
use tauri_specta::{collect_commands, Builder};
use yapper_audio_windows::{MicrophoneCapture, WindowsAudioRuntime};
use yapper_domain::{
    AudioLevels, DetectedApp, DownloadState, ModelDownload, ModelInfo, SessionSnapshot,
    SessionState, SessionType, SettingValue, Speaker, TargetAppMode, UpdateInfo, UpdateStatus,
    Utterance,
};
use yapper_output::{render_transcript, TranscriptRenderInput};
use yapper_platform_windows::{detect_supported_app, supported_apps, ForegroundWindowInfo};
use yapper_settings::{apply_setting, migrate, AppSettings};
use yapper_transcribe::{ModelRegistry, TranscriptSegment};

const SESSION_STATE_EVENT: &str = "session-state";
const FINAL_TRANSCRIPT_EVENT: &str = "final-transcript";
const AUDIO_LEVELS_EVENT: &str = "audio-levels";
const MODEL_DOWNLOAD_PROGRESS_EVENT: &str = "model-download-progress";
const FOREGROUND_APP_CHANGED_EVENT: &str = "foreground-app-changed";
const UPDATE_PROGRESS_EVENT: &str = "update-progress";
const CAPTURE_ERROR_EVENT: &str = "capture-error";

struct InnerState {
    settings: AppSettings,
    models: Vec<ModelInfo>,
    current_download: Option<ModelDownload>,
    session: SessionSnapshot,
    current_foreground_app: Option<DetectedApp>,
    update_info: UpdateInfo,
    active_recording: Option<ActiveRecording>,
}

struct ActiveRecording {
    session_type: SessionType,
    created_at: chrono::DateTime<chrono::FixedOffset>,
    source_app: Option<DetectedApp>,
    recorder: MicrophoneCapture,
}

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Mutex<InnerState>>,
}

impl AppState {
    fn new() -> Self {
        let settings = load_persisted_settings();
        let models = ModelRegistry::builtins(
            Path::new(&settings.models.model_storage_folder),
            &settings.models.active_model_id,
        );

        Self {
            inner: Arc::new(Mutex::new(InnerState {
                settings,
                models,
                current_download: None,
                session: SessionSnapshot {
                    status_message: "Ready".to_string(),
                    ..SessionSnapshot::default()
                },
                current_foreground_app: detect_supported_app(&ForegroundWindowInfo {
                    executable_name: "ms-teams.exe".to_string(),
                    executable_path: Some(
                        "C:\\Program Files\\WindowsApps\\MSTeams\\ms-teams.exe".to_string(),
                    ),
                }),
                update_info: UpdateInfo {
                    status: UpdateStatus::Idle,
                    current_version: env!("CARGO_PKG_VERSION").to_string(),
                    latest_version: None,
                    release_notes_url: Some(
                        "https://github.com/stephanvs/yapper/releases".to_string(),
                    ),
                },
                active_recording: None,
            })),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionStatePayload {
    session: SessionSnapshot,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct FinalTranscriptPayload {
    utterances: Vec<Utterance>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AudioLevelsPayload {
    levels: AudioLevels,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ModelDownloadProgressPayload {
    progress: ModelDownload,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ForegroundAppChangedPayload {
    app: Option<DetectedApp>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProgressPayload {
    update: UpdateInfo,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CaptureErrorPayload {
    message: String,
}

#[tauri::command]
#[specta::specta]
fn get_settings(state: State<'_, AppState>) -> AppSettings {
    state.inner.lock().unwrap().settings.clone()
}

#[tauri::command]
#[specta::specta]
fn set_setting(
    key: String,
    value: SettingValue,
    state: State<'_, AppState>,
) -> Result<AppSettings, String> {
    let mut inner = state.inner.lock().unwrap();
    apply_setting(&mut inner.settings, &key, value)?;
    refresh_models_locked(&mut inner);
    save_persisted_settings(&inner.settings)?;
    Ok(inner.settings.clone())
}

#[tauri::command]
#[specta::specta]
fn pick_vault_folder(app: tauri::AppHandle) -> Option<String> {
    app.dialog()
        .file()
        .blocking_pick_folder()
        .and_then(|path| path.into_path().ok())
        .map(|path| path.display().to_string())
}

#[tauri::command]
#[specta::specta]
fn list_models(state: State<'_, AppState>) -> Vec<ModelInfo> {
    let mut inner = state.inner.lock().unwrap();
    refresh_models_locked(&mut inner);
    inner.models.clone()
}

#[tauri::command]
#[specta::specta]
fn download_model(
    model_id: String,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<ModelDownload, String> {
    let progress = {
        let mut inner = state.inner.lock().unwrap();
        let progress = ModelDownload {
            model_id: model_id.clone(),
            progress: 0.0,
            state: DownloadState::Downloading,
            message: "Starting model download".to_string(),
        };
        inner.current_download = Some(progress.clone());
        progress
    };

    let state_handle = state.inner.clone();
    let app_handle = app.clone();
    thread::spawn(move || {
        let result =
            ModelRegistry::download_model(&model_id, Path::new("."), |percent, message| {
                let snapshot = {
                    let mut inner = state_handle.lock().unwrap();
                    if let Some(download) = inner.current_download.as_mut() {
                        download.progress = percent;
                        download.message = message.to_string();
                        download.state = DownloadState::Downloading;
                        download.clone()
                    } else {
                        return;
                    }
                };

                let _ = app_handle.emit(
                    MODEL_DOWNLOAD_PROGRESS_EVENT,
                    ModelDownloadProgressPayload { progress: snapshot },
                );
            });

        match result {
            Ok(_) => {
                let final_snapshot = {
                    let mut inner = state_handle.lock().unwrap();
                    inner.settings.models.active_model_id = model_id.clone();
                    let _ = save_persisted_settings(&inner.settings);
                    refresh_models_locked(&mut inner);
                    let snapshot = ModelDownload {
                        model_id: model_id.clone(),
                        progress: 100.0,
                        state: DownloadState::Installed,
                        message: "Model ready".to_string(),
                    };
                    inner.current_download = Some(snapshot.clone());
                    snapshot
                };

                let _ = app_handle.emit(
                    MODEL_DOWNLOAD_PROGRESS_EVENT,
                    ModelDownloadProgressPayload {
                        progress: final_snapshot,
                    },
                );
            }
            Err(error) => {
                let snapshot = {
                    let mut inner = state_handle.lock().unwrap();
                    let snapshot = ModelDownload {
                        model_id: model_id.clone(),
                        progress: 0.0,
                        state: DownloadState::Failed,
                        message: error.clone(),
                    };
                    inner.current_download = Some(snapshot.clone());
                    snapshot
                };

                let _ = app_handle.emit(
                    MODEL_DOWNLOAD_PROGRESS_EVENT,
                    ModelDownloadProgressPayload { progress: snapshot },
                );
            }
        }
    });

    Ok(progress)
}

#[tauri::command]
#[specta::specta]
fn cancel_model_download(model_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut inner = state.inner.lock().unwrap();
    if let Some(download) = inner.current_download.as_mut() {
        if download.model_id == model_id && download.state == DownloadState::Downloading {
            download.state = DownloadState::Cancelled;
            download.message =
                "Download cancellation is not available until the current transfer completes."
                    .to_string();
        }
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
fn start_voice_memo(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<SessionSnapshot, String> {
    start_session(SessionType::VoiceMemo, app, state)
}

#[tauri::command]
#[specta::specta]
fn start_meeting_capture(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<SessionSnapshot, String> {
    start_session(SessionType::MeetingCapture, app, state)
}

#[tauri::command]
#[specta::specta]
fn stop_session(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<SessionSnapshot, String> {
    let (recording, settings, stopping_snapshot) = {
        let mut inner = state.inner.lock().unwrap();
        let Some(recording) = inner.active_recording.take() else {
            return Err("No recording is active.".to_string());
        };

        inner.session.state = SessionState::Stopping;
        inner.session.status_message = "Transcribing audio...".to_string();
        let snapshot = inner.session.clone();
        (recording, inner.settings.clone(), snapshot)
    };

    emit_session_state(&app, &stopping_snapshot);

    let outcome = (|| {
        let captured = recording.recorder.stop()?;
        if captured.samples.len() < 2_000 {
            return Err(
                "The recording was too short to transcribe. Try speaking for a bit longer."
                    .to_string(),
            );
        }

        let wav_path = write_temp_wav_file(&captured.samples, captured.sample_rate_hz)?;
        let segments = ModelRegistry::transcribe_wav(
            Path::new(&settings.models.model_storage_folder),
            &settings.models.active_model_id,
            &wav_path,
        )?;
        let _ = fs::remove_file(&wav_path);

        if segments.is_empty() {
            return Err("No speech was detected in the recording.".to_string());
        }

        let utterances = transcript_segments_to_utterances(&segments);
        let duration_seconds = ((captured.samples.len() as f32 / captured.sample_rate_hz as f32)
            .round() as u32)
            .max(1);
        let source_app_name = recording
            .source_app
            .as_ref()
            .map(|app| app.display_name.clone())
            .unwrap_or_else(|| "Microphone".to_string());
        let output_folder = match recording.session_type {
            SessionType::VoiceMemo => settings.general.voice_memo_folder.clone(),
            SessionType::MeetingCapture => settings.general.meeting_vault_folder.clone(),
        };

        let rendered = render_transcript(&TranscriptRenderInput {
            session_type: recording.session_type.clone(),
            created_at: recording.created_at,
            duration_seconds,
            source_app: source_app_name,
            context: default_context_for_session(
                &recording.session_type,
                recording.source_app.as_ref(),
            ),
            utterances: utterances.clone(),
        });
        let note_path = write_note_file(
            Path::new(&output_folder),
            &rendered.filename,
            &rendered.markdown,
        )?;

        Ok((
            utterances,
            note_path,
            recording.session_type,
            recording.source_app,
        ))
    })();

    match outcome {
        Ok((utterances, note_path, session_type, source_app)) => {
            let snapshot = {
                let mut inner = state.inner.lock().unwrap();
                inner.session = SessionSnapshot {
                    session_type,
                    state: SessionState::Idle,
                    source_app: source_app.clone(),
                    locked_target: source_app.clone(),
                    status_message: "Transcript saved".to_string(),
                    started_at: None,
                    note_path: Some(note_path.clone()),
                    live_preview: utterances.clone(),
                };
                inner.session.clone()
            };

            emit_audio_levels(
                &app,
                AudioLevels {
                    microphone: 0.0,
                    system: 0.0,
                },
            );
            let _ = app.emit(
                FINAL_TRANSCRIPT_EVENT,
                FinalTranscriptPayload { utterances },
            );
            emit_session_state(&app, &snapshot);
            Ok(snapshot)
        }
        Err(error) => {
            let snapshot = {
                let mut inner = state.inner.lock().unwrap();
                inner.session.state = SessionState::Error;
                inner.session.status_message = error.clone();
                inner.session.started_at = None;
                inner.session.clone()
            };

            emit_audio_levels(
                &app,
                AudioLevels {
                    microphone: 0.0,
                    system: 0.0,
                },
            );
            emit_capture_error(&app, &error);
            emit_session_state(&app, &snapshot);
            Err(error)
        }
    }
}

#[tauri::command]
#[specta::specta]
fn get_supported_apps() -> Vec<DetectedApp> {
    supported_apps()
}

#[tauri::command]
#[specta::specta]
fn get_detected_foreground_app(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Option<DetectedApp> {
    let detected = state.inner.lock().unwrap().current_foreground_app.clone();
    let _ = app.emit(
        FOREGROUND_APP_CHANGED_EVENT,
        ForegroundAppChangedPayload {
            app: detected.clone(),
        },
    );
    detected
}

#[tauri::command]
#[specta::specta]
fn set_target_app_mode(
    mode: TargetAppMode,
    state: State<'_, AppState>,
) -> Result<TargetAppMode, String> {
    let mut inner = state.inner.lock().unwrap();
    inner.settings.general.target_app_mode = mode.clone();
    save_persisted_settings(&inner.settings)?;
    Ok(mode)
}

#[tauri::command]
#[specta::specta]
fn set_shortcut(shortcut: String, state: State<'_, AppState>) -> Result<String, String> {
    let mut inner = state.inner.lock().unwrap();
    inner.settings.general.recording_shortcut = shortcut.clone();
    save_persisted_settings(&inner.settings)?;
    Ok(shortcut)
}

#[tauri::command]
#[specta::specta]
fn check_for_updates(app: tauri::AppHandle, state: State<'_, AppState>) -> UpdateInfo {
    let update = {
        let mut inner = state.inner.lock().unwrap();
        inner.update_info = UpdateInfo {
            status: UpdateStatus::UpToDate,
            current_version: env!("CARGO_PKG_VERSION").to_string(),
            latest_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            release_notes_url: Some("https://github.com/stephanvs/yapper/releases".to_string()),
        };
        inner.update_info.clone()
    };

    let _ = app.emit(
        UPDATE_PROGRESS_EVENT,
        UpdateProgressPayload {
            update: update.clone(),
        },
    );
    update
}

#[tauri::command]
#[specta::specta]
fn install_update(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<UpdateInfo, String> {
    let update = {
        let mut inner = state.inner.lock().unwrap();
        inner.update_info.status = UpdateStatus::ReadyToInstall;
        inner.update_info.clone()
    };

    let _ = app.emit(
        UPDATE_PROGRESS_EVENT,
        UpdateProgressPayload {
            update: update.clone(),
        },
    );
    Ok(update)
}

fn start_session(
    session_type: SessionType,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<SessionSnapshot, String> {
    let app_for_levels = app.clone();
    let (snapshot, detected_app) = {
        let mut inner = state.inner.lock().unwrap();
        if inner.active_recording.is_some() {
            return Err("A recording is already active.".to_string());
        }

        refresh_models_locked(&mut inner);

        let locked_target =
            pick_locked_target(&inner.settings, inner.current_foreground_app.clone());
        let recorder = WindowsAudioRuntime::start_microphone_capture(
            inner.settings.general.selected_microphone.as_deref(),
            Some(Arc::new(move |level| {
                emit_audio_levels(
                    &app_for_levels,
                    AudioLevels {
                        microphone: level,
                        system: 0.0,
                    },
                );
            })),
        )?;

        let created_at = Local::now().fixed_offset();
        inner.active_recording = Some(ActiveRecording {
            session_type: session_type.clone(),
            created_at,
            source_app: locked_target.clone(),
            recorder,
        });

        inner.session = SessionSnapshot {
            session_type: session_type.clone(),
            state: SessionState::Recording,
            source_app: locked_target.clone(),
            locked_target: locked_target.clone(),
            status_message: match session_type {
                SessionType::VoiceMemo => "Recording voice memo".to_string(),
                SessionType::MeetingCapture => "Recording meeting note".to_string(),
            },
            started_at: Some(created_at.to_rfc3339()),
            note_path: preview_note_path(&inner.settings, &session_type, locked_target.as_ref()),
            live_preview: Vec::new(),
        };

        (inner.session.clone(), locked_target)
    };

    emit_audio_levels(
        &app,
        AudioLevels {
            microphone: 0.0,
            system: 0.0,
        },
    );
    emit_session_state(&app, &snapshot);

    if let Some(app_match) = detected_app {
        let _ = app.emit(
            FOREGROUND_APP_CHANGED_EVENT,
            ForegroundAppChangedPayload {
                app: Some(app_match),
            },
        );
    }

    Ok(snapshot)
}

fn pick_locked_target(
    settings: &AppSettings,
    auto_detected: Option<DetectedApp>,
) -> Option<DetectedApp> {
    match &settings.general.target_app_mode {
        TargetAppMode::Auto => auto_detected,
        TargetAppMode::None => None,
        TargetAppMode::Manual(kind) => supported_apps().into_iter().find(|app| &app.kind == kind),
    }
}

fn transcript_segments_to_utterances(segments: &[TranscriptSegment]) -> Vec<Utterance> {
    segments
        .iter()
        .map(|segment| Utterance {
            speaker: Speaker::you(),
            text: segment.text.clone(),
            timestamp: format_timestamp(segment.start_seconds),
        })
        .collect()
}

fn format_timestamp(seconds: f32) -> String {
    let total_seconds = seconds.max(0.0).round() as u32;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;
    format!("{hours:02}:{minutes:02}:{secs:02}")
}

fn default_context_for_session(
    session_type: &SessionType,
    source_app: Option<&DetectedApp>,
) -> String {
    match session_type {
        SessionType::VoiceMemo => "Voice memo".to_string(),
        SessionType::MeetingCapture => source_app
            .map(|app| {
                format!(
                    "Meeting captured while {} was the active app",
                    app.display_name
                )
            })
            .unwrap_or_else(|| "Meeting note".to_string()),
    }
}

fn write_note_file(base_folder: &Path, filename: &str, markdown: &str) -> Result<String, String> {
    if base_folder.as_os_str().is_empty() {
        return Err("Choose an output folder before recording.".to_string());
    }

    fs::create_dir_all(base_folder)
        .map_err(|error| format!("failed to create output folder: {error}"))?;
    let note_path = dedupe_note_path(base_folder.to_path_buf(), filename);
    fs::write(&note_path, markdown)
        .map_err(|error| format!("failed to write transcript note: {error}"))?;
    Ok(note_path.display().to_string())
}

fn write_temp_wav_file(samples: &[f32], sample_rate_hz: u32) -> Result<PathBuf, String> {
    let temp_path = env::temp_dir().join(format!(
        "yapper-recording-{}.wav",
        Local::now().timestamp_millis()
    ));

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate_hz,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(&temp_path, spec)
        .map_err(|error| format!("failed to create temporary WAV file: {error}"))?;

    for sample in samples {
        let pcm = (sample.clamp(-1.0, 1.0) * i16::MAX as f32).round() as i16;
        writer
            .write_sample(pcm)
            .map_err(|error| format!("failed to write temporary WAV sample: {error}"))?;
    }

    writer
        .finalize()
        .map_err(|error| format!("failed to finalize temporary WAV file: {error}"))?;

    Ok(temp_path)
}

fn emit_session_state(app: &tauri::AppHandle, snapshot: &SessionSnapshot) {
    let _ = app.emit(
        SESSION_STATE_EVENT,
        SessionStatePayload {
            session: snapshot.clone(),
        },
    );
}

fn emit_audio_levels(app: &tauri::AppHandle, levels: AudioLevels) {
    let _ = app.emit(AUDIO_LEVELS_EVENT, AudioLevelsPayload { levels });
}

fn emit_capture_error(app: &tauri::AppHandle, message: &str) {
    let _ = app.emit(
        CAPTURE_ERROR_EVENT,
        CaptureErrorPayload {
            message: message.to_string(),
        },
    );
}

fn refresh_models_locked(inner: &mut InnerState) {
    inner.models = ModelRegistry::builtins(
        Path::new(&inner.settings.models.model_storage_folder),
        &inner.settings.models.active_model_id,
    );
}

fn preview_note_path(
    settings: &AppSettings,
    session_type: &SessionType,
    source_app: Option<&DetectedApp>,
) -> Option<String> {
    let base_folder = match session_type {
        SessionType::VoiceMemo => &settings.general.voice_memo_folder,
        SessionType::MeetingCapture => &settings.general.meeting_vault_folder,
    };
    let rendered = render_transcript(&TranscriptRenderInput {
        session_type: session_type.clone(),
        created_at: Local::now().fixed_offset(),
        duration_seconds: 0,
        source_app: source_app
            .map(|app| app.display_name.clone())
            .unwrap_or_else(|| "Microphone".to_string()),
        context: default_context_for_session(session_type, source_app),
        utterances: Vec::new(),
    });

    Some(
        Path::new(base_folder)
            .join(rendered.filename)
            .display()
            .to_string(),
    )
}

fn settings_file_path() -> PathBuf {
    let base_dir = env::var("LOCALAPPDATA")
        .ok()
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    base_dir.join("Yapper").join("settings.json")
}

fn load_persisted_settings() -> AppSettings {
    let settings_path = settings_file_path();

    fs::read_to_string(settings_path)
        .ok()
        .and_then(|contents| serde_json::from_str::<AppSettings>(&contents).ok())
        .map(migrate)
        .unwrap_or_default()
}

fn save_persisted_settings(settings: &AppSettings) -> Result<(), String> {
    let settings_path = settings_file_path();
    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create settings folder: {error}"))?;
    }

    let serialized = serde_json::to_string_pretty(settings)
        .map_err(|error| format!("failed to serialize settings: {error}"))?;
    fs::write(settings_path, serialized)
        .map_err(|error| format!("failed to save settings: {error}"))
}

fn dedupe_note_path(folder: PathBuf, filename: &str) -> PathBuf {
    let candidate = folder.join(filename);
    if !candidate.exists() {
        return candidate;
    }

    let stem = candidate
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("transcript");
    let extension = candidate
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("md");

    for index in 2..1000 {
        let next = folder.join(format!("{stem} ({index}).{extension}"));
        if !next.exists() {
            return next;
        }
    }

    candidate
}

fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![
        get_settings,
        set_setting,
        pick_vault_folder,
        list_models,
        download_model,
        cancel_model_download,
        start_voice_memo,
        start_meeting_capture,
        stop_session,
        get_supported_apps,
        get_detected_foreground_app,
        set_target_app_mode,
        set_shortcut,
        check_for_updates,
        install_update,
    ])
}

pub fn builder() -> tauri::Builder<tauri::Wry> {
    let specta_builder = specta_builder();
    let invoke_handler = specta_builder.invoke_handler();
    let state = AppState::new();
    let capture_plan = WindowsAudioRuntime::plan();

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(invoke_handler)
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(move |app| {
            log::info!(
                "Windows capture plan: mic={}, full_loopback={}, process_loopback={}",
                capture_plan.supports_microphone,
                capture_plan.supports_full_system_loopback,
                capture_plan.supports_process_loopback,
            );

            let app_state = app.state::<AppState>();
            let detected = app_state
                .inner
                .lock()
                .unwrap()
                .current_foreground_app
                .clone();
            let _ = app.emit(
                FOREGROUND_APP_CHANGED_EVENT,
                ForegroundAppChangedPayload { app: detected },
            );

            Ok(())
        })
}

#[cfg(test)]
mod tests {
    use super::specta_builder;
    use specta_typescript::Typescript;

    #[test]
    #[ignore = "Requires executing a Tauri-linked test binary in a matching Windows runtime environment."]
    fn export_bindings() {
        specta_builder()
            .export(
                Typescript::default(),
                "../../apps/yapper/src/lib/bindings.ts",
            )
            .expect("bindings should export cleanly");
    }
}
