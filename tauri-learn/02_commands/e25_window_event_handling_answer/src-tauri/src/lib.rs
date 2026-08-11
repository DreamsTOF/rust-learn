// ============================================================
// 练习 E25: 窗口事件处理
// 目标: 用 on_window_event 监听窗口事件并记录日志
// 知识点: on_window_event / Resized / Moved / Focused / 状态管理
// ============================================================

use std::sync::Mutex;

use tauri::{Manager, State};

/// 全局窗口事件日志（Mutex 保证跨线程安全）。
struct WindowLog(Mutex<Vec<String>>);

/// 返回全部日志，最新在前。
#[tauri::command]
fn get_window_log(state: State<WindowLog>) -> Result<Vec<String>, String> {
    let entries = state.0.lock().map_err(|e| e.to_string())?;
    let mut log = entries.clone();
    log.reverse();
    Ok(log)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(WindowLog(Mutex::new(Vec::new())))
        .on_window_event(|window, event| {
            // 记录窗口事件：调整尺寸 / 移动 / 焦点变化（lock 失败静默处理）
            let log = window.state::<WindowLog>();
            match event {
                tauri::WindowEvent::Resized(size) => {
                    if let Ok(mut entries) = log.0.lock() {
                        entries.push(format!("窗口调整尺寸: {size:?}"));
                    }
                }
                tauri::WindowEvent::Moved(pos) => {
                    if let Ok(mut entries) = log.0.lock() {
                        entries.push(format!("窗口移动: {pos:?}"));
                    }
                }
                tauri::WindowEvent::Focused(f) => {
                    if let Ok(mut entries) = log.0.lock() {
                        entries.push(format!("焦点变化: {f}"));
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![get_window_log])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}