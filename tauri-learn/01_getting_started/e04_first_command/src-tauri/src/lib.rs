// ============================================================
// 练习 E04: 第一个命令
// 目标: 走通 #[tauri::command] → generate_handler! → invoke() 全链路
// TODO: 按照注释提示补全代码
// ============================================================


// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 编写 greet 命令 ——————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全函数：接收 name: &str，返回 String 问候语
// 提示: format!("你好, {name}! ...")
fn greet() -> String {
    // TODO: 使用 format! 拼接待返回的问候语（可以带上 emoji 🎉）
    String::new()
}

// === 步骤 2: 注册命令 ————————————————————————————————————
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 greet 命令
        // 提示: .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}