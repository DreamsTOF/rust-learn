// ============================================================
// 练习 E36: 剪贴板
// 目标: 使用 tauri-plugin-clipboard-manager 读写与清空系统剪贴板
// 知识点: 插件注册 / clipboard-manager:default 权限 / writeText / readText / clear
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册剪贴板插件（读写系统剪贴板）
        .plugin(tauri_plugin_clipboard_manager::init())
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}