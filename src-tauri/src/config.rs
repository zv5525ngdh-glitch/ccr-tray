use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default = "default_ccr_port")]
    pub ccr_port: u16,
    #[serde(default = "default_proxy_port")]
    pub proxy_port: u16,
    #[serde(default = "default_ollama_port")]
    pub ollama_port: u16,
    #[serde(default = "default_ollama_model")]
    pub ollama_model: String,
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    #[serde(default = "default_autostart")]
    pub autostart: bool,
}

fn default_ccr_port() -> u16 { 3456 }
fn default_proxy_port() -> u16 { 3457 }
fn default_ollama_port() -> u16 { 11434 }
fn default_ollama_model() -> String { "qwen3-vl:8b".to_string() }
fn default_temperature() -> f64 { 0.7 }
fn default_autostart() -> bool { true }

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ccr_port: default_ccr_port(),
            proxy_port: default_proxy_port(),
            ollama_port: default_ollama_port(),
            ollama_model: default_ollama_model(),
            temperature: default_temperature(),
            autostart: default_autostart(),
        }
    }
}

fn config_path() -> PathBuf {
    let home = dirs_next_home();
    home.join(".claude").join("ccr-tray").join("config.json")
}

fn config_dir() -> PathBuf {
    let home = dirs_next_home();
    home.join(".claude").join("ccr-tray")
}

fn dirs_next_home() -> PathBuf {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

pub fn load_config() -> AppConfig {
    let path = config_path();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => eprintln!("[ccr-tray] Config parse error: {}", e),
                }
            }
            Err(e) => eprintln!("[ccr-tray] Config read error: {}", e),
        }
    }
    AppConfig::default()
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let dir = config_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("无法创建配置目录: {}", e))?;
    let path = config_path();
    let content = serde_json::to_string_pretty(config).map_err(|e| format!("序列化失败: {}", e))?;
    fs::write(&path, content).map_err(|e| format!("写入失败: {}", e))?;
    Ok(())
}
