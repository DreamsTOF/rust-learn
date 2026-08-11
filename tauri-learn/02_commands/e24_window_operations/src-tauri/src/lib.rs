// ============================================================
// 练习 E24: 创建与操作窗口
// 目标: 动态创建子窗口，并用命令移动/缩放/居中/显隐/关闭
// 知识点: WebviewWindowBuilder / label 管理 / set_position / set_size / center / is_visible
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

use tauri::Manager;

// === 步骤 1: 创建 ops 窗口 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 WebviewWindowBuilder 创建 'ops' 窗口（title "可操作窗口" / inner_size 500x400 / build）
// 提示: WebviewWindowBuilder::new(&app, "ops", WebviewUrl::App("index.html".into()))
//           .title("可操作窗口")
//           .inner_size(500.0, 400.0)
//           .build()
//           .map_err(|e| e.to_string())?;
//       需要 use tauri::{WebviewUrl, WebviewWindowBuilder};
fn spawn_ops_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("ops") {
        win.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    // TODO: 补全 builder 链创建 ops 窗口（当前直接返回 Ok 占位）
    Ok(())
}

// === 步骤 2: 移动窗口 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 set_position 调用（PhysicalPosition::new(x, y)）
// 提示: app.get_webview_window("ops").ok_or("窗口未打开")?
//           .set_position(tauri::PhysicalPosition::new(x, y))
//           .map_err(|e| e.to_string())
fn move_window(_app: tauri::AppHandle, _x: i32, _y: i32) -> Result<(), String> {
    // TODO: 补全 set_position 调用（当前返回 Ok 占位）
    Ok(())
}

// === 步骤 3: 缩放窗口 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 set_size 调用（PhysicalSize::new(w, h)）
// 提示: app.get_webview_window("ops").ok_or("窗口未打开")?
//           .set_size(tauri::PhysicalSize::new(w, h))
//           .map_err(|e| e.to_string())
fn resize_window(_app: tauri::AppHandle, _w: u32, _h: u32) -> Result<(), String> {
    // TODO: 补全 set_size 调用（当前返回 Ok 占位）
    Ok(())
}

// === 步骤 4: 居中窗口 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 center() 调用
// 提示: app.get_webview_window("ops").ok_or("窗口未打开")?
//           .center()
//           .map_err(|e| e.to_string())
fn center_window(_app: tauri::AppHandle) -> Result<(), String> {
    // TODO: 补全 center() 调用（当前返回 Ok 占位）
    Ok(())
}

// === 步骤 5: 显隐切换 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全 toggle 逻辑：is_visible() 为 true 则 hide()，否则 show()，返回新可见性
// 提示: let win = app.get_webview_window("ops").ok_or("窗口未打开")?;
//       let visible = win.is_visible().map_err(|e| e.to_string())?;
//       if visible { win.hide().map_err(|e| e.to_string())?; }
//       else { win.show().map_err(|e| e.to_string())?; }
//       Ok(!visible)
fn toggle_window(_app: tauri::AppHandle) -> Result<bool, String> {
    // TODO: 补全 toggle 逻辑（当前返回 false 占位）
    Ok(false)
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
        // === 步骤 6: 注册命令 ————————————————————————————————————
        // TODO: 注册 spawn_ops_window / move_window / resize_window / center_window / toggle_window / close_ops_window
        // 提示: .invoke_handler(tauri::generate_handler![
        //         spawn_ops_window, move_window, resize_window,
        //         center_window, toggle_window, close_ops_window
        //       ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}