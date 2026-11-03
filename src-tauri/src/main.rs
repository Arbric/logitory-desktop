// Release builds on Windows run without a console window behind the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    logitory_lib::run()
}
