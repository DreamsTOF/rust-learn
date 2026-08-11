// ============================================================
// 练习 E22: 窗口级事件
// 目标: emit_to 定向发送，区分窗口级监听与全局监听
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

use tauri::Manager;

// === 步骤 1: 打开聊天窗口 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 WebviewWindowBuilder 创建 chat 窗口（title / inner_size 420x300 / build）
// 提示: WebviewWindowBuilder::new(&app, "chat", WebviewUrl::App("index.html".into()))
//           .title("事件接收窗")
//           .inner_size(420.0, 300.0)
//           .build()
//           .map_err(|e| e.to_string())?;
//       需要 use tauri::{WebviewUrl, WebviewWindowBuilder};
fn open_chat_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("chat") {
        win.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    // TODO: 补全 builder 链创建 chat 窗口（当前直接返回 Ok 占位）
    Ok(())
}

// === 步骤 2: 定向发给主窗口 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 emit_to("main", "targeted-event", msg)
// 提示: app.emit_to("main", "targeted-event", msg).map_err(|e| e.to_string())
//       需要 use tauri::Emitter;
fn send_to_main(_app: tauri::AppHandle, _msg: String) -> Result<(), String> {
    // TODO: 补全 emit_to 调用（当前返回 Ok 占位）
    Ok(())
}

// === 步骤 3: 定向发给 chat 窗口 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 emit_to("chat", "targeted-event", msg)
// 提示: app.emit_to("chat", "targeted-event", msg).map_err(|e| e.to_string())
fn send_to_chat(_app: tauri::AppHandle, _msg: String) -> Result<(), String> {
    // TODO: 补全 emit_to 调用（当前返回 Ok 占位）
    Ok(())
}

// === 步骤 4: 全局广播 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 emit("broadcast-event", msg)（全局事件，所有窗口都收到）
// 提示: app.emit("broadcast-event", msg).map_err(|e| e.to_string())
fn send_to_all(_app: tauri::AppHandle, _msg: String) -> Result<(), String> {
    // TODO: 补全 emit 调用（当前返回 Ok 占位）
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 5: 注册命令 ————————————————————————————————————
        // TODO: 注册 open_chat_window / send_to_main / send_to_chat / send_to_all
        // 提示: .invoke_handler(tauri::generate_handler![open_chat_window, send_to_main, send_to_chat, send_to_all])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}