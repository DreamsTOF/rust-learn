// ============================================================
// 练习 E30: 文件系统（fs 插件）
// 目标: 用 @tauri-apps/plugin-fs 读写文件、列目录、查信息、操作文件
// 知识点: fs 插件 API / scope 限制 / 应用数据目录
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}