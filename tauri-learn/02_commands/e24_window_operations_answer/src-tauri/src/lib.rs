// ============================================================
// 练习 E24: 创建与操作窗口
// 目标: 动态创建子窗口，并用命令移动/缩放/居中/显隐/关闭
// 知识点: WebviewWindowBuilder / label 管理 / set_position / set_size / center / is_visible
// ============================================================

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

/// 创建 'ops' 可操作窗口（500x400，加载 index.html）。
/// 已存在同名窗口时直接聚焦，避免重复创建。
#[tauri::command]
fn spawn_ops_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("ops") {
        win.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    WebviewWindowBuilder::new(&app, "ops", WebviewUrl::App("index.html".into()))
        .title("可操作窗口")
        .inner_size(500.0, 400.0)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 把 ops 窗口移动到屏幕坐标 (x, y)。
#[tauri::command]
fn move_window(app: tauri::AppHandle, x: i32, y: i32) -> Result<(), String> {
    app.get_webview_window("ops")
        .ok_or("窗口未打开")?
        .set_position(tauri::PhysicalPosition::new(x, y))
        .map_err(|e| e.to_string())
}

/// 把 ops 窗口缩放为 w x h 像素。
#[tauri::command]
fn resize_window(app: tauri::AppHandle, w: u32, h: u32) -> Result<(), String> {
    app.get_webview_window("ops")
        .ok_or("窗口未打开")?
        .set_size(tauri::PhysicalSize::new(w, h))
        .map_err(|e| e.to_string())
}

/// 让 ops 窗口在屏幕中居中。
#[tauri::command]
fn center_window(app: tauri::AppHandle) -> Result<(), String> {
    app.get_webview_window("ops")
        .ok_or("窗口未打开")?
        .center()
        .map_err(|e| e.to_string())
}

/// 切换 ops 窗口显示/隐藏，返回切换后的可见性。
#[tauri::command]
fn toggle_window(app: tauri::AppHandle) -> Result<bool, String> {
    let win = app.get_webview_window("ops").ok_or("窗口未打开")?;
    let visible = win.is_visible().map_err(|e| e.to_string())?;
    if visible {
        win.hide().map_err(|e| e.to_string())?;
    } else {
        win.show().map_err(|e| e.to_string())?;
    }
    Ok(!visible)
}

/// 关闭并销毁 ops 窗口。
#[tauri::command]
fn close_ops_window(app: tauri::AppHandle) -> Result<(), String> {
    app.get_webview_window("ops")
        .ok_or("窗口未打开")?
        .close()
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            spawn_ops_window,
            move_window,
            resize_window,
            center_window,
            toggle_window,
            close_ops_window
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}