// ============================================================
// 练习 005: 创建第一个 Tauri 应用 (答案)
//
// 目标: 编写一个返回应用元数据的命令
// ============================================================
use serde::Serialize;

#[derive(Serialize)]
struct AppMetadata {
    name: String,
    version: String,
    tauri_version: String,
    os: String,
}

#[tauri::command]
fn get_app_metadata() -> AppMetadata {
    AppMetadata {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
       tauri_version: tauri::VERSION.to_string(),
        os: std::env::consts::OS.to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_app_metadata])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
