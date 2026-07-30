// ============================================================
// 练习 009: 跨平台开发注意事项 (答案版)
//
// 目标: 检测当前平台信息，展示 Tauri 跨平台支持
// 难度: ⭐⭐
// ============================================================

use serde::Serialize;

#[derive(Serialize)]
struct PlatformInfo {
    os: String,
    arch: String,
    family: String,
    is_windows: bool,
    is_macos: bool,
    is_linux: bool,
}

#[tauri::command]
fn get_platform_info() -> PlatformInfo {
    PlatformInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        family: std::env::consts::FAMILY.to_string(),
        is_windows: cfg!(target_os = "windows"),
        is_macos: cfg!(target_os = "macos"),
        is_linux: cfg!(target_os = "linux"),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_platform_info])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
