// ============================================================
// 练习 E41: React 集成
// 目标: 用 React 重构前端，通过 invoke 调用后端命令，并监听后端广播事件
// 知识点: useState + invoke / 事件监听 hook 化 / 组件卸载时 unlisten / 管理状态
// ============================================================

use std::sync::Mutex;
use tauri::{Emitter, State};

/// 计数器状态：.manage() 注入，命令用 State 访问
struct Counter(Mutex<u32>);

/// 计数 +1，返回新值，并广播 counter-changed 事件（前端监听同步）
#[tauri::command]
fn increment(app: tauri::AppHandle, state: State<Counter>) -> Result<u32, String> {
    let mut counter = state.0.lock().map_err(|e| e.to_string())?;
    *counter += 1;
    let _ = app.emit("counter-changed", *counter);
    Ok(*counter)
}

/// 计数置 0，返回新值，并广播 counter-changed 事件
#[tauri::command]
fn reset_counter(app: tauri::AppHandle, state: State<Counter>) -> Result<u32, String> {
    let mut counter = state.0.lock().map_err(|e| e.to_string())?;
    *counter = 0;
    let _ = app.emit("counter-changed", *counter);
    Ok(*counter)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Counter(Mutex::new(0)))
        .invoke_handler(tauri::generate_handler![increment, reset_counter])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}