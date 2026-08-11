// ============================================================
// 练习 E07: 调试
// 目标: 掌握前端 DevTools（Web Inspector）与后端 println! 日志
// 知识点: dev 模式 devtools / println! / eprintln! / console.log
// ============================================================

/// 演示后端日志：println! 输出到运行终端，结果回传前端。
/// 运行 `cargo tauri dev` 的终端中可看到 println! 输出。
#[tauri::command]
fn run_debug_trace(message: String) -> Vec<String> {
    println!("[debug] 收到前端消息: {message}");
    eprintln!("[debug] stderr 示例：错误日志走 eprintln!，与 stdout 区分");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis().to_string())
        .unwrap_or_else(|_| "unknown".into());

    vec![
        format!("收到消息: {message}"),
        format!("消息长度: {} 字符", message.chars().count()),
        format!("后端时间戳: {timestamp} ms"),
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_debug_trace])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}