// ============================================================
// 练习 002: Node.js 环境检查 (答案)
//
// 目标: 使用 std::process::Command 调用 node --version 检查 Node.js 版本
// ============================================================
use std::process::Command;

#[tauri::command]
fn check_node_version() -> String {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .expect("无法执行 node 命令");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_node_version])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
