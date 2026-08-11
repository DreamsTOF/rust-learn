// ============================================================
// 练习 E29: 窗口状态持久化（window-state 插件）
// 目标: 用 tauri-plugin-window-state 保存/恢复窗口位置、大小、最大化状态
// 知识点: window-state 插件 / StateFlags / save_window_state / 状态文件
// ============================================================

use tauri::Manager;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

/// 把所有窗口的当前状态（位置/大小/最大化等）写入磁盘。
#[tauri::command]
fn save_window_state(app: tauri::AppHandle) -> Result<String, String> {
    app.save_window_state(StateFlags::all())
        .map_err(|e| e.to_string())?;
    Ok("窗口状态已保存".into())
}

/// 删除磁盘上的窗口状态文件，之后启动不再恢复旧状态。
///
/// 注意: window-state 插件没有暴露 Rust 侧的清除 API，
/// 这里手动删除 `AppHandleExt::filename()` 返回的状态文件
/// （位于 app_config_dir 下），与前端 JS 的 clearWindowState() 等价。
#[tauri::command]
fn clear_window_state(app: tauri::AppHandle) -> Result<String, String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let state_file = dir.join(app.filename());
    if state_file.exists() {
        std::fs::remove_file(state_file).map_err(|e| e.to_string())?;
    }
    Ok("已清除保存的状态".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 插件会在窗口创建时自动恢复状态、退出时自动保存状态
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            save_window_state,
            clear_window_state
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}