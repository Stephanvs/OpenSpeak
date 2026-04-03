#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    yapper_app_api::builder()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
