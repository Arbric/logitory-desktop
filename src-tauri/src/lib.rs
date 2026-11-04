// Logitory desktop: one window on https://logitory.io/login/ (see tauri.conf.json). The web app is
// served live, so every web deploy reaches the desktop app without a new installer.
//
// The window is built here rather than from the config (`"create": false` there) so that links the
// app opens in a new tab (Privacy Policy, Pay Invoice, Stripe receipts…) and mail / phone links
// (support@logitory.io) go to the computer's default browser or mail app; a desktop window has no
// tabs of its own.
use tauri::webview::NewWindowResponse;
use tauri::{Url, WebviewWindowBuilder};

// Hands a link to the operating system (default browser, mail app, …).
fn open_outside(url: &Url) {
    if let Err(err) = tauri_plugin_opener::open_url(url.as_str(), None::<&str>) {
        eprintln!("couldn't open {url}: {err}");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let config = app
                .config()
                .app
                .windows
                .first()
                .cloned()
                .expect("tauri.conf.json defines the main window");
            WebviewWindowBuilder::from_config(app.handle(), &config)?
                // target="_blank" links and window.open(): the default browser instead.
                .on_new_window(|url, _features| {
                    open_outside(&url);
                    NewWindowResponse::Deny
                })
                // mailto: / tel: links: the mail or phone app; web pages stay in the window.
                .on_navigation(|url| {
                    if matches!(url.scheme(), "http" | "https") {
                        true
                    } else {
                        open_outside(url);
                        false
                    }
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Logitory");
}
