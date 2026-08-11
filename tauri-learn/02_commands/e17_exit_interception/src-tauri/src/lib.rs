// ============================================================
// 练习 E17: 退出拦截
// 目标: 拦截窗口关闭请求，经前端确认后再真正退出
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 编写确认退出命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 通过 AppHandle 找到主窗口并销毁
// 提示: app.get_webview_window("main").ok_or("找不到主窗口")?
//           .destroy().map_err(|e| e.to_string())
//       （需要 use tauri::Manager;）
fn confirm_close(_app: tauri::AppHandle) -> Result<(), String> {
    // TODO: 补全销毁逻辑（当前直接返回）
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 2: 拦截窗口关闭事件 ————————————————————————————————————
        // TODO: 在 on_window_event 中匹配 CloseRequested：
        //   api.prevent_close() 阻止默认关闭
        //   window.emit("close-requested", ()) 通知前端弹确认框
        // 提示: if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        //         api.prevent_close();
        //         let _ = window.emit("close-requested", ());
        //       }
        //       （需要 use tauri::{Emitter, Manager};）
        .on_window_event(|window, event| {
            // TODO: 补全拦截逻辑（当前仅消费参数避免警告）
            let _ = (window, event);
        })
        // === 步骤 3: 注册命令 ————————————————————————————————————
        // TODO: 注册 confirm_close
        // 提示: .invoke_handler(tauri::generate_handler![confirm_close])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}