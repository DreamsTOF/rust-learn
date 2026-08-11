// ============================================================
// 练习 E09: 异步命令
// 目标: 编写 async 命令，演示延迟执行与超时控制
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 编写异步命令 slow_echo ————————————————————————————————
// TODO: 添加 #[tauri::command] 属性，并给函数加上 async 关键字
// TODO: 补全 sleep 等待：
//   tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
// 提示: 完整签名:
//   async fn slow_echo(message: String, delay_ms: u64) -> Result<String, String>
//   返回: Ok(format!("延迟 {delay_ms}ms 后回显: {message}"))
fn slow_echo(message: String, delay_ms: u64) -> Result<String, String> {
    // TODO: 在返回前 sleep delay_ms 毫秒（当前为同步占位，完成填空后改为 async fn）
    Ok(format!("延迟 {delay_ms}ms 后回显: {message}"))
}

// === 步骤 2: 编写超时命令 run_with_timeout ————————————————————————————
// TODO: 添加 #[tauri::command] 属性与 async 关键字
// TODO: 用 tokio::time::timeout 包裹 3 秒模拟任务，超时返回 Err("操作超时")
// 提示: let task = async move {
//         tokio::time::sleep(std::time::Duration::from_secs(3)).await;
//         format!("3 秒耗时任务完成: {message}")
//       };
//       match tokio::time::timeout(std::time::Duration::from_millis(timeout_ms), task).await {
//           Ok(text) => Ok(text),
//           Err(_) => Err("操作超时".into()),
//       }
fn run_with_timeout(message: String, timeout_ms: u64) -> Result<String, String> {
    // TODO: 补全超时包裹逻辑（当前直接返回占位）
    let _ = (message, timeout_ms);
    Ok("占位: 未实现超时逻辑".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 3: 注册命令 ————————————————————————————————————
        // TODO: 注册 slow_echo 与 run_with_timeout
        // 提示: .invoke_handler(tauri::generate_handler![slow_echo, run_with_timeout])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}