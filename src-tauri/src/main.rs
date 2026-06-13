// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod autostart;
mod commands;
mod config;
mod process;
mod tray;

use std::sync::Mutex;
use std::io::Write;

macro_rules! log {
    ($($arg:tt)*) => {
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(std::env::temp_dir().join("ccr-tray.log"))
        {
            let _ = writeln!(f, $($arg)*);
        }
    };
}

#[allow(dead_code)]
struct AppState {
    config: Mutex<config::AppConfig>,
}

fn main() {
    log!("[ccr-tray] Starting...");

    let result = tauri::Builder::default()
        .manage(AppState {
            config: Mutex::new(config::AppConfig::default()),
        })
        .setup(|app| {
            log!("[ccr-tray] Setup started");

            // Auto-start services in background
            std::thread::spawn(|| {
                log!("[ccr-tray] Auto-starting services...");
                match process::start_services() {
                    Ok(msg) => log!("[ccr-tray] Services: {}", msg),
                    Err(e) => log!("[ccr-tray] Service error: {}", e),
                }
            });

            // Build tray
            log!("[ccr-tray] Building tray...");
            match tray::build_tray(&app.handle()) {
                Ok(_) => log!("[ccr-tray] Tray built OK"),
                Err(e) => log!("[ccr-tray] Tray error: {}", e),
            }

            // Apply auto-start
            let config = config::load_config();
            if config.autostart != autostart::is_enabled() {
                let _ = autostart::set_enabled(config.autostart);
            }
            log!("[ccr-tray] Autostart: {}", config.autostart);

            // Status refresh
            tray::start_status_refresh(app.handle().clone());
            log!("[ccr-tray] Setup complete");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_status,
            commands::start_all,
            commands::stop_all,
            commands::get_config,
            commands::save_app_config,
            commands::get_autostatus,
            commands::set_autostart,
        ])
        .run(tauri::generate_context!());

    match result {
        Ok(_) => log!("[ccr-tray] Exited normally"),
        Err(e) => log!("[ccr-tray] Exited with error: {}", e),
    }
}
