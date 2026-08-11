// ============================================================
// 练习 E10: 依赖注入
// 目标: 在命令中注入 AppHandle / WebviewWindow / State<T> 多个依赖
// 知识点: AppHandle / WebviewWindow / State<T> 组合注入
// ============================================================

use std::sync::Mutex;
use tauri::{Manager, State};

/// 由 Builder.manage 注入的共享计数器。
struct Counter(Mutex<i32>);

/// inspect 命令的返回值：注入多个依赖后组装的信息。
#[derive(serde::Serialize)]
struct InspectInfo {
    window_title: String,
    window_label: String,
    window_count: usize,
    counter: i32,
    app_name: String,
}

/// 同时注入 AppHandle、WebviewWindow、State<Counter> 三个依赖。
#[tauri::command]
fn inspect(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    state: State<Counter>,
) -> Result<InspectInfo, String> {
    let counter = state.0.lock().map_err(|e| e.to_string())?;
    Ok(InspectInfo {
        window_title: window.title().map_err(|e| e.to_string())?,
        window_label: window.label().into(),
        window_count: app.webview_windows().len(),
        counter: *counter,
        app_name: app.package_info().name.clone(),
    })
}

/// 计数 +1 并返回新值。
#[tauri::command]
fn increment(state: State<Counter>) -> Result<i32, String> {
    let mut counter = state.0.lock().map_err(|e| e.to_string())?;
    *counter += 1;
    Ok(*counter)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Counter(Mutex::new(0)))
        .invoke_handler(tauri::generate_handler![inspect, increment])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}