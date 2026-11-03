// Logitory desktop: one window on https://logitory.io/login/ (see tauri.conf.json). The web app is
// served live, so every web deploy reaches the desktop app without a new installer.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running Logitory");
}
