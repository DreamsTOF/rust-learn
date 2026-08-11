// ============================================================
// 练习 E21: 前端事件
// 目标: 前端 listen / once / unlisten 配合类型化 payload 收发事件
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 定义事件负载结构体 ————————————————————————————————————
// TODO: 定义 EventPayload 并派生 Clone + serde::Serialize + serde::Deserialize
// 提示: #[derive(Clone, serde::Serialize, serde::Deserialize)]
//       struct EventPayload { id: u32, message: String, ts: u64 }
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct EventPayload {
    // TODO: 补全字段（id: u32 / message: String / ts: u64）
}

// === 步骤 2: 发送普通事件 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全函数体：emit("custom-event", payload)
// 提示: app.emit("custom-event", payload).map_err(|e| e.to_string())
//       需要 use tauri::Emitter;
fn emit_custom_event(_app: tauri::AppHandle, _payload: EventPayload) -> Result<(), String> {
    // TODO: 补全 emit 调用（当前返回 Ok 占位）
    Ok(())
}

// === 步骤 3: 发送一次性事件 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全函数体：emit("one-time-event", payload)
// 提示: app.emit("one-time-event", payload).map_err(|e| e.to_string())
fn emit_once_event(_app: tauri::AppHandle, _payload: EventPayload) -> Result<(), String> {
    // TODO: 补全 emit 调用（当前返回 Ok 占位）
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 4: 注册命令 ————————————————————————————————————
        // TODO: 注册 emit_custom_event 与 emit_once_event
        // 提示: .invoke_handler(tauri::generate_handler![emit_custom_event, emit_once_event])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}