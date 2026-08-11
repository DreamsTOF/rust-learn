// ============================================================
// 练习 E27: 应用菜单
// 目标: 构建应用菜单（子菜单/勾选项/分隔线/快捷键）并响应事件
// 知识点: Menu / Submenu / MenuItem / CheckMenuItem / separator / on_menu_event
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // === 步骤 1: 构建「文件」子菜单 ————————————————————————————————————
            // TODO: 补全菜单项数组：新建 / 打开 / 分隔线 / 深色模式勾选项 / 分隔线 / 退出
            // 提示: let file_menu = Submenu::with_items(app, "文件", true, &[
            //         &MenuItem::with_id(app, "new", "新建", true, Some("Ctrl+N"))?,
            //         &MenuItem::with_id(app, "open", "打开...", true, Some("Ctrl+O"))?,
            //         &PredefinedMenuItem::separator(app)?,
            //         &CheckMenuItem::with_id(app, "dark", "深色模式", true, true, None::<&str>)?,
            //         &PredefinedMenuItem::separator(app)?,
            //         &MenuItem::with_id(app, "quit", "退出", true, Some("Ctrl+Q"))?,
            //       ])?;
            // === 步骤 2: 组装应用菜单并设置 ————————————————————————————————————
            // TODO: Menu::with_items 组装后 app.set_menu(menu)
            // 提示: let menu = Menu::with_items(app, &[&file_menu])?;
            //       app.set_menu(menu)?;
            //       需要 use tauri::menu::{Menu, Submenu, MenuItem, CheckMenuItem, PredefinedMenuItem};
            //       set_menu 是 App 固有方法，无需额外 trait
            // 当前占位：直接返回 Ok
            let _ = app;
            Ok(())
        })
        .on_menu_event(|app, event| {
            // === 步骤 3: 响应菜单事件 ————————————————————————————————————
            // TODO: match event.id().as_ref() 处理 "new" / "open" / "quit" / "dark" 分支
            // 提示: "new" => { println!("[菜单] 新建"); let _ = app.emit("menu-action", "新建文档"); }
            //       "open" => { println!("[菜单] 打开"); let _ = app.emit("menu-action", "打开文档"); }
            //       "quit" => app.exit(0),
            //       "dark" => { println!("[菜单] 深色模式切换"); let _ = app.emit("menu-action", "深色模式切换"); }
            //       _ => {}
            //       app.emit 需要 use tauri::Emitter
            let _ = (app, event);
        })
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}