// ============================================================
// 练习 E18: 路径 API
// 目标: 用 PathResolver 获取各系统目录并展示
// 知识点: app.path() / app_data_dir 等目录解析 / 路径拼接
// ============================================================

use tauri::Manager;

/// 一条路径信息：名称 + 解析结果。
#[derive(serde::Serialize)]
struct PathItem {
    name: String,
    path: String,
}

/// 列出 Tauri 提供的常用目录。
#[tauri::command]
fn list_paths(app: tauri::AppHandle) -> Result<Vec<PathItem>, String> {
    let p = app.path();
    let mut items = Vec::new();
    for (name, res) in [
        ("app_data_dir", p.app_data_dir()),
        ("app_config_dir", p.app_config_dir()),
        ("app_log_dir", p.app_log_dir()),
        ("app_cache_dir", p.app_cache_dir()),
        ("resource_dir", p.resource_dir()),
        ("temp_dir", p.temp_dir()),
    ] {
        items.push(PathItem {
            name: name.into(),
            path: res
                .map(|x| x.display().to_string())
                .unwrap_or_else(|e| format!("错误: {e}")),
        });
    }
    Ok(items)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_paths])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}