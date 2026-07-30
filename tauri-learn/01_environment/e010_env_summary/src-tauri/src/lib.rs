// ============================================================
// 练习 010: 环境诊断工具 (练习版)
//
// 目标: 综合所有环境检测，返回完整诊断报告
// 难度: ⭐⭐⭐
//
// 说明:
//   定义 EnvSummary 结构体并实现 get_env_summary 命令。
//   综合前面练习的知识，完成完整的环境诊断。
//   - 包信息使用 env!() 宏
//   - 平台信息使用 std::env::consts 和 cfg!() 宏
//   - 版本信息使用 option_env!() 或 std::process::Command
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

#[tauri::command]
fn get_env_summary() -> EnvSummary {
    todo!("实现完整的环境诊断逻辑并返回 EnvSummary")

    // 提示：
    // - rust_version: 使用 option_env!("CARGO_PKG_RUST_VERSION") 或 "stable"
    // - node_version: 使用 std::process::Command::new("node").arg("--version")
    // - tauri_cli_version: 使用 std::process::Command::new("cargo").args(["tauri", "--version"])
    // - platform: std::env::consts::OS.to_string()
    // - arch: std::env::consts::ARCH.to_string()
    // - env_mode: if cfg!(debug_assertions) { "debug" } else { "release" }
    // - app_name: env!("CARGO_PKG_NAME").to_string()
    // - app_version: env!("CARGO_PKG_VERSION").to_string()
    //
    // 注意：std::process::Command 在运行时执行，可能失败
    // 可以用 .ok() 和 .and_then() 链式处理 Option
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // TODO: 在此注册命令
            // get_env_summary,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
