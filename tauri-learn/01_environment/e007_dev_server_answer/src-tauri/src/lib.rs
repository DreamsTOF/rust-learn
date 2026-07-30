// ============================================================
// 练习 007: 开发服务器与热更新 (答案版)
//
// 目标: 使用条件编译判断当前是 dev 还是 release 模式
// 难度: ⭐
// ============================================================

#[tauri::command]
fn get_env_mode() -> String {
    // 方法一：使用条件编译属性块（推荐用于学习条件编译）
    #[cfg(debug_assertions)]
    { "开发模式 (debug)".to_string() }
    #[cfg(not(debug_assertions))]
    { "生产模式 (release)".to_string() }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_env_mode])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
