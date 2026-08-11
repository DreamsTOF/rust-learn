// ============================================================
// 练习 E16: setup 钩子
// 目标: 在 setup 阶段初始化状态、执行异步任务与主线程回调
// 知识点: Builder::setup / async_runtime::spawn / emit / run_on_main_thread
// ============================================================

use std::sync::Mutex;
use tauri::{Emitter, Manager, State};

/// 初始化状态：setup 中写入，命令中读取。
struct SetupState(Mutex<String>);

/// 读取当前初始化状态。
#[tauri::command]
fn get_setup_state(state: State<SetupState>) -> Result<String, String> {
    let s = state.0.lock().map_err(|e| e.to_string())?;
    Ok(s.clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            println!("[setup] 应用启动，窗口已创建");

            // 1. 注入共享状态
            app.manage(SetupState(Mutex::new("未初始化".into())));

            // 2. 异步任务：1 秒后向前端广播初始化完成事件
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                let _ = handle.emit("init-done", "初始化完成");
            });

            // 3. 主线程回调（在窗口事件循环的主线程上执行）
            let _ = app.run_on_main_thread(|| {
                println!("[setup] run_on_main_thread 回调（主线程）");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_setup_state])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}