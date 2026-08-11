// ============================================================
// 练习 E35: 通知
// 目标: 使用 tauri-plugin-notification 请求权限、发送系统通知并监听点击
// 知识点: 插件注册 / notification:default 权限 / 权限检查与请求 / 通知点击事件
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册通知插件（Windows 上通知显示在系统通知中心）
        .plugin(tauri_plugin_notification::init())
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}