// ============================================================
// 练习 E22: 窗口级事件
// 目标: emit_to 定向发送，区分窗口级监听与全局监听
// 知识点: emit_to 定向 / emit 全局广播 / 窗口级 listen / 多窗口
// ============================================================

use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

/// 打开「事件接收窗」chat 窗口（与主窗口共用 index.html）。
/// 已存在时直接聚焦，避免重复创建。
#[tauri::command]
fn open_chat_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("chat") {
        win.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    WebviewWindowBuilder::new(&app, "chat", WebviewUrl::App("index.html".into()))
        .title("事件接收窗")
        .inner_size(420.0, 300.0)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 定向发送给主窗口：只有主窗口的窗口级监听能收到 targeted-event。
#[tauri::command]
fn send_to_main(app: tauri::AppHandle, msg: String) -> Result<(), String> {
    app.emit_to("main", "targeted-event", msg).map_err(|e| e.to_string())
}

/// 定向发送给 chat 窗口。
#[tauri::command]
fn send_to_chat(app: tauri::AppHandle, msg: String) -> Result<(), String> {
    app.emit_to("chat", "targeted-event", msg).map_err(|e| e.to_string())
}

/// 全局广播：两个窗口的全局监听都能收到 broadcast-event。
#[tauri::command]
fn send_to_all(app: tauri::AppHandle, msg: String) -> Result<(), String> {
    app.emit("broadcast-event", msg).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_chat_window,
            send_to_main,
            send_to_chat,
            send_to_all
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}