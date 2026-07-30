// ============================================================
// 练习 010: 环境诊断工具 (答案版)
//
// 目标: 综合所有环境检测，返回完整诊断报告
// 难度: ⭐⭐⭐
// ============================================================

use serde::Serialize;

#[derive(Serialize)]
struct EnvSummary {
    rust_version: String,
    node_version: String,
    tauri_cli_version: String,
    platform: String,
    arch: String,
    env_mode: String,
    app_name: String,
    app_version: String,
}

/// 获取 Node.js 版本（通过运行 node --version）
fn get_node_version() -> String {
    std::process::Command::new("node")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "未检测到".to_string())
}

/// 获取 Tauri CLI 版本（通过运行 cargo tauri --version）
fn get_tauri_cli_version() -> String {
    std::process::Command::new("cargo")
        .args(["tauri", "--version"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "未检测到".to_string())
}

#[tauri::command]
fn get_env_summary() -> EnvSummary {
    EnvSummary {
        rust_version: option_env!("CARGO_PKG_RUST_VERSION")
            .unwrap_or("stable")
            .to_string(),
        node_version: get_node_version(),
        tauri_cli_version: get_tauri_cli_version(),
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        env_mode: if cfg!(debug_assertions) {
            "debug".to_string()
        } else {
            "release".to_string()
        },
        app_name: env!("CARGO_PKG_NAME").to_string(),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_env_summary])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
