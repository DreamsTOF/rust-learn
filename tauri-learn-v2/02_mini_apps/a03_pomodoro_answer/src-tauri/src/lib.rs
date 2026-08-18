// ============================================================
// 练习 A03: 番茄钟 —— 答案版
// 目标: async 命令 + tokio、通知插件、系统托盘 + 隐藏窗口
// ============================================================

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, WindowEvent};
use tauri_plugin_notification::NotificationExt;

/// 计时状态：running 防止重复开始，stop_requested 用于中途停止
#[derive(Default)]
struct PomodoroState {
    running: Mutex<bool>,
    stop_requested: AtomicBool,
}

/// 开始倒计时：async 命令，每秒 emit 剩余秒数，结束发通知
#[tauri::command]
async fn start_pomodoro(
    app: AppHandle,
    state: tauri::State<'_, PomodoroState>,
    minutes: u64,
) -> Result<(), String> {
    {
        let mut running = state.running.lock().unwrap();
        if *running {
            return Err("已经在计时了".into());
        }
        *running = true;
    }
    state.stop_requested.store(false, Ordering::SeqCst);

    let total = minutes * 60;
    for remaining in (1..=total).rev() {
        if state.stop_requested.load(Ordering::SeqCst) {
            break;
        }
        let _ = app.emit("pomodoro-tick", remaining);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    // 无论正常结束还是被打断，都要复位 running
    *state.running.lock().unwrap() = false;

    if state.stop_requested.load(Ordering::SeqCst) {
        state.stop_requested.store(false, Ordering::SeqCst);
        let _ = app.emit("pomodoro-stopped", ());
        return Ok(());
    }

    let _ = app.emit("pomodoro-tick", 0);
    let _ = app.emit("pomodoro-done", ());

    app.notification()
        .builder()
        .title("番茄钟")
        .body("时间到！休息一下吧。")
        .show()
        .map_err(|e| format!("通知失败：{e}"))?;
    Ok(())
}

/// 中途停止计时
#[tauri::command]
fn stop_pomodoro(state: tauri::State<'_, PomodoroState>) {
    state.stop_requested.store(true, Ordering::SeqCst);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .manage(PomodoroState::default())
        .setup(|app| {
            // 托盘菜单：显示/隐藏 + 退出
            let show_item = MenuItem::with_id(app, "toggle", "显示/隐藏", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "toggle" => {
                        if let Some(win) = app.get_webview_window("main") {
                            if win.is_visible().unwrap_or(false) {
                                let _ = win.hide();
                            } else {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    // 左键单击托盘图标 → 显示窗口
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            // 点关闭按钮 → 不是退出，而是隐藏到托盘
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![start_pomodoro, stop_pomodoro])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
