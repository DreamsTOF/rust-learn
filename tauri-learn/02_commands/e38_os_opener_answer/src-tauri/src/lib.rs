// ============================================================
// 练习 E38: OS 与 Opener
// 目标: 查询系统信息，并用 opener 打开 URL / 在资源管理器中显示文件
// 知识点: 插件注册 / os:default + opener:default 权限 / platform-version-arch-type-family / openUrl / revealItemInDir
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册 OS 插件（查询平台/版本/架构/类型等信息）
        .plugin(tauri_plugin_os::init())
        // 注册 Opener 插件（用系统默认应用打开 URL / 文件，或定位到资源管理器）
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}