// ============================================================
// 练习 E09: 异步命令
// 目标: 编写 async 命令，演示延迟执行与超时控制
// 知识点: async fn 命令 / tokio::time::sleep / tokio::time::timeout
// ============================================================

use std::time::Duration;

/// 异步命令：等待 delay_ms 毫秒后返回回显文本。
/// Tauri 会把 async 命令调度到异步运行时执行，不阻塞主线程。
#[tauri::command]
async fn slow_echo(message: String, delay_ms: u64) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    Ok(format!("延迟 {delay_ms}ms 后回显: {message}"))
}

/// 超时演示：模拟一个 3 秒的耗时任务，并用 tokio::time::timeout 包裹。
/// 超过 timeout_ms 毫秒未完成时返回 Err("操作超时")。
#[tauri::command]
async fn run_with_timeout(message: String, timeout_ms: u64) -> Result<String, String> {
    let task = async move {
        tokio::time::sleep(Duration::from_secs(3)).await;
        format!("3 秒耗时任务完成: {message}")
    };
    match tokio::time::timeout(Duration::from_millis(timeout_ms), task).await {
        Ok(text) => Ok(text),
        Err(_) => Err("操作超时".into()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![slow_echo, run_with_timeout])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}