// ============================================================
// 练习 E27: 应用菜单
// 目标: 构建应用菜单（子菜单/勾选项/分隔线/快捷键）并响应事件
// 知识点: Menu / Submenu / MenuItem / CheckMenuItem / separator / on_menu_event
// ============================================================

use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 「文件」子菜单：新建 / 打开 / 分隔线 / 深色模式勾选项 / 分隔线 / 退出
            let file_menu = Submenu::with_items(app, "文件", true, &[
                &MenuItem::with_id(app, "new", "新建", true, Some("Ctrl+N"))?,
                &MenuItem::with_id(app, "open", "打开...", true, Some("Ctrl+O"))?,
                &PredefinedMenuItem::separator(app)?,
                &CheckMenuItem::with_id(app, "dark", "深色模式", true, true, None::<&str>)?,
                &PredefinedMenuItem::separator(app)?,
                &MenuItem::with_id(app, "quit", "退出", true, Some("Ctrl+Q"))?,
            ])?;

            // 应用菜单 = 一个「文件」子菜单（set_menu 是 App 固有方法，无需额外 trait）
            let menu = Menu::with_items(app, &[&file_menu])?;
            app.set_menu(menu)?;
            Ok(())
        })
        .on_menu_event(|app, event| {
            // 根据菜单项 id 分发处理（快捷键触发也会走到这里）
            match event.id().as_ref() {
                "new" => {
                    println!("[菜单] 新建");
                    let _ = app.emit("menu-action", "新建文档");
                }
                "open" => {
                    println!("[菜单] 打开");
                    let _ = app.emit("menu-action", "打开文档");
                }
                "quit" => app.exit(0),
                "dark" => {
                    println!("[菜单] 深色模式切换");
                    let _ = app.emit("menu-action", "深色模式切换");
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}