use std::env;

use serde::{Deserialize, Serialize};
use specta::Type;
use yapper_domain::{DetectedAppKind, SessionType, SettingValue, TargetAppMode};

pub const SETTINGS_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeneralSettings {
    pub recording_shortcut: String,
    pub default_session_type: SessionType,
    pub selected_microphone: Option<String>,
    pub meeting_vault_folder: String,
    pub voice_memo_folder: String,
    pub target_app_mode: TargetAppMode,
    pub silence_auto_stop_seconds: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModelSettings {
    pub active_model_id: String,
    pub model_storage_folder: String,
    pub prefer_direct_ml: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedSettings {
    pub start_hidden: bool,
    pub launch_at_login: bool,
    pub tray_icon: bool,
    pub check_for_updates: bool,
    pub log_folder: String,
    pub temp_audio_retention_days: u32,
    pub manual_target_app: Option<DetectedAppKind>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub schema_version: u32,
    pub onboarding_complete: bool,
    pub general: GeneralSettings,
    pub models: ModelSettings,
    pub advanced: AdvancedSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        let documents_dir = path_in_profile("Documents\\Yapper");
        let local_app_data = env::var("LOCALAPPDATA")
            .ok()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| format!("{}\\AppData\\Local", home_dir()));

        Self {
            schema_version: SETTINGS_SCHEMA_VERSION,
            onboarding_complete: false,
            general: GeneralSettings {
                recording_shortcut: "Ctrl+Shift+Space".to_string(),
                default_session_type: SessionType::VoiceMemo,
                selected_microphone: None,
                meeting_vault_folder: format!("{}\\Meetings", documents_dir),
                voice_memo_folder: format!("{}\\Voice Memos", documents_dir),
                target_app_mode: TargetAppMode::Auto,
                silence_auto_stop_seconds: 120,
            },
            models: ModelSettings {
                active_model_id: "whisper-tiny-en-q5_1".to_string(),
                model_storage_folder: format!("{}\\Yapper\\Models", local_app_data),
                prefer_direct_ml: false,
            },
            advanced: AdvancedSettings {
                start_hidden: false,
                launch_at_login: false,
                tray_icon: true,
                check_for_updates: true,
                log_folder: format!("{}\\Yapper\\Logs", local_app_data),
                temp_audio_retention_days: 0,
                manual_target_app: None,
            },
        }
    }
}

pub fn migrate(mut settings: AppSettings) -> AppSettings {
    settings.schema_version = SETTINGS_SCHEMA_VERSION;

    if settings.general.meeting_vault_folder.trim().is_empty() {
        settings.general.meeting_vault_folder = AppSettings::default().general.meeting_vault_folder;
    }

    if settings.general.voice_memo_folder.trim().is_empty() {
        settings.general.voice_memo_folder = AppSettings::default().general.voice_memo_folder;
    }

    if settings.models.active_model_id != AppSettings::default().models.active_model_id {
        settings.models.active_model_id = AppSettings::default().models.active_model_id;
    }

    settings
}

pub fn apply_setting(
    settings: &mut AppSettings,
    key: &str,
    value: SettingValue,
) -> Result<(), String> {
    match (key, value) {
        ("general.meetingVaultFolder", SettingValue::String(path)) => {
            settings.general.meeting_vault_folder = path;
        }
        ("general.voiceMemoFolder", SettingValue::String(path)) => {
            settings.general.voice_memo_folder = path;
        }
        ("general.selectedMicrophone", SettingValue::String(device)) => {
            settings.general.selected_microphone = if device.trim().is_empty() {
                None
            } else {
                Some(device)
            };
        }
        ("general.defaultSessionType", SettingValue::SessionType(session_type)) => {
            settings.general.default_session_type = session_type;
        }
        ("general.targetAppMode", SettingValue::TargetAppMode(mode)) => {
            settings.general.target_app_mode = mode;
        }
        ("general.silenceAutoStopSeconds", SettingValue::U32(seconds)) => {
            settings.general.silence_auto_stop_seconds = seconds;
        }
        ("models.activeModelId", SettingValue::String(model_id)) => {
            settings.models.active_model_id = model_id;
        }
        ("models.preferDirectMl", SettingValue::Bool(enabled)) => {
            settings.models.prefer_direct_ml = enabled;
        }
        ("advanced.startHidden", SettingValue::Bool(enabled)) => {
            settings.advanced.start_hidden = enabled;
        }
        ("advanced.launchAtLogin", SettingValue::Bool(enabled)) => {
            settings.advanced.launch_at_login = enabled;
        }
        ("advanced.trayIcon", SettingValue::Bool(enabled)) => {
            settings.advanced.tray_icon = enabled;
        }
        ("advanced.checkForUpdates", SettingValue::Bool(enabled)) => {
            settings.advanced.check_for_updates = enabled;
        }
        ("advanced.tempAudioRetentionDays", SettingValue::U32(days)) => {
            settings.advanced.temp_audio_retention_days = days;
        }
        ("advanced.manualTargetApp", SettingValue::OptionalApp(app)) => {
            settings.advanced.manual_target_app = app;
        }
        ("onboarding.complete", SettingValue::Bool(done)) => {
            settings.onboarding_complete = done;
        }
        _ => return Err(format!("unsupported setting update: {key}")),
    }

    Ok(())
}

fn home_dir() -> String {
    env::var("USERPROFILE")
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| env::var("HOME").ok().filter(|value| !value.is_empty()))
        .unwrap_or_else(|| ".".to_string())
}

fn path_in_profile(tail: &str) -> String {
    format!("{}\\{}", home_dir(), tail)
}

#[cfg(test)]
mod tests {
    use super::{migrate, AppSettings, SETTINGS_SCHEMA_VERSION};

    #[test]
    fn migration_fills_schema_version() {
        let mut settings = AppSettings::default();
        settings.schema_version = 0;
        settings.general.meeting_vault_folder.clear();

        let migrated = migrate(settings);

        assert_eq!(migrated.schema_version, SETTINGS_SCHEMA_VERSION);
        assert!(!migrated.general.meeting_vault_folder.is_empty());
    }
}
