// ============================================================
// 练习 E45: 权限系统（permissions）
// 目标: 用自定义 permission 文件收紧 fs 插件的读写 scope
// 知识点: capabilities / 自定义 permission / allow 路径 / scope 通配
// TODO: 按照注释提示补全代码
// ============================================================

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // 预创建笔记目录：fs 插件的 writeTextFile 不会自动创建父目录
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(dir.join("notes"))?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}