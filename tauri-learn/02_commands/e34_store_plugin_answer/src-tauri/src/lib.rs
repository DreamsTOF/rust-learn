// ============================================================
// 练习 E34: Store 插件
// 目标: 使用 tauri-plugin-store 实现键值持久化与变化监听
// 知识点: 插件注册 / store:default 权限 / 前端 load-set-get-has-delete-save
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册 store 插件（数据文件默认保存在 app_data_dir 下）
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}