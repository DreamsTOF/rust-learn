// ============================================================
// 练习 E01: 计数器 —— 练习版
// 目标: invoke / #[tauri::command] / generate_handler! / serde
// TODO: 按注释提示补全两处
// ============================================================

// === 步骤 1 ————————————————————————————————————————————————
// TODO: 给 count_up 函数加上 #[tauri::command] 属性，
//       让它成为"可被前端 invoke 调用的命令"。
// 提示: 在 `fn count_up` 的上一行写 `#[tauri::command]`
fn count_up(current: i32) -> i32 {
    current + 1
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // === 步骤 2 ————————————————————————————————————
            // TODO: 在命令注册表里登记 count_up
            // 提示: 取消注释下面这一行
            // count_up,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
