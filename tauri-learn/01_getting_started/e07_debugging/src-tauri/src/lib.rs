// ============================================================
// 练习 E07: 调试
// 目标: 掌握前端 DevTools（Web Inspector）与后端 println! 日志
// TODO: 按照注释提示补全代码
// ============================================================


// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 编写 run_debug_trace 命令 ————————————————————
// 演示后端日志：println! 输出到运行终端，结果回传前端
// TODO: 添加 #[tauri::command] 属性
fn run_debug_trace(message: String) -> Vec<String> {
    // TODO: 用 println! 打印一行日志（运行 cargo tauri dev 的终端可见）
    // 提示: println!("[debug] 收到前端消息: {message}")

    // TODO: 用 eprintln! 打印一行 stderr 日志（与 stdout 区分）
    // 提示: eprintln!("[debug] stderr 示例")

    // TODO: 计算时间戳并加入返回列表：
    //   std::time::SystemTime::now()
    //       .duration_since(std::time::UNIX_EPOCH)
    //       .map(|d| d.as_millis().to_string())
    //       .unwrap_or_else(|_| "unknown".into())

    vec![format!("收到消息: {message}")]
}

// === 步骤 2: 注册命令 ————————————————————————————————————
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 run_debug_trace 命令
        // 提示: .invoke_handler(tauri::generate_handler![run_debug_trace])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}