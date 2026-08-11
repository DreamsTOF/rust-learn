// ============================================================
// 练习 E32: Shell（shell 插件）
// 目标: 用 @tauri-apps/plugin-shell 执行外部命令，读取 stdout/stderr
// 知识点: Command.create / execute / 超时 / scope 白名单
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}