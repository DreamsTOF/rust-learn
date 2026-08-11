// ============================================================
// 练习 E31: 对话框（dialog 插件）
// 目标: 用 @tauri-apps/plugin-dialog 打开/保存文件、选择目录、确认与消息
// 知识点: open / save / ask / message / 文件过滤器 / 取消处理
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}