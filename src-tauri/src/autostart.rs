use winreg::enums::*;
use winreg::RegKey;

const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const VALUE_NAME: &str = "CCRTray";

/// Check if auto-start is enabled in registry
pub fn is_enabled() -> bool {
    get_current_value().is_some()
}

/// Enable or disable auto-start
pub fn set_enabled(enabled: bool) -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(RUN_KEY, KEY_SET_VALUE | KEY_QUERY_VALUE)
        .map_err(|e| format!("无法打开注册表: {}", e))?;

    if enabled {
        let exe_path = std::env::current_exe()
            .map_err(|e| format!("无法获取程序路径: {}", e))?;
        let path_str = exe_path.to_string_lossy().to_string();
        run_key.set_value(VALUE_NAME, &path_str)
            .map_err(|e| format!("写入注册表失败: {}", e))?;
    } else {
        // Delete the value if it exists
        match run_key.delete_value(VALUE_NAME) {
            Ok(_) => {},
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {},
            Err(e) => return Err(format!("删除注册表项失败: {}", e)),
        }
    }
    Ok(())
}

/// Get current auto-start executable path
fn get_current_value() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(RUN_KEY, KEY_READ).ok()?;
    let value: String = run_key.get_value(VALUE_NAME).ok()?;
    Some(value)
}
