// ============================================================
// 练习 E01: 计数器 —— 答案版
// 目标: invoke / #[tauri::command] / generate_handler! / serde
// ============================================================

/// 返回 current + 1。参数与返回值通过 IPC 序列化往返前后端。
#[tauri::command]
fn count_up(current: i32) -> i32 {
    current + 1
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![count_up])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
