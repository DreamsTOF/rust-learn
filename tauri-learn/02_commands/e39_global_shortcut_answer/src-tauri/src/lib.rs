// ============================================================
// 练习 E39: 全局快捷键
// 目标: 用 global-shortcut 插件注册/注销/查询全局快捷键，并接收按键事件
// 知识点: 插件注册 / register / unregister / is_registered / with_handler 事件回调 / emit 广播
// ============================================================

use std::str::FromStr;
use tauri::Emitter;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

/// 注册全局快捷键（与系统快捷键冲突时会返回错误，前端可展示）
#[tauri::command]
fn register_shortcut(app: tauri::AppHandle, combo: String) -> Result<String, String> {
    app.global_shortcut()
        .register(Shortcut::from_str(&combo).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    Ok(format!("已注册: {combo}"))
}

/// 注销全局快捷键
#[tauri::command]
fn unregister_shortcut(app: tauri::AppHandle, combo: String) -> Result<String, String> {
    app.global_shortcut()
        .unregister(Shortcut::from_str(&combo).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    Ok(format!("已注销: {combo}"))
}

/// 查询快捷键当前是否已注册
#[tauri::command]
fn is_shortcut_registered(app: tauri::AppHandle, combo: String) -> Result<bool, String> {
    let shortcut = Shortcut::from_str(&combo).map_err(|e| e.to_string())?;
    // is_registered 直接返回 bool（无需再 map_err）
    Ok(app.global_shortcut().is_registered(shortcut))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册 global-shortcut 插件，并设置快捷键事件回调：
        // 任何已注册快捷键被按下（Pressed）时都会走到这里，与窗口是否聚焦无关
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        println!("快捷键被按下: {:?}", shortcut);
                        // 广播给前端，前端通过 listen("shortcut-pressed") 接收
                        let _ = app.emit("shortcut-pressed", shortcut.to_string());
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            register_shortcut,
            unregister_shortcut,
            is_shortcut_registered
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}