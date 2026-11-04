# Logitory desktop

The Logitory desktop app for Windows (and macOS / Linux), built with [Tauri 2](https://v2.tauri.app).

It's a native window on **https://logitory.io/login/**: the web app is loaded live rather than bundled, so
every deploy of the web app (repo `Arbric/Logitory`) reaches the desktop app straight away, with no new
installer. Logitory needs to be online anyway (it talks to Supabase), and sign-in, invites, Stripe and
password resets all keep working because the window is on the real logitory.io address.

## Files
- `src-tauri/tauri.conf.json`: the app's name, id (`io.logitory.desktop`), window (1280 × 820, resizable,
  down to phone width), the URL it opens (`build.frontendDist` / `devUrl`), and the installer details.
- `src-tauri/src/lib.rs`, `main.rs`: the Rust entry point (just starts the window).
- `src-tauri/icons/`: every icon size, generated from `app-icon.png` (1024 × 1024, the Logitory cube).
  To change the icon, replace `app-icon.png` and run `npm run icons`.

## One-time setup (Windows)
1. **Microsoft C++ Build Tools**: https://aka.ms/vs/17/release/vs_BuildTools.exe — in the installer tick
   **Desktop development with C++**.
2. **Rust**: https://rustup.rs (run `rustup-init.exe`, accept the defaults), then open a new terminal.
3. WebView2 comes with Windows 11, so there's nothing to install for it.
4. In this folder: `npm install`

Check with `npx tauri info`: everything under Environment should have a ✔.

## Run and build
- `npm run dev` opens the app in a window (the first run compiles Rust and takes a few minutes).
- `npm run build` makes the installers in `src-tauri/target/release/bundle/`:
  `msi/Logitory_1.0.0_x64_en-US.msi` and `nsis/Logitory_1.0.0_x64-setup.exe`.

For a new release, raise `version` in `src-tauri/tauri.conf.json` (and `package.json` / `Cargo.toml`).
The web app itself updates without a new release.

## Notes
- **Google / Microsoft / company SSO sign-in** opens the provider's page inside the window. Microsoft and
  most SSO providers work; Google may refuse sign-in from an embedded window, in which case use email and
  password (or a browser).
- **Links that open a new tab** (Privacy Policy, Pay Invoice, Stripe receipts) open in the computer's default
  browser, and mail / phone links (support@logitory.io) in its mail or phone app (`src-tauri/src/lib.rs`,
  Tauri's opener plugin). Everything else stays in the Logitory window.
- **Code signing**: unsigned Windows installers show a SmartScreen warning. A code-signing certificate
  (set in `bundle.windows`) removes it.
- **Desktop updates**: only needed when this shell changes (icon, window, Tauri version). Tauri's updater
  plugin can deliver those automatically if wanted.
