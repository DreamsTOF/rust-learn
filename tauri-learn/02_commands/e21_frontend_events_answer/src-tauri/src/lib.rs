// ============================================================
// 练习 E21: 前端事件
// 目标: 前端 listen / once / unlisten 配合类型化 payload 收发事件
// 知识点: emit 事件 / 类型化 payload / 一次性监听 / 取消监听
// ============================================================

use tauri::Emitter;

/// 事件负载：id + 消息 + 时间戳，前后端共用同一结构。
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct EventPayload {
    id: u32,
    message: String,
    ts: u64,
}

/// 发送普通事件 custom-event（前端可多次接收）。
#[tauri::command]
fn emit_custom_event(app: tauri::AppHandle, payload: EventPayload) -> Result<(), String> {
    app.emit("custom-event", payload).map_err(|e| e.to_string())
}

/// 发送一次性事件 one-time-event（前端用 once 只收一次）。
#[tauri::command]
fn emit_once_event(app: tauri::AppHandle, payload: EventPayload) -> Result<(), String> {
    app.emit("one-time-event", payload).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![emit_custom_event, emit_once_event])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}