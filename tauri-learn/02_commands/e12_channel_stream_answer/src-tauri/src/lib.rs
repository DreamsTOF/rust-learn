// ============================================================
// 练习 E12: Channel 流式传输
// 目标: 用 Channel 把后端数据流式推送给前端
// 知识点: Channel::new / send 推送 / onmessage 消费
// ============================================================

use tauri::ipc::Channel;

/// 流式消息结构：step 步号 + label 文本。
#[derive(Clone, serde::Serialize)]
struct StreamItem {
    step: u32,
    label: String,
}

/// 进度流：0 → 100 共 11 条消息，每条间隔 100ms。
#[tauri::command]
fn start_progress(channel: Channel<u32>) -> Result<(), String> {
    for i in 0..=10 {
        channel.send(i * 10).map_err(|e| e.to_string())?;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    Ok(())
}

/// 消息流：连续发送 5 条 StreamItem。
#[tauri::command]
fn start_stream(channel: Channel<StreamItem>) -> Result<(), String> {
    for step in 1..=5 {
        channel
            .send(StreamItem {
                step,
                label: format!("第 {step} 条消息"),
            })
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_progress, start_stream])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}