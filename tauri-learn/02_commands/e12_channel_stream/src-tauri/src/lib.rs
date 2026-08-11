// ============================================================
// 练习 E12: Channel 流式传输
// 目标: 用 Channel 把后端数据流式推送给前端
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

use tauri::ipc::Channel;

/// 流式消息结构：step 步号 + label 文本。
#[derive(Clone, serde::Serialize)]
struct StreamItem {
    step: u32,
    label: String,
}

// === 步骤 1: 编写进度推送命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 Channel 参数签名（去掉下划线）：channel: Channel<u32>
// TODO: 0..=10 循环推送 i * 10，每条间隔 100ms
// 提示: channel.send(i * 10).map_err(|e| e.to_string())?;
//       std::thread::sleep(std::time::Duration::from_millis(100));
fn start_progress(_channel: Channel<u32>) -> Result<(), String> {
    // TODO: 补全循环推送逻辑（当前直接返回）
    Ok(())
}

// === 步骤 2: 编写消息流命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 Channel 参数签名（去掉下划线）：channel: Channel<StreamItem>
// TODO: 1..=5 循环发送 StreamItem
// 提示: channel
//         .send(StreamItem { step, label: format!("第 {step} 条消息") })
//         .map_err(|e| e.to_string())?;
fn start_stream(_channel: Channel<StreamItem>) -> Result<(), String> {
    // TODO: 补全循环推送逻辑（当前直接返回）
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 3: 注册命令 ————————————————————————————————————
        // TODO: 注册 start_progress 与 start_stream
        // 提示: .invoke_handler(tauri::generate_handler![start_progress, start_stream])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}