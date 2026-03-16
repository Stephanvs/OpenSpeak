use std::fs;

#[cfg(target_os = "macos")]
use objc2_app_kit::{NSColor, NSWindow};
use tauri::{
    menu::{MenuBuilder, MenuEvent, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
};
#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri_plugin_global_shortcut::{Builder as GlobalShortcutBuilder, ShortcutState};
use tauri_plugin_shell::ShellExt;

const GLOBAL_SHORTCUT: &str = "CommandOrControl+Shift+Y";
const MAIN_WINDOW_LABEL: &str = "main";
const MAIN_WINDOW_TITLE: &str = "Yapper";
const MAIN_WINDOW_WIDTH: f64 = 560.0;
const MAIN_WINDOW_HEIGHT: f64 = 420.0;
const MAIN_WINDOW_MIN_WIDTH: f64 = 420.0;
const MAIN_WINDOW_MIN_HEIGHT: f64 = 320.0;
const TRAY_ID: &str = "main-tray";
const TRAY_TOGGLE_UI_ID: &str = "toggle-ui";
const TRAY_QUIT_ID: &str = "quit";
#[cfg(target_os = "macos")]
const MACOS_WINDOW_BACKGROUND: (f64, f64, f64, f64) = (
    232.0 / 255.0,
    240.0 / 255.0,
    247.0 / 255.0,
    1.0,
);

#[tauri::command]
async fn synthesize_text(app: AppHandle, text: String) -> Result<String, String> {
    let text = text.trim().to_string();

    if text.is_empty() {
        return Err("Enter text before generating audio.".to_string());
    }

    let output_dir = std::env::temp_dir().join("yapper-audio");
    fs::create_dir_all(&output_dir)
        .map_err(|error| format!("Failed to create output directory: {error}"))?;
    let output_dir_arg = output_dir.to_string_lossy().to_string();
    let command_output = app
        .shell()
        .sidecar("yap")
        .map_err(|error| format!("Failed to prepare bundled `yap` sidecar: {error}"))?
        .args(vec![
            "synthesize".to_string(),
            text,
            "--output".to_string(),
            output_dir_arg,
        ])
        .output()
        .await
        .map_err(|error| format!("Failed to execute bundled `yap` sidecar: {error}"))?;

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

fn show_main_window(app: &AppHandle) {
    let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) else {
        return;
    };

    let _ = window.unminimize();
    let _ = window.show();
    let _ = window.set_focus();
}

fn toggle_main_window(app: &AppHandle) {
    let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) else {
        return;
    };

    match window.is_visible() {
        Ok(true) => {
            let _ = window.hide();
        }
        Ok(false) | Err(_) => show_main_window(app),
    }
}

fn create_main_window<R: Runtime>(app: &mut tauri::App<R>) -> tauri::Result<()> {
    let window_builder = WebviewWindowBuilder::new(app, MAIN_WINDOW_LABEL, WebviewUrl::default())
        .title(MAIN_WINDOW_TITLE)
        .inner_size(MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT)
        .min_inner_size(MAIN_WINDOW_MIN_WIDTH, MAIN_WINDOW_MIN_HEIGHT)
        .resizable(true)
        .visible(false)
        .always_on_top(true)
        .center();

    #[cfg(target_os = "macos")]
    let window_builder = window_builder
        .title_bar_style(TitleBarStyle::Transparent)
        .hidden_title(true);

    let window = window_builder.build()?;

    #[cfg(target_os = "macos")]
    apply_macos_window_background(&window);

    Ok(())
}

#[cfg(target_os = "macos")]
fn apply_macos_window_background<R: Runtime>(window: &WebviewWindow<R>) {
    let ns_window = match window.ns_window() {
        Ok(ns_window) => ns_window,
        Err(error) => {
            eprintln!(
                "Failed to access native macOS window for `{MAIN_WINDOW_LABEL}`: {error}"
            );
            return;
        }
    };

    let ns_window = ns_window.cast::<NSWindow>();

    if ns_window.is_null() {
        eprintln!("Native macOS window pointer for `{MAIN_WINDOW_LABEL}` was null");
        return;
    }

    unsafe {
        let background_color = NSColor::colorWithSRGBRed_green_blue_alpha(
            MACOS_WINDOW_BACKGROUND.0,
            MACOS_WINDOW_BACKGROUND.1,
            MACOS_WINDOW_BACKGROUND.2,
            MACOS_WINDOW_BACKGROUND.3,
        );
        (&*ns_window).setBackgroundColor(Some(&background_color));
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
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            create_main_window(app)?;

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
