// ============================================================
// 练习 007: 开发服务器与热更新 (练习版)
//
// 目标: 使用条件编译判断当前是 dev 还是 release 模式
// 难度: ⭐
//
// 说明:
//   实现 get_env_mode 命令，使用 #[cfg(debug_assertions)] 和
//   #[cfg(not(debug_assertions))] 条件编译返回不同字符串。
// ============================================================

#[tauri::command]
fn get_env_mode() -> String {
    todo!("实现条件编译：debug 模式返回 '开发模式 (debug)'，release 模式返回 '生产模式 (release)'")

    // 提示：使用 #[cfg(debug_assertions)] 和 #[cfg(not(debug_assertions))]
    // 分别定义两个函数或使用 cfg! 宏
    //
    // 方法一（条件编译块）:
    // #[cfg(debug_assertions)]
    // { "开发模式 (debug)".to_string() }
    // #[cfg(not(debug_assertions))]
    // { "生产模式 (release)".to_string() }
    //
    // 方法二（cfg! 宏）:
    // if cfg!(debug_assertions) {
    //     "开发模式 (debug)".to_string()
    // } else {
    //     "生产模式 (release)".to_string()
    // }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // TODO: 在此注册命令
            // get_env_mode,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
