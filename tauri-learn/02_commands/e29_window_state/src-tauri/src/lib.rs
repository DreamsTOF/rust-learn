// ============================================================
// 练习 E29: 窗口状态持久化（window-state 插件）
// 目标: 用 tauri-plugin-window-state 保存/恢复窗口位置、大小、最大化状态
// 知识点: window-state 插件 / StateFlags / save_window_state / 状态文件
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// TODO: 引入 window-state 插件的扩展 trait 与状态标志
// 提示: use tauri_plugin_window_state::{AppHandleExt, StateFlags};

// === 步骤 1: 保存窗口状态命令 ——————————————————————————————
// TODO: 添加 #[tauri::command] 属性，并把参数 _app 改名为 app，补全函数体
// 提示: app.save_window_state(StateFlags::all()).map_err(|e| e.to_string())?;
//       成功后返回 Ok("窗口状态已保存".into())
fn save_window_state(_app: tauri::AppHandle) -> Result<String, String> {
    // TODO: 调用 app.save_window_state(StateFlags::all()) 保存全部窗口状态
    Ok(String::new())
}

// === 步骤 2: 清除窗口状态命令 ——————————————————————————————
// TODO: 添加 #[tauri::command] 属性，并把参数 _app 改名为 app，补全函数体：
//   1. let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
//   2. let state_file = dir.join(app.filename());   // 状态文件名来自 AppHandleExt
//   3. 若存在则 std::fs::remove_file(state_file)
//   4. 成功返回 Ok("已清除保存的状态".into())
// 提示: 插件没有 Rust 侧清除 API，手动删除状态文件等价于前端 clearWindowState()
fn clear_window_state(_app: tauri::AppHandle) -> Result<String, String> {
    // TODO: 删除 app_config_dir 下的窗口状态文件
    Ok(String::new())
}

// === 步骤 3: 注册插件与命令 ————————————————————————————————
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 window-state 插件（插件会自动恢复/保存窗口状态）
        // 提示: .plugin(tauri_plugin_window_state::Builder::default().build())
        // TODO: 补全两个命令的 #[tauri::command] 属性后，取消注释注册行
        // 提示: .invoke_handler(tauri::generate_handler![save_window_state, clear_window_state])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}