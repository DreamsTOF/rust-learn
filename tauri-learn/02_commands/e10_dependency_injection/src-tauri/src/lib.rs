// ============================================================
// 练习 E10: 依赖注入
// 目标: 在命令中注入 AppHandle / WebviewWindow / State<T> 多个依赖
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

use std::sync::Mutex;

/// 由 Builder.manage 注入的共享计数器。
struct Counter(Mutex<i32>);

/// inspect 命令的返回值：注入多个依赖后组装的信息。
#[derive(serde::Serialize)]
struct InspectInfo {
    window_title: String,
    window_label: String,
    window_count: usize,
    counter: i32,
    app_name: String,
}

// === 步骤 1: 编写 inspect 命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全注入参数签名（去掉下划线前缀）：
//   app: tauri::AppHandle / window: tauri::WebviewWindow / state: tauri::State<Counter>
// TODO: 填充 InspectInfo 字段：
//   window_title: window.title().map_err(|e| e.to_string())?
//   window_label: window.label().into()
//   window_count: app.webview_windows().len()
//   counter: *state.0.lock().map_err(|e| e.to_string())?
//   app_name: app.package_info().name.clone()
// 提示: app.webview_windows() 需要 use tauri::Manager;
fn inspect(
    _app: tauri::AppHandle,
    _window: tauri::WebviewWindow,
    _state: tauri::State<Counter>,
) -> Result<InspectInfo, String> {
    Ok(InspectInfo {
        window_title: String::new(),
        window_label: String::new(),
        window_count: 0,
        counter: 0,
        app_name: String::new(),
    })
}

// === 步骤 2: 编写 increment 命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性，lock 后 +1 并返回新值
// 提示: let mut c = state.0.lock().map_err(|e| e.to_string())?; *c += 1; Ok(*c)
fn increment(_state: tauri::State<Counter>) -> Result<i32, String> {
    Ok(0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 3: 注入共享状态 ————————————————————————————————————
        // TODO: 把 Counter 注册为可注入状态（初始值 0）
        // 提示: .manage(Counter(Mutex::new(0)))
        // === 步骤 4: 注册命令 ————————————————————————————————————
        // TODO: 注册 inspect 与 increment
        // 提示: .invoke_handler(tauri::generate_handler![inspect, increment])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}