// ============================================================
// 练习 E17: 退出拦截
// 目标: 拦截窗口关闭请求，经前端确认后再真正退出
// 知识点: CloseRequested / prevent_close / emit 通知前端
// ============================================================

use tauri::{Emitter, Manager};

/// 前端确认后调用：销毁主窗口，真正退出。
#[tauri::command]
fn confirm_close(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("main").ok_or("找不到主窗口")?;
    window.destroy().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_window_event(|window, event| {
            // 拦截关闭请求：先 prevent_close，再通知前端弹确认框
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.emit("close-requested", ());
            }
        })
        .invoke_handler(tauri::generate_handler![confirm_close])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}