use crate::config::{AppConfig, load_config, save_config};
use crate::process::{get_services_status, start_services, stop_services};

#[tauri::command]
pub fn get_status() -> Result<crate::process::ServicesStatus, String> {
    let config = load_config();
    Ok(get_services_status(config.ccr_port, config.proxy_port, config.ollama_port))
}

#[tauri::command]
pub fn start_all() -> Result<String, String> {
    start_services()
}

#[tauri::command]
pub fn stop_all() -> Result<String, String> {
    stop_services()
}

#[tauri::command]
pub fn get_config() -> Result<AppConfig, String> {
    Ok(load_config())
}

#[tauri::command]
pub fn save_app_config(config: AppConfig) -> Result<String, String> {
    save_config(&config)?;
    Ok("配置已保存".to_string())
}

#[tauri::command]
pub fn get_autostatus() -> Result<bool, String> {
    Ok(crate::autostart::is_enabled())
}

#[tauri::command]
pub fn set_autostart(enabled: bool) -> Result<String, String> {
    crate::autostart::set_enabled(enabled)?;
    Ok(if enabled { "开机自启已启用".to_string() } else { "开机自启已禁用".to_string() })
}
