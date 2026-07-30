// ============================================================
// 练习 008: 构建与产物分析 (练习版)
//
// 目标: 返回 Tauri 构建配置信息
// 难度: ⭐⭐
//
// 说明:
//   补全 BuildConfig 结构体的字段赋值，实现 get_build_config 命令。
//   使用 env!() 宏在编译期获取包信息。
// ============================================================

use serde::Serialize;

#[derive(Serialize)]
struct BuildConfig {
    profile: String,           // "debug" 或 "release"
    target_os: String,         // 目标操作系统
    target_arch: String,       // 目标架构
    rust_version: String,      // Rust 版本
    cargo_pkg_name: String,    // 包名（来自 Cargo.toml）
    cargo_pkg_version: String, // 包版本（来自 Cargo.toml）
}

#[tauri::command]
fn get_build_config() -> BuildConfig {
    // TODO: 补全 BuildConfig 各字段的赋值
    // 提示：
    // - profile: if cfg!(debug_assertions) { "debug" } else { "release" }
    // - target_os: std::env::consts::OS.to_string()
    // - target_arch: std::env::consts::ARCH.to_string()
    // - rust_version: "stable".to_string() (或使用 env!("CARGO_PKG_RUST_VERSION"))
    // - cargo_pkg_name: env!("CARGO_PKG_NAME").to_string()
    // - cargo_pkg_version: env!("CARGO_PKG_VERSION").to_string()
    todo!("补全 BuildConfig 各字段的赋值并返回结构体")

    // BuildConfig {
    //     profile: if cfg!(debug_assertions) { "debug".to_string() } else { "release".to_string() },
    //     target_os: std::env::consts::OS.to_string(),
    //     target_arch: std::env::consts::ARCH.to_string(),
    //     rust_version: option_env!("CARGO_PKG_RUST_VERSION").unwrap_or("stable").to_string(),
    //     cargo_pkg_name: env!("CARGO_PKG_NAME").to_string(),
    //     cargo_pkg_version: env!("CARGO_PKG_VERSION").to_string(),
    // }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // TODO: 在此注册命令
            // get_build_config,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
