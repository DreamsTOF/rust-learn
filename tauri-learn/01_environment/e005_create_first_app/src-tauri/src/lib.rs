// ============================================================
// 练习 005: 创建第一个 Tauri 应用
//
// 目标: 编写一个返回应用元数据的命令
// 难度: ⭐⭐⭐
// ============================================================

// TODO: 导入 serde::Serialize
// use serde::Serialize;

// TODO: 定义 AppMetadata 结构体，包含以下字段:
// - name: String
// - version: String
// - tauri_version: String
// - os: String
// 提示: 需要派生 Serialize trait
struct AppMetadata;

#[tauri::command]
fn get_app_metadata() -> AppMetadata {
    // TODO: 返回 AppMetadata 实例
    // name 从 env!("CARGO_PKG_NAME") 获取
    // version 从 env!("CARGO_PKG_VERSION") 获取
   // tauri_version 从 tauri::VERSION 获取
    // os 从 std::env::consts::OS.to_string() 获取
    todo!("实现 get_app_metadata 函数 - 返回应用元数据")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // TODO: 在此注册命令
            // get_app_metadata,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
