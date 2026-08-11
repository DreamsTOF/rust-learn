// ============================================================
// 练习 E23: 后端监听
// 目标: 后端用 app.handle().listen 监听事件，解析 payload 并转发回窗口
// 知识点: Listener 监听 / event.payload 解析 / emit_to 转发
// ============================================================

use tauri::{Emitter, Listener};

/// 前端按 kind 选择事件名发送 ping（a → ping-a，b → ping-b）。
/// 后端监听器收到后会在终端打印，并向主窗口转发 pong 回应。
#[tauri::command]
fn emit_ping(app: tauri::AppHandle, kind: String, msg: String) -> Result<(), String> {
    match kind.as_str() {
        "a" => app.emit("ping-a", msg).map_err(|e| e.to_string()),
        "b" => app.emit("ping-b", msg).map_err(|e| e.to_string()),
        _ => Err("未知 kind".into()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 后端监听 ping-a：打印 payload，并转发 pong-a 回主窗口
            let handle = app.handle().clone();
            handle.clone().listen("ping-a", move |event| {
                let payload = event.payload().to_string();
                println!("[后端监听] ping-a: {payload}");
                let _ = handle.emit_to("main", "pong", format!("pong-a 回应: {payload}"));
            });

            // 后端监听 ping-b
            let handle2 = app.handle().clone();
            handle2.clone().listen("ping-b", move |event| {
                let payload = event.payload().to_string();
                println!("[后端监听] ping-b: {payload}");
                let _ = handle2.emit_to("main", "pong", format!("pong-b 回应: {payload}"));
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![emit_ping])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}