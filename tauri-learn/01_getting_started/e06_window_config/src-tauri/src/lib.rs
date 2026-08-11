// ============================================================
// 练习 E06: 窗口配置
// 目标: 掌握窗口属性配置（title/尺寸/居中）与多窗口创建
// TODO: 按照注释提示补全代码
// ============================================================


// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 引入所需类型 ——————————————————————————————————
// get_webview_window 来自 Manager trait
use tauri::Manager;
// TODO: 填空时需要额外引入窗口构建类型：
//   use tauri::{WebviewUrl, WebviewWindowBuilder};

// === 步骤 2: 编写 open_about_window 命令 ——————————————————
// 创建「关于」子窗口：title / inner_size / center / resizable
// TODO: 添加 #[tauri::command] 属性
fn open_about_window(app: tauri::AppHandle) -> Result<(), String> {
    // 已存在同名窗口时直接聚焦，避免重复创建（保留这段逻辑）
    if let Some(win) = app.get_webview_window("about") {
        win.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // TODO: 使用 WebviewWindowBuilder 创建窗口，例如：
    //   WebviewWindowBuilder::new(&app, "about", WebviewUrl::App("index.html".into()))
    //       .title("关于本应用")
    //       .inner_size(420.0, 300.0)
    //       .center()
    //       .resizable(false)
    //       .build()
    //       .map_err(|e| e.to_string())?;

    Ok(())
}

// === 步骤 3: 注册命令 ————————————————————————————————————
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 open_about_window 命令
        // 提示: .invoke_handler(tauri::generate_handler![open_about_window])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}