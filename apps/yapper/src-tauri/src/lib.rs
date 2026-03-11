use std::{fs, process::Command};

use tauri::{
    menu::{MenuBuilder, MenuEvent, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};
use tauri_plugin_global_shortcut::{Builder as GlobalShortcutBuilder, ShortcutState};

const GLOBAL_SHORTCUT: &str = "CommandOrControl+Shift+Y";
const TRAY_ID: &str = "main-tray";
const TRAY_TOGGLE_UI_ID: &str = "toggle-ui";
const TRAY_QUIT_ID: &str = "quit";

#[tauri::command]
async fn synthesize_text(text: String) -> Result<String, String> {
    let text = text.trim().to_string();

    if text.is_empty() {
        return Err("Enter text before generating audio.".to_string());
    }

    let output_dir = std::env::temp_dir().join("yapper-audio");
    fs::create_dir_all(&output_dir)
        .map_err(|error| format!("Failed to create output directory: {error}"))?;
    let output_dir_arg = output_dir.to_string_lossy().to_string();
    let yap_binary = std::env::var("YAP_BINARY_PATH").unwrap_or_else(|_| "yap".to_string());
    let yap_binary_for_error = yap_binary.clone();

    let command_output = tauri::async_runtime::spawn_blocking(move || {
        Command::new(&yap_binary)
            .arg("synthesize")
            .arg(text)
            .arg("--output")
            .arg(output_dir_arg)
            .output()
    })
    .await
    .map_err(|error| format!("Failed to wait for yap process: {error}"))?
    .map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            format!(
                "Could not find `{}`. Install the published yap binary and add it to PATH, or set YAP_BINARY_PATH.",
                yap_binary_for_error
            )
        } else {
            format!("Failed to execute yap binary: {error}")
        }
    })?;

    if !command_output.status.success() {
        let stderr = String::from_utf8_lossy(&command_output.stderr);
        let stdout = String::from_utf8_lossy(&command_output.stdout);
        return Err(format!(
            "Audio generation failed.\nstdout:\n{}\nstderr:\n{}",
            stdout.trim(),
            stderr.trim()
        ));
    }

    let stdout = String::from_utf8_lossy(&command_output.stdout);
    let output_path = stdout
        .lines()
        .rev()
        .find_map(|line| {
            line.rsplit_once(" to ")
                .map(|(_, path)| path.trim().to_string())
        })
        .unwrap_or_else(|| output_dir.display().to_string());

    Ok(output_path)
}

fn hide_main_window(app: &AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    let _ = window.hide();
}

fn show_main_window(app: &AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    let _ = window.unminimize();
    let _ = window.show();
    let _ = window.set_focus();
}

fn toggle_main_window(app: &AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    match window.is_visible() {
        Ok(true) => {
            let _ = window.hide();
        }
        Ok(false) | Err(_) => show_main_window(app),
    }
}

fn create_tray_icon(app: &AppHandle) -> tauri::Result<()> {
    let toggle_ui = MenuItemBuilder::with_id(TRAY_TOGGLE_UI_ID, "Toggle UI").build(app)?;
    let quit = MenuItemBuilder::with_id(TRAY_QUIT_ID, "Quit").build(app)?;
    let menu = MenuBuilder::new(app).items(&[&toggle_ui, &quit]).build()?;

    let mut tray_builder = TrayIconBuilder::with_id(TRAY_ID)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .tooltip("Yapper")
        .on_menu_event(|app, event: MenuEvent| match event.id().as_ref() {
            TRAY_TOGGLE_UI_ID => toggle_main_window(app),
            TRAY_QUIT_ID => app.exit(0),
            _ => {}
        });

    if let Some(icon) = app.default_window_icon() {
        tray_builder = tray_builder.icon(icon.clone());
    }

    let _tray = tray_builder.build(app)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            hide_main_window(&app.handle());

            #[cfg(desktop)]
            {
                app.handle().plugin(
                    GlobalShortcutBuilder::new()
                        .with_shortcuts([GLOBAL_SHORTCUT])?
                        .with_handler(|app, _shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                toggle_main_window(app);
                            }
                        })
                        .build(),
                )?;

                create_tray_icon(&app.handle())?;
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![synthesize_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
