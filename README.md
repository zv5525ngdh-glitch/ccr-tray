# CCR Tray

[English](#english) | [中文](#中文)

---

## English

### What is CCR Tray?

CCR Tray is a lightweight **Windows system tray application** that manages the lifecycle of [Claude Code Router (CCR)](https://github.com/musistudio/claude-code-router) and its orchestrator proxy. Instead of manually starting/stoping processes in terminal windows, you get a simple tray icon with right-click controls.

### Why?

- **CCR process instability**: CCR would die when its terminal window was closed. CCR Tray runs it via PM2 with auto-restart.
- **No visual management**: Previously required PowerShell commands to check status, restart, or stop services.
- **Auto-start reliability**: Replaced fragile `startup.bat` + registry hacks with a proper Windows startup integration.

### Features

| Feature | Description |
|---------|-------------|
| 🟢 System Tray | Red icon with status indicator, right-click menu |
| ▶ Start / ■ Stop | One-click start/stop CCR + Proxy via PM2 |
| 🔧 Config Panel | GUI for port settings, model selection |
| 🚀 Auto-start | Optional Windows startup with registry toggle |
| 🔄 Status Refresh | Polls process health every 30 seconds |
| 🛡️ Non-invasive | Does NOT touch `settings.local.json`. Works alongside CC Switch. |

### Architecture

```
┌─────────────────────────────────┐
│        CCR Tray (System Tray)    │
│  🟢 Services Running             │
│  ├─ ▶ Start Services             │
│  ├─ ■ Stop Services              │
│  ├─ ──────────                   │
│  ├─ 🔧 Config...                 │
│  ├─ 🚀 Auto-start ✓              │
│  └─ ❌ Quit                      │
└─────────────────────────────────┘

CC Switch (Routing)        CCR Tray (Process Lifecycle)
     │                           │
     ├─ ON: settings.local       ├─ ON: pm2 start ecosystem
     │  → 127.0.0.1:3457        │  → CCR:3456 + Proxy:3457
     │                           │
     ├─ OFF: direct DeepSeek     ├─ OFF: pm2 stop all
     │                           │
     ▼                           ▼
   Independent, no interference
```

### How It Works

1. **Startup**: CCR Tray launches → spawns CCR + Proxy via PM2 ecosystem file
2. **Tray Menu**: Right-click the icon to start/stop services, open config, toggle auto-start
3. **PM2 Guardian**: Both processes are managed by PM2 with auto-restart on crash
4. **Status Polling**: Every 30s, checks ports 3456/3457 and updates the tray menu

### Requirements

- Windows 10/11
- Node.js (for PM2 and CCR)
- PM2 installed globally (`npm i -g pm2`)
- CCR installed (`npm i -g @musistudio/claude-code-router`)

### Install

Download the latest installer from [Releases](https://github.com/zv5525ngdh-glitch/ccr-tray/releases) and run it.

Or build from source:

```powershell
# Prerequisites: Rust, Node.js, Visual Studio Build Tools
git clone https://github.com/zv5525ngdh-glitch/ccr-tray.git
cd ccr-tray
npm install
npx tauri build
# Installer at: src-tauri/target/release/bundle/nsis/CCR Tray_*_x64-setup.exe
```

### Usage

1. Launch CCR Tray (auto-starts on login if enabled)
2. Right-click the tray icon (red circle with "C")
3. **Start Services** — launches CCR (port 3456) + Proxy (port 3457)
4. **Stop Services** — stops all managed processes
5. **Config** — opens settings panel (ports, model, temperature)
6. **Auto-start** — toggle Windows startup behavior

### Tech Stack

- **Backend**: Rust + Tauri v2
- **Frontend**: Vanilla HTML/CSS/JS + Vite
- **Process Manager**: PM2
- **Packaging**: Tauri NSIS bundler

---

## 中文

### 这是什么？

CCR Tray 是一个轻量级的 **Windows 系统托盘应用**，用于管理 [Claude Code Router (CCR)](https://github.com/musistudio/claude-code-router) 和编排代理 (Orchestrator Proxy) 的进程生命周期。不需要开终端窗口，托盘右键即可操作。

### 解决了什么问题？

- **CCR 进程不稳定**：终端窗口关了 CCR 就死。CCR Tray 通过 PM2 管理，崩溃自动重启。
- **没有可视化管理**：以前要用 PowerShell 命令查看状态、重启、停止服务。
- **开机自启不靠谱**：用 Tauri 应用替换了脆弱的 `startup.bat` + 注册表方案。

### 功能

| 功能 | 说明 |
|------|------|
| 🟢 系统托盘 | 红色图标 + 状态指示，右键菜单操作 |
| ▶ 启动 / ■ 停止 | 一键启停 CCR + Proxy |
| 🔧 配置面板 | 可视化设置端口、模型、温度等 |
| 🚀 开机自启 | 可选，通过注册表实现 |
| 🔄 状态刷新 | 每 30 秒检测进程健康状态 |
| 🛡️ 互不干扰 | 不碰 `settings.local.json`，与 CC Switch 共存 |

### 职责边界

| 组件 | 职责 |
|------|------|
| **CC Switch** | 路由切换（改写 `settings.local.json`） |
| **CCR Tray** | 进程管理（CCR + Proxy 启停） |
| **PM2** | 进程守护（崩溃自动重启） |

两者各管各的，互不影响。

### 环境要求

- Windows 10/11
- Node.js（PM2 和 CCR 依赖）
- 全局安装 PM2：`npm i -g pm2`
- 全局安装 CCR：`npm i -g @musistudio/claude-code-router`

### 安装

从 [Releases](https://github.com/zv5525ngdh-glitch/ccr-tray/releases) 下载最新安装包运行即可。

或从源码构建：

```powershell
# 前置：Rust, Node.js, Visual Studio Build Tools
git clone https://github.com/zv5525ngdh-glitch/ccr-tray.git
cd ccr-tray
npm install
npx tauri build
# 安装包位置：src-tauri/target/release/bundle/nsis/CCR Tray_*_x64-setup.exe
```

### 使用

1. 启动 CCR Tray（如开启开机自启则自动启动）
2. 右键托盘图标（红色圆圈内有"C"）
3. **启动服务** — 拉起 CCR (3456) + Proxy (3457)
4. **停止服务** — 停止所有托管进程
5. **配置** — 打开设置面板（端口、模型等）
6. **开机自启** — 切换是否开机自动启动

### 技术栈

- **后端**：Rust + Tauri v2
- **前端**：原生 HTML/CSS/JS + Vite
- **进程管理**：PM2
- **打包**：Tauri NSIS 安装包

---

## License

MIT
