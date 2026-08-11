// ============================================================
// 练习 E20: 单实例
// 目标: 用 single-instance 插件拦截重复启动并唤起已有窗口
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 定义实例 ID 状态 ————————————————————————————————————
// TODO: 定义 struct InstanceId(String)
// 提示: struct InstanceId(String);
struct InstanceId(String);

// === 步骤 2: 编写读取实例 ID 的命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全函数体：返回 state.0 的克隆
// 提示: fn get_instance_id(state: tauri::State<InstanceId>) -> Result<String, String> {
//           Ok(state.0.clone())
//       }
fn get_instance_id(_state: tauri::State<InstanceId>) -> Result<String, String> {
    // TODO: 补全返回值（当前返回空串占位）
    Ok(String::new())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 3: 注册单实例插件 ————————————————————————————————————
        // TODO: 补全插件 init 闭包：打印日志 + 聚焦已有的 main 窗口
        // 提示: .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
        //           println!("重复启动被拦截，参数: {args:?}，工作目录: {cwd}");
        //           let _ = app.get_webview_window("main").map(|w| w.set_focus());
        //       }))
        //       需要 use tauri::Manager;
        .plugin(tauri_plugin_single_instance::init(|_app, args, cwd| {
            println!("重复启动被拦截，参数: {args:?}，工作目录: {cwd}");
            // TODO: 聚焦 main 窗口（set_focus）
            // 提示: let _ = _app.get_webview_window("main").map(|w| w.set_focus());
        }))
        // === 步骤 4: 注入实例 ID 状态 ————————————————————————————————————
        // TODO: 注入 InstanceId（值: 实例-{进程 ID}）
        // 提示: .manage(InstanceId(format!("实例-{}", std::process::id())))
        // === 步骤 5: 注册命令 ————————————————————————————————————
        // TODO: 注册 get_instance_id
        // 提示: .invoke_handler(tauri::generate_handler![get_instance_id])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}