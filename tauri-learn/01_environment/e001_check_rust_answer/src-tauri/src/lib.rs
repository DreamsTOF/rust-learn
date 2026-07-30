// ============================================================
// 练习 001: Rust 环境检查 (答案)
//
// 目标: 使用 std::process::Command 调用 rustc --version 检查 Rust 版本
// ============================================================
use std::process::Command;

#[tauri::command]
fn check_rust_version() -> String {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("无法执行 rustc 命令");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_rust_version])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
