// ============================================================
// 练习 E13: 命令模块化
// 目标: 把命令拆分到 commands/ 子模块并按路径注册
// 知识点: 模块拆分 / 子模块定义命令 / 跨模块注册
// ============================================================

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::math::add,
            commands::math::sub,
            commands::text::to_upper,
            commands::text::word_count
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}