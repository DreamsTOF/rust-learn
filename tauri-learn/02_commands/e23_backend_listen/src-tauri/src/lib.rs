// ============================================================
// 练习 E23: 后端监听
// 目标: 后端用 app.handle().listen 监听事件，解析 payload 并转发回窗口
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 编写 ping 发送命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 按 kind 分发：a → emit("ping-a", msg)，b → emit("ping-b", msg)，其他 Err
// 提示: match kind.as_str() {
//           "a" => app.emit("ping-a", msg).map_err(|e| e.to_string()),
//           "b" => app.emit("ping-b", msg).map_err(|e| e.to_string()),
//           _ => Err("未知 kind".into()),
//       }
//       需要 use tauri::Emitter;
fn emit_ping(_app: tauri::AppHandle, _kind: String, _msg: String) -> Result<(), String> {
    // TODO: 补全 match 分发（当前返回 Ok 占位）
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            // === 步骤 2: 后端监听 ping-a ————————————————————————————————————
            // TODO: clone handle → listen("ping-a") → 解析 payload 打印 → emit_to 主窗口 pong
            // 提示: let handle = _app.handle().clone();
            //       handle.clone().listen("ping-a", move |event| {
            //           let payload = event.payload().to_string();
            //           println!("[后端监听] ping-a: {payload}");
            //           let _ = handle.emit_to("main", "pong", format!("pong-a 回应: {payload}"));
            //       });
            //       需要 use tauri::{Emitter, Listener};

            // === 步骤 3: 后端监听 ping-b ————————————————————————————————————
            // TODO: 同上，监听 "ping-b"，回应前缀改为 pong-b
            // 提示: 参照步骤 2，事件名 "ping-b"，println 文案与回应前缀相应修改

            Ok(())
        })
        // === 步骤 4: 注册命令 ————————————————————————————————————
        // TODO: 注册 emit_ping
        // 提示: .invoke_handler(tauri::generate_handler![emit_ping])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}