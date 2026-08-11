// ============================================================
// 练习 E19: 后台任务
// 目标: 用 async_runtime::spawn / spawn_blocking 跑后台任务并回传结果
// 知识点: spawn 异步任务 / spawn_blocking 阻塞任务 / 事件回传
// ============================================================

use std::time::Duration;
use tauri::Emitter;

/// 启动异步任务：等待指定秒数后向前端广播完成事件。
/// spawn 的任务跑在异步运行时上，不阻塞 UI，适合 IO 等待类工作。
#[tauri::command]
fn start_async_task(app: tauri::AppHandle, seconds: u64) -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_secs(seconds)).await;
        let _ = app.emit("task-done", format!("异步任务完成（{seconds}s）"));
    });
    Ok(())
}

/// 启动阻塞任务：在独立线程池上计算 1..=n 的平方和。
/// spawn_blocking 适合 CPU 密集计算，避免占用异步运行时线程。
#[tauri::command]
fn start_blocking_task(app: tauri::AppHandle, n: u64) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let sum: u64 = (1..=n).map(|i| i * i).sum();
        let _ = app.emit("blocking-done", format!("阻塞任务完成，1..={n} 平方和 = {sum}"));
    });
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_async_task, start_blocking_task])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}