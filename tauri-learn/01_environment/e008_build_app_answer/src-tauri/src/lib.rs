// ============================================================
// 练习 008: 构建与产物分析 (答案版)
//
// 目标: 返回 Tauri 构建配置信息
// 难度: ⭐⭐
// ============================================================

use serde::Serialize;

#[derive(Serialize)]
struct BuildConfig {
    profile: String,
    target_os: String,
    target_arch: String,
    rust_version: String,
    cargo_pkg_name: String,
    cargo_pkg_version: String,
}

#[tauri::command]
fn get_build_config() -> BuildConfig {
    BuildConfig {
        profile: if cfg!(debug_assertions) {
            "debug".to_string()
        } else {
            "release".to_string()
        },
        target_os: std::env::consts::OS.to_string(),
        target_arch: std::env::consts::ARCH.to_string(),
        rust_version: option_env!("CARGO_PKG_RUST_VERSION")
            .unwrap_or("stable")
            .to_string(),
        cargo_pkg_name: env!("CARGO_PKG_NAME").to_string(),
        cargo_pkg_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_build_config])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
