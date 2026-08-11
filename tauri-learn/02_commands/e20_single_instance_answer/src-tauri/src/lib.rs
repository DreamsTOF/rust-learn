// ============================================================
// 练习 E20: 单实例
// 目标: 用 single-instance 插件拦截重复启动并唤起已有窗口
// 知识点: tauri-plugin-single-instance / 重复启动回调 / 共享状态
// ============================================================

use tauri::Manager;

/// 当前实例 ID：首次启动的进程写入，之后所有窗口读取到的都是它。
struct InstanceId(String);

/// 读取实例 ID。单实例生效时，第二次启动会被拦截、不会创建新进程，
/// 前端拿到的仍是首个进程的 ID——这正是单实例生效的证明。
#[tauri::command]
fn get_instance_id(state: tauri::State<InstanceId>) -> Result<String, String> {
    Ok(state.0.clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 单实例插件：重复启动时回调，聚焦已有主窗口
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            println!("重复启动被拦截，参数: {args:?}，工作目录: {cwd}");
            let _ = app.get_webview_window("main").map(|w| w.set_focus());
        }))
        .manage(InstanceId(format!("实例-{}", std::process::id())))
        .invoke_handler(tauri::generate_handler![get_instance_id])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}