// ============================================================
// 练习 E13: 命令模块化
// 目标: 把命令拆分到 commands/ 子模块并按路径注册
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 声明命令模块 ————————————————————————————————————
// TODO: 取消注释，声明 commands 模块（其内部再声明 math / text 子模块）
// 提示: mod commands;
// mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 2: 注册跨模块命令 ————————————————————————————————————
        // TODO: 用完整路径注册四个命令
        // 提示: .invoke_handler(tauri::generate_handler![
        //         commands::math::add, commands::math::sub,
        //         commands::text::to_upper, commands::text::word_count
        //       ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}