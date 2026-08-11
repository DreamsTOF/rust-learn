// ============================================================
// 练习 E25: 窗口事件处理
// 目标: 用 on_window_event 监听窗口事件并记录日志
// 知识点: on_window_event / Resized / Moved / Focused / 状态管理
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

use tauri::Manager;

// === 步骤 1: 定义窗口日志状态 ————————————————————————————————————
// TODO: 补全 WindowLog 结构体：元组结构体，包装 Mutex<Vec<String>>
// 提示: struct WindowLog(Mutex<Vec<String>>);
//       需要 use std::sync::Mutex;
struct WindowLog(Vec<String>);

// === 步骤 2: 读取日志命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全命令体：lock 后克隆并反转（最新在前）
// 提示: let entries = state.0.lock().map_err(|e| e.to_string())?;
//       let mut log = entries.clone();
//       log.reverse();
//       Ok(log)
//       参数类型为 tauri::State<WindowLog>
fn get_window_log(_state: tauri::State<WindowLog>) -> Result<Vec<String>, String> {
    // TODO: 补全命令体（当前返回空列表占位）
    Ok(Vec::new())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 把 WindowLog 注册为全局状态（完成步骤 1 后与 Mutex 保持一致）
        // 提示: .manage(WindowLog(Mutex::new(Vec::new())))
        .manage(WindowLog(Vec::new()))
        .on_window_event(|window, event| {
            // === 步骤 3: 记录窗口事件 ————————————————————————————————————
            // TODO: 用 window.state::<WindowLog>() 取状态，match event 处理三个分支：
            //   Resized(size) → 追加 format!("窗口调整尺寸: {size:?}")
            //   Moved(pos)    → 追加 format!("窗口移动: {pos:?}")
            //   Focused(f)    → 追加 format!("焦点变化: {f}")
            // 提示: let log = window.state::<WindowLog>();
            //       match event {
            //         tauri::WindowEvent::Resized(size) => {
            //           if let Ok(mut entries) = log.0.lock() {
            //             entries.push(format!("窗口调整尺寸: {size:?}"));
            //           }
            //         }
            //         tauri::WindowEvent::Moved(pos) => { ... }
            //         tauri::WindowEvent::Focused(f) => { ... }
            //         _ => {}
            //       }
            //       lock 失败静默处理，闭包返回 ()
            //       需要 use tauri::Manager（window.state 是 Manager 提供的方法）
            let _log = window.state::<WindowLog>();
            let _ = event;
        })
        // === 步骤 4: 注册命令 ————————————————————————————————————
        // TODO: 注册 get_window_log
        // 提示: .invoke_handler(tauri::generate_handler![get_window_log])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}