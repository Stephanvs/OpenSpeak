use serde::{Deserialize, Serialize};
use specta::Type;
use yapper_domain::{DetectedApp, DetectedAppKind};

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ForegroundWindowInfo {
    pub executable_name: String,
    pub executable_path: Option<String>,
}

pub fn supported_apps() -> Vec<DetectedApp> {
    vec![
        DetectedApp::new(
            DetectedAppKind::Teams,
            &["ms-teams.exe", "teams.exe", "msteams.exe"],
            Some("microsoft\\teams"),
        ),
        DetectedApp::new(DetectedAppKind::Slack, &["slack.exe"], Some("slack")),
        DetectedApp::new(DetectedAppKind::Zoom, &["zoom.exe"], Some("zoom")),
    ]
}

pub fn detect_supported_app(info: &ForegroundWindowInfo) -> Option<DetectedApp> {
    let executable_name = normalize(&info.executable_name);
    let executable_path = info.executable_path.as_deref().map(normalize);

    supported_apps().into_iter().find(|candidate| {
        candidate
            .executable_names
            .iter()
            .any(|name| normalize(name) == executable_name)
            || candidate
                .executable_path
                .as_ref()
                .zip(executable_path.as_ref())
                .map(|(needle, haystack)| haystack.contains(&normalize(needle)))
                .unwrap_or(false)
    })
}

fn normalize(value: &str) -> String {
    value.to_ascii_lowercase().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::{detect_supported_app, ForegroundWindowInfo};
    use yapper_domain::DetectedAppKind;

    #[test]
    fn matches_slack_by_executable_name() {
        let detected = detect_supported_app(&ForegroundWindowInfo {
            executable_name: "slack.exe".to_string(),
            executable_path: None,
        })
        .unwrap();

        assert_eq!(detected.kind, DetectedAppKind::Slack);
    }

    #[test]
    fn matches_teams_by_path_fallback() {
        let detected = detect_supported_app(&ForegroundWindowInfo {
            executable_name: "update.exe".to_string(),
            executable_path: Some(
                "C:\\Users\\demo\\AppData\\Local\\Microsoft\\Teams\\current\\teams.exe".to_string(),
            ),
        })
        .unwrap();

        assert_eq!(detected.kind, DetectedAppKind::Teams);
    }
}
