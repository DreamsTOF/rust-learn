// ============================================================
// 练习 004: 安装 Tauri CLI (答案)
//
// 目标: 检查 Tauri CLI 是否已安装
// ============================================================
use std::process::Command;

#[tauri::command]
fn check_tauri_cli() -> String {
    match Command::new("cargo").args(["tauri", "--version"]).output() {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "未检测到 Tauri CLI，请运行: cargo install tauri-cli".to_string()
            }
        }
        Err(_) => "未检测到 Tauri CLI，请运行: cargo install tauri-cli".to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_tauri_cli])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
