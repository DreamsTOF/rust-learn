// ============================================================
// 练习 E19: 后台任务
// 目标: 用 async_runtime::spawn / spawn_blocking 跑后台任务并回传结果
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 启动异步任务 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 spawn 调用：sleep 指定秒数后 emit "task-done" 完成事件
// 提示: tauri::async_runtime::spawn(async move {
//           tokio::time::sleep(Duration::from_secs(seconds)).await;
//           let _ = app.emit("task-done", format!("异步任务完成（{seconds}s）"));
//       });
//       需要 use std::time::Duration; 与 use tauri::Emitter;
fn start_async_task(_app: tauri::AppHandle, _seconds: u64) -> Result<(), String> {
    // TODO: 补全 tauri::async_runtime::spawn 调用
    Ok(())
}

// === 步骤 2: 启动阻塞任务 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 spawn_blocking 调用：计算 1..=n 的平方和后 emit "blocking-done"
// 提示: tauri::async_runtime::spawn_blocking(move || {
//           let sum: u64 = (1..=n).map(|i| i * i).sum();
//           let _ = app.emit("blocking-done", format!("阻塞任务完成，1..={n} 平方和 = {sum}"));
//       });
//       需要 use tauri::Emitter;
fn start_blocking_task(_app: tauri::AppHandle, _n: u64) -> Result<(), String> {
    // TODO: 补全 tauri::async_runtime::spawn_blocking 调用
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 3: 注册命令 ————————————————————————————————————
        // TODO: 注册 start_async_task 与 start_blocking_task
        // 提示: .invoke_handler(tauri::generate_handler![start_async_task, start_blocking_task])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}