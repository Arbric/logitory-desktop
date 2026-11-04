// Logitory desktop: one window on https://logitory.io/login/ (see tauri.conf.json). The web app is
// served live, so every web deploy reaches the desktop app without a new installer.
//
// The window is built here rather than from the config (`"create": false` there) so that links the
// app opens in a new tab (Privacy Policy, Pay Invoice, Stripe receipts…) and mail / phone links
// (support@logitory.io) go to the computer's default browser or mail app; a desktop window has no
// tabs of its own.
use tauri::webview::NewWindowResponse;
use tauri::{Url, WebviewWindowBuilder};

// Pages that open inside the Logitory window: the app itself (logitory.io), Logitory's Supabase
// project (sign-in links and redirects) and Stripe (Checkout and the billing portal). Every other
// page goes to the default browser, which shows its address, so a look-alike sign-in page can't pass
// for Logitory in this window without an address bar (security audit finding 13).
const IN_APP_SITES: &[&str] = &["logitory.io", "stripe.com"];
const IN_APP_HOSTS: &[&str] = &["nddjgpzdqjbkcvrimbql.supabase.co"];

fn opens_in_app(url: &Url) -> bool {
    url.scheme() == "https"
        && url.host_str().is_some_and(|host| {
            IN_APP_HOSTS.contains(&host)
                || IN_APP_SITES
                    .iter()
                    .any(|site| host == *site || host.ends_with(&format!(".{site}")))
        })
}

// Hands a link to the operating system (default browser, mail app, …).
fn open_outside(url: &Url) {
    if let Err(err) = tauri_plugin_opener::open_url(url.as_str(), None::<&str>) {
        eprintln!("couldn't open {url}: {err}");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Only the Rust side of the opener is used. Its page script (on by default) catches link
        // clicks and asks the app over IPC, which logitory.io isn't allowed to do, so the click was
        // swallowed; with it off, links reach on_new_window / on_navigation below.
        .plugin(
            tauri_plugin_opener::Builder::new()
                .open_js_links_on_click(false)
                .build(),
        )
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
                // Logitory and Stripe pages stay in the window; any other page opens in the default
                // browser, and mailto: / tel: links in the mail or phone app.
                .on_navigation(|url| {
                    if opens_in_app(url) {
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

#[cfg(test)]
mod tests {
    use super::opens_in_app;
    use tauri::Url;

    fn inside(url: &str) -> bool {
        opens_in_app(&Url::parse(url).unwrap())
    }

    #[test]
    fn logitory_stripe_and_the_project_stay_in_the_window() {
        assert!(inside("https://logitory.io/login/"));
        assert!(inside("https://logitory.io/sandbox/?access=x"));
        assert!(inside("https://checkout.stripe.com/c/pay/cs_test"));
        assert!(inside("https://billing.stripe.com/p/session/x"));
        assert!(inside("https://nddjgpzdqjbkcvrimbql.supabase.co/auth/v1/verify?token=x"));
    }

    #[test]
    fn everything_else_goes_to_the_browser() {
        assert!(!inside("https://logitory-login.evil/"));
        assert!(!inside("https://logitory.io.evil.com/login/"));
        assert!(!inside("https://evillogitory.io/"));
        assert!(!inside("https://attacker.supabase.co/"));
        assert!(!inside("http://logitory.io/login/"));
        assert!(!inside("https://google.com/"));
    }
}
