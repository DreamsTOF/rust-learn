// ============================================================
// 练习 E28: 系统托盘
// 目标: 托盘图标 + 菜单 + 关闭时隐藏到托盘，左键单击恢复
// 知识点: TrayIconBuilder / 托盘菜单与事件 / CloseRequested 拦截
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 退出命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全命令体：app.exit(0) 后返回 Ok(())
fn quit_app(_app: tauri::AppHandle) -> Result<(), String> {
    // TODO: 补全命令体（当前返回 Ok 占位）
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // === 步骤 2: 创建托盘菜单 ————————————————————————————————————
            // TODO: 创建「显示主窗口」/「退出」两个菜单项并组装成 Menu
            // 提示: let show_i = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
            //       let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            //       let menu = Menu::with_items(app, &[&show_i, &quit_i])?;
            //       需要 use tauri::menu::{Menu, MenuItem};
            // === 步骤 3: 构建托盘图标 ————————————————————————————————————
            // TODO: 补全 TrayIconBuilder 链：icon / menu / show_menu_on_left_click(false) / 事件 / build
            // 提示: let _tray = TrayIconBuilder::new()
            //         .icon(app.default_window_icon().unwrap().clone())
            //         .menu(&menu)
            //         .show_menu_on_left_click(false)
            //         .on_menu_event(|app, event| match event.id().as_ref() {
            //           "show" => {
            //             if let Some(w) = app.get_webview_window("main") {
            //               let _ = w.show();
            //               let _ = w.set_focus();
            //             }
            //           }
            //           "quit" => app.exit(0),
            //           _ => {}
            //         })
            //         .on_tray_icon_event(|tray, event| {
            //           // 左键单击托盘图标：恢复并聚焦主窗口
            //           if let TrayIconEvent::Click {
            //             button: MouseButton::Left,
            //             button_state: MouseButtonState::Up,
            //             ..
            //           } = event
            //           {
            //             let app = tray.app_handle();
            //             if let Some(w) = app.get_webview_window("main") {
            //               let _ = w.show();
            //               let _ = w.set_focus();
            //             }
            //           }
            //         })
            //         .build(app)?;
            //       需要 use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
            //       use tauri::Manager（get_webview_window / default_window_icon）
            // 当前占位：直接返回 Ok
            let _ = app;
            Ok(())
        })
        .on_window_event(|window, event| {
            // === 步骤 4: 关闭时隐藏到托盘 ————————————————————————————————————
            // TODO: 拦截 CloseRequested：api.prevent_close() 后 window.hide()
            // 提示: if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            //         api.prevent_close();
            //         let _ = window.hide();
            //       }
            let _ = (window, event);
        })
        // === 步骤 5: 注册命令 ————————————————————————————————————
        // TODO: 注册 quit_app
        // 提示: .invoke_handler(tauri::generate_handler![quit_app])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}