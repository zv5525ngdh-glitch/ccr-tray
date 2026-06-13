import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

// Types
interface AppConfig {
  ccr_port: number;
  proxy_port: number;
  ollama_port: number;
  ollama_model: string;
  temperature: number;
  autostart: boolean;
}

interface ProcessStatus {
  name: string;
  running: boolean;
  pid: number | null;
  uptime: string | null;
  cpu: number | null;
  memory: string | null;
}

interface ServicesStatus {
  ccr: ProcessStatus;
  proxy: ProcessStatus;
  ccr_port_open: boolean;
  proxy_port_open: boolean;
  ollama_port_open: boolean;
  all_healthy: boolean;
}

// DOM elements
const statusIndicator = document.getElementById("status-indicator")!;
const ccrStatus = document.getElementById("ccr-status")!;
const proxyStatus = document.getElementById("proxy-status")!;
const ollamaStatus = document.getElementById("ollama-status")!;

const inputCcrPort = document.getElementById("ccr_port") as HTMLInputElement;
const inputProxyPort = document.getElementById("proxy_port") as HTMLInputElement;
const inputOllamaPort = document.getElementById("ollama_port") as HTMLInputElement;
const inputOllamaModel = document.getElementById("ollama_model") as HTMLInputElement;
const inputTemperature = document.getElementById("temperature") as HTMLInputElement;

// Load config and status on open
async function init() {
  try {
    const config = await invoke<AppConfig>("get_config");
    inputCcrPort.value = String(config.ccr_port);
    inputProxyPort.value = String(config.proxy_port);
    inputOllamaPort.value = String(config.ollama_port);
    inputOllamaModel.value = config.ollama_model;
    inputTemperature.value = String(config.temperature);
  } catch (e) {
    console.error("加载配置失败:", e);
  }
  await refreshStatus();
}

async function refreshStatus() {
  try {
    const status = await invoke<ServicesStatus>("get_status");
    updateStatusUI(status);
  } catch (e) {
    console.error("获取状态失败:", e);
  }
}

function updateStatusUI(s: ServicesStatus) {
  if (s.all_healthy) {
    statusIndicator.textContent = "🟢 全部运行中";
  } else if (s.ccr_port_open || s.proxy_port_open) {
    statusIndicator.textContent = "🟡 部分运行";
  } else {
    statusIndicator.textContent = "🔴 已停止";
  }

  ccrStatus.textContent = s.ccr_port_open ? `运行中 (端口 OK)` : "已停止";
  ccrStatus.style.color = s.ccr_port_open ? "#2ecc71" : "#e74c3c";

  proxyStatus.textContent = s.proxy_port_open ? `运行中 (端口 OK)` : "已停止";
  proxyStatus.style.color = s.proxy_port_open ? "#2ecc71" : "#e74c3c";

  ollamaStatus.textContent = s.ollama_port_open ? "运行中" : "未运行";
  ollamaStatus.style.color = s.ollama_port_open ? "#2ecc71" : "#e74c3c";
}

// Button handlers
document.getElementById("btn-save")!.addEventListener("click", async () => {
  try {
    const config: AppConfig = {
      ccr_port: parseInt(inputCcrPort.value) || 3456,
      proxy_port: parseInt(inputProxyPort.value) || 3457,
      ollama_port: parseInt(inputOllamaPort.value) || 11434,
      ollama_model: inputOllamaModel.value || "qwen3-vl:8b",
      temperature: parseFloat(inputTemperature.value) || 0.7,
      autostart: true,
    };
    await invoke("save_app_config", { config });
    alert("配置已保存！");
  } catch (e) {
    alert("保存失败: " + e);
  }
});

document.getElementById("btn-start")!.addEventListener("click", async () => {
  try {
    const msg = await invoke<string>("start_all");
    console.log(msg);
    await refreshStatus();
  } catch (e) {
    alert("启动失败: " + e);
  }
});

document.getElementById("btn-stop")!.addEventListener("click", async () => {
  try {
    const msg = await invoke<string>("stop_all");
    console.log(msg);
    await refreshStatus();
  } catch (e) {
    alert("停止失败: " + e);
  }
});

document.getElementById("btn-refresh")!.addEventListener("click", refreshStatus);

document.getElementById("btn-close")!.addEventListener("click", async () => {
  await getCurrentWindow().hide();
});

// Init on load
init();
