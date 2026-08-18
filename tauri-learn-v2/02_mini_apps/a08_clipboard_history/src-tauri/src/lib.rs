// ============================================================
// 练习 A08: 剪贴板历史 —— 练习版
// 目标: 剪贴板插件、全局快捷键、应用菜单
// TODO: 按注释提示补全（共 5 处）
// ============================================================

use std::time::Duration;
use tauri::menu::{Menu, MenuItem};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use tauri_plugin_store::StoreExt;

const HISTORY_FILE: &str = "clipboard-history.json";
const HISTORY_KEY: &str = "history";
/// 最多保留的历史条数
const MAX_ITEMS: usize = 50;

fn load_history(app: &AppHandle) -> Vec<String> {
    app.store(HISTORY_FILE)
        .ok()
        .and_then(|store| store.get(HISTORY_KEY))
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}

fn save_history(app: &AppHandle, history: &[String]) {
    if let Ok(store) = app.store(HISTORY_FILE) {
        store.set(HISTORY_KEY, serde_json::json!(history));
        let _ = store.save();
    }
}

/// 把一条内容插入历史最前（去重 + 限长），并广播给前端
fn push_history(app: &AppHandle, text: &str) {
    let mut history = load_history(app);
    // === 步骤 1 ————————————————————————————————————————————
    // TODO: 去重（重复的移除）、插到最前、截断到 MAX_ITEMS
    // 提示: if let Some(pos) = history.iter().position(|s| s == text) { history.remove(pos); }
    //       history.insert(0, text.to_string());
    //       history.truncate(MAX_ITEMS);
    save_history(app, &history);
    let _ = app.emit("clipboard-history", history);
}

/// 启动时拉取历史
#[tauri::command]
fn get_history(app: AppHandle) -> Vec<String> {
    load_history(&app)
}

/// 把选中的历史项写回剪贴板（相当于"粘贴"）
#[tauri::command]
fn copy_text(app: AppHandle, text: String) -> Result<(), String> {
    // === 步骤 2 ————————————————————————————————————————————
    // TODO: 把 text 写回剪贴板
    // 提示: app.clipboard().write_text(text.clone()).map_err(|e| format!("写入剪贴板失败：{e}"))?;
    push_history(&app, &text);
    Ok(())
}

/// 清空历史
#[tauri::command]
fn clear_history(app: AppHandle) -> Result<(), String> {
    save_history(&app, &[]);
    let _ = app.emit("clipboard-history", Vec::<String>::new());
    Ok(())
}

/// 后台监控剪贴板：每 800ms 读一次，发现变化就记入历史
fn spawn_clipboard_watcher(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut last = String::new();
        loop {
            tokio::time::sleep(Duration::from_millis(800)).await;
            let Ok(current) = app.clipboard().read_text() else {
                continue;
            };
            let current = current.trim().to_string();
            // === 步骤 3 ————————————————————————————————————————————
            // TODO: 内容为空或没变化就跳过；否则更新 last 并 push_history
            // 提示: if current.is_empty() || current == last { continue; }
            //       last = current.clone();
            //       push_history(&app, &current);
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        if let Some(win) = app.get_webview_window("main") {
                            if win.is_visible().unwrap_or(false) {
                                let _ = win.hide();
                            } else {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // === 步骤 4 ————————————————————————————————————————————
            // TODO: 注册全局快捷键 Ctrl+Shift+V
            // 提示: app.global_shortcut().register("ctrl+shift+v")?;

            // 应用菜单（窗口菜单栏）
            let clear_item = MenuItem::with_id(app, "clear", "清空历史", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&clear_item, &quit_item])?;
            app.set_menu(menu)?;
            app.on_menu_event(|app, event| match event.id.as_ref() {
                "clear" => {
                    save_history(app, &[]);
                    let _ = app.emit("clipboard-history", Vec::<String>::new());
                }
                "quit" => app.exit(0),
                _ => {}
            });

            // 后台监控剪贴板
            spawn_clipboard_watcher(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // === 步骤 5 ————————————————————————————————————
            // TODO: 登记 get_history / copy_text / clear_history
            // 提示: get_history, copy_text, clear_history,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
