use serde::{Deserialize, Serialize};
use std::process::Command;
use std::net::TcpStream;
use std::time::Duration;

// Windows-only: suppress console window flash
#[cfg(windows)]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessStatus {
    pub name: String,
    pub running: bool,
    pub pid: Option<u32>,
    pub uptime: Option<String>,
    pub cpu: Option<f64>,
    pub memory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServicesStatus {
    pub ccr: ProcessStatus,
    pub proxy: ProcessStatus,
    pub ccr_port_open: bool,
    pub proxy_port_open: bool,
    pub ollama_port_open: bool,
    pub all_healthy: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Pm2Process {
    name: String,
    pid: Option<u32>,
    pm2_env: Option<Pm2Env>,
    monit: Option<Pm2Monit>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Pm2Env {
    status: Option<String>,
    pm_uptime: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct Pm2Monit {
    cpu: Option<f64>,
    memory: Option<u64>,
}

fn pm2_cmd() -> String {
    let home = std::env::var("APPDATA").unwrap_or_default();
    let pm2_path = format!("{}\\npm\\pm2.cmd", home);
    if std::path::Path::new(&pm2_path).exists() {
        pm2_path
    } else {
        "pm2".to_string()
    }
}

fn run_pm2(args: &[&str]) -> Result<String, String> {
    let mut cmd = Command::new(pm2_cmd());
    cmd.args(args);
    #[cfg(windows)]
    {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    let output = cmd.output()
        .map_err(|e| format!("pm2 命令执行失败: {}", e))?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Native TCP connect check — zero shell, zero flash
fn check_port(port: u16) -> bool {
    TcpStream::connect_timeout(
        &format!("127.0.0.1:{}", port).parse().unwrap(),
        Duration::from_millis(200),
    )
    .is_ok()
}

pub fn get_services_status(ccr_port: u16, proxy_port: u16, ollama_port: u16) -> ServicesStatus {
    let ccr_running = check_port(ccr_port);
    let proxy_running = check_port(proxy_port);
    let ollama_running = check_port(ollama_port);

    // Try to get PM2 details
    let pm2_ccr = get_pm2_process("ccr");
    let pm2_proxy = get_pm2_process("orchestrator-proxy");

    ServicesStatus {
        ccr: ProcessStatus {
            name: "CCR".to_string(),
            running: ccr_running,
            pid: pm2_ccr.as_ref().and_then(|p| p.pid),
            uptime: None,
            cpu: pm2_ccr.as_ref().and_then(|p| p.cpu),
            memory: pm2_ccr.as_ref().and_then(|p| p.memory.clone()),
        },
        proxy: ProcessStatus {
            name: "Proxy".to_string(),
            running: proxy_running,
            pid: pm2_proxy.as_ref().and_then(|p| p.pid),
            uptime: None,
            cpu: pm2_proxy.as_ref().and_then(|p| p.cpu),
            memory: pm2_proxy.as_ref().and_then(|p| p.memory.clone()),
        },
        ccr_port_open: ccr_running,
        proxy_port_open: proxy_running,
        ollama_port_open: ollama_running,
        all_healthy: ccr_running && proxy_running,
    }
}

#[derive(Debug, Clone)]
struct Pm2ProcInfo {
    pid: Option<u32>,
    cpu: Option<f64>,
    memory: Option<String>,
}

fn get_pm2_process(name: &str) -> Option<Pm2ProcInfo> {
    let jlist = run_pm2(&["jlist"]).ok()?;
    let processes: Vec<Pm2Process> = serde_json::from_str(&jlist).ok()?;
    processes.iter().find(|p| p.name == name).map(|p| {
        Pm2ProcInfo {
            pid: p.pid,
            cpu: p.monit.as_ref().and_then(|m| m.cpu),
            memory: p.monit.as_ref().and_then(|m| m.memory).map(|mem| {
                format!("{:.1} MB", mem as f64 / 1024.0 / 1024.0)
            }),
        }
    })
}

fn ecosystem_path() -> String {
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    format!("{}\\.claude-code-router\\ecosystem.config.js", home)
}

pub fn start_services() -> Result<String, String> {
    let eco = ecosystem_path();
    if std::path::Path::new(&eco).exists() {
        run_pm2(&["start", &eco])?;
        Ok("CCR 和 Proxy 已启动".to_string())
    } else {
        // Fallback: try individual starts
        run_pm2(&["start", "ccr"])?;
        run_pm2(&["start", "orchestrator-proxy"])?;
        Ok("CCR 和 Proxy 已启动".to_string())
    }
}

pub fn stop_services() -> Result<String, String> {
    run_pm2(&["stop", "all"])?;
    Ok("所有服务已停止".to_string())
}
