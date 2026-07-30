// ============================================================
// 练习 009: 跨平台开发注意事项 (练习版)
//
// 目标: 检测当前平台信息，展示 Tauri 跨平台支持
// 难度: ⭐⭐
//
// 说明:
//   补全 PlatformInfo 结构体的字段赋值，实现 get_platform_info 命令。
//   使用 cfg!() 宏在编译期检测当前平台。
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
    // TODO: 补全 PlatformInfo 各字段的赋值
    // 提示：
    // - os: std::env::consts::OS.to_string()
    // - arch: std::env::consts::ARCH.to_string()
    // - family: std::env::consts::FAMILY.to_string()
    // - is_windows: cfg!(target_os = "windows")
    // - is_macos: cfg!(target_os = "macos")
    // - is_linux: cfg!(target_os = "linux")
    todo!("补全 PlatformInfo 各字段的赋值并返回结构体")

    // PlatformInfo {
    //     os: std::env::consts::OS.to_string(),
    //     arch: std::env::consts::ARCH.to_string(),
    //     family: std::env::consts::FAMILY.to_string(),
    //     is_windows: cfg!(target_os = "windows"),
    //     is_macos: cfg!(target_os = "macos"),
    //     is_linux: cfg!(target_os = "linux"),
    // }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // TODO: 在此注册命令
            // get_platform_info,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
