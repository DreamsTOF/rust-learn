// ============================================================
// 练习 E06: 窗口配置
// 目标: 掌握窗口属性配置（title/尺寸/居中）与多窗口创建
// 知识点: tauri.conf.json 主窗口 / WebviewWindowBuilder 动态建窗
// ============================================================

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

/// 创建「关于」子窗口：标题 / 尺寸 / 居中 / 禁止缩放。
/// 已存在同名窗口时直接聚焦，避免重复创建。
#[tauri::command]
fn open_about_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("about") {
        win.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(&app, "about", WebviewUrl::App("index.html".into()))
        .title("关于本应用")
        .inner_size(420.0, 300.0)
        .center()
        .resizable(false)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_about_window])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}