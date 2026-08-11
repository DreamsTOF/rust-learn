// ============================================================
// 练习 E33: SQL（sql 插件）
// 目标: 用 @tauri-apps/plugin-sql 建表、增删查、绑定参数与事务
// 知识点: Database.load / execute / select / 绑定参数 / 事务
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}