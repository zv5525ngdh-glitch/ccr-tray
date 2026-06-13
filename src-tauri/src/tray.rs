use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager};
use crate::process::ServicesStatus;
use crate::config::AppConfig;

// Embedded icon data
include!("../icons/icon_data.txt");

// Menu item IDs
pub const ID_START: &str = "start";
pub const ID_STOP: &str = "stop";
pub const ID_CONFIG: &str = "config";
pub const ID_AUTOSTART: &str = "autostart";
pub const ID_STATUS: &str = "status";

pub fn build_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let status = crate::process::get_services_status(3456, 3457, 11434);
    let config = crate::config::load_config();
    let menu = build_menu(app, &status, &config)?;

    // Use embedded raw RGBA icon data
    let icon = tauri::image::Image::new(ICON_RGBA, ICON_W, ICON_H);

    let _tray = TrayIconBuilder::with_id("main")
        .tooltip("CCR Tray")
        .icon(icon)
        .menu(&menu)
        .on_menu_event(move |app, event| handle_menu_event(app, event))
        .build(app)?;

    Ok(())
}

fn build_menu(
    app: &AppHandle,
    status: &ServicesStatus,
    config: &AppConfig,
) -> Result<tauri::menu::Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    let status_text = if status.all_healthy {
        "🟢 服务运行中"
    } else if status.ccr_port_open || status.proxy_port_open {
        "🟡 部分运行"
    } else {
        "🔴 服务已停止"
    };

    let autostart_text = if config.autostart {
        "🚀 开机自启 ✓"
    } else {
        "🚀 开机自启"
    };

    let menu = MenuBuilder::new(app)
        .item(&MenuItemBuilder::with_id(ID_STATUS, status_text).enabled(false).build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id(ID_START, "▶ 启动服务").enabled(!status.all_healthy).build(app)?)
        .item(&MenuItemBuilder::with_id(ID_STOP, "■ 停止服务").enabled(status.ccr_port_open || status.proxy_port_open).build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id(ID_CONFIG, "🔧 配置...").build(app)?)
        .separator()
        .item(&MenuItemBuilder::with_id(ID_AUTOSTART, autostart_text).build(app)?)
        .separator()
        .item(&PredefinedMenuItem::quit(app, Some("❌ 退出"))?)
        .build()?;

    Ok(menu)
}

fn handle_menu_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    let id = event.id().as_ref();
    match id {
        ID_START => {
            match crate::process::start_services() {
                Ok(msg) => {
                    println!("[ccr-tray] {}", msg);
                    refresh_tray(app);
                }
                Err(e) => {
                    eprintln!("[ccr-tray] 启动失败: {}", e);
                }
            }
        }
        ID_STOP => {
            match crate::process::stop_services() {
                Ok(msg) => {
                    println!("[ccr-tray] {}", msg);
                    refresh_tray(app);
                }
                Err(e) => {
                    eprintln!("[ccr-tray] 停止失败: {}", e);
                }
            }
        }
        ID_CONFIG => {
            show_config_window(app);
        }
        ID_AUTOSTART => {
            let mut config = crate::config::load_config();
            config.autostart = !config.autostart;
            let _ = crate::config::save_config(&config);
            let _ = crate::autostart::set_enabled(config.autostart);
            refresh_tray(app);
        }
        _ => {}
    }
}

fn show_config_window(app: &AppHandle) {
    use tauri::WebviewWindowBuilder;

    // Close existing config window if open
    if let Some(window) = app.get_webview_window("config") {
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    match WebviewWindowBuilder::new(
        app,
        "config",
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("CCR Tray - 配置")
    .inner_size(480.0, 420.0)
    .resizable(false)
    .maximizable(false)
    .minimizable(false)
    .build()
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("[ccr-tray] 无法打开配置窗口: {}", e);
        }
    }
}

pub fn refresh_tray(app: &AppHandle) {
    let status = crate::process::get_services_status(3456, 3457, 11434);
    let config = crate::config::load_config();

    if let Some(tray) = app.tray_by_id("main") {
        let Ok(menu) = build_menu(app, &status, &config) else {
            return;
        };
        let _ = tray.set_menu(Some(menu));
    }
}

// Periodic status refresh
pub fn start_status_refresh(app: AppHandle) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(30));
            // Must run on main thread
            let app_clone = app.clone();
            let _ = app.run_on_main_thread(move || {
                refresh_tray(&app_clone);
            });
        }
    });
}
