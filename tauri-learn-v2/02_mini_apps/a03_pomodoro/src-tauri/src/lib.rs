// ============================================================
// 练习 A03: 番茄钟 —— 练习版
// 目标: async 命令 + tokio、通知插件、系统托盘 + 隐藏窗口
// TODO: 按注释提示补全（共 6 处）
// ============================================================

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
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
    // === 步骤 1 ————————————————————————————————————————————
    // TODO: 倒计时循环——每秒 emit 一次 "pomodoro-tick"（剩余秒数），
    //       然后 sleep 1 秒；被 stop_requested 打断就 break
    // 提示: for remaining in (1..=total).rev() {
    //         if state.stop_requested.load(Ordering::SeqCst) { break; }
    //         let _ = app.emit("pomodoro-tick", remaining);
    //         tokio::time::sleep(Duration::from_secs(1)).await;
    //       }
    let _ = total; // 占位：完成后删除

    // 无论正常结束还是被打断，都要复位 running
    *state.running.lock().unwrap() = false;

    if state.stop_requested.load(Ordering::SeqCst) {
        state.stop_requested.store(false, Ordering::SeqCst);
        let _ = app.emit("pomodoro-stopped", ());
        return Ok(());
    }

    let _ = app.emit("pomodoro-tick", 0);
    let _ = app.emit("pomodoro-done", ());

    // === 步骤 2 ————————————————————————————————————————————
    // TODO: 发系统通知"时间到"
    // 提示: app.notification().builder().title("番茄钟").body("时间到！休息一下吧。")
    //           .show().map_err(|e| format!("通知失败：{e}"))?;
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
        // === 步骤 3 ————————————————————————————————————————————
        // TODO: 注册通知插件 + 用 .manage 注册 PomodoroState
        // 提示: .plugin(tauri_plugin_notification::init())
        //       .manage(PomodoroState::default())
        .setup(|app| {
            // === 步骤 4 ————————————————————————————————————————————
            // TODO: 创建系统托盘（菜单：显示/隐藏、退出；左键点击显示窗口）
            // 提示: tauri::menu::Menu / MenuItem::with_id
            //       tauri::tray::TrayIconBuilder::new().icon(...).menu(&menu)...
            //       参考答案讲解 a03_pomodoro_answer.md
            Ok(())
        })
        .on_window_event(|window, event| {
            // === 步骤 5 ————————————————————————————————————————————
            // TODO: 点关闭按钮时 prevent_close + hide（隐藏到托盘而不是退出）
            // 提示: if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            //         api.prevent_close(); let _ = window.hide();
            //       }
        })
        .invoke_handler(tauri::generate_handler![
            // === 步骤 6 ————————————————————————————————————————————
            // TODO: 登记 start_pomodoro / stop_pomodoro
            // 提示: start_pomodoro, stop_pomodoro,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
