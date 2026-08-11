// ============================================================
// 练习 E04: 第一个命令
// 目标: 走通 #[tauri::command] → generate_handler! → invoke() 全链路
// 知识点: 命令定义 / 注册 / 前端调用 / 参数传递
// ============================================================

/// 接收 name 参数并返回问候语。
/// &str 参数会被前端按值传入，命令返回 String 直接序列化给前端。
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {name}! 这是你的第一个 Tauri 命令 🎉")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}