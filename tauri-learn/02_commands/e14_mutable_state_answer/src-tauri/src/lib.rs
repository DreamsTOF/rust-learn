// ============================================================
// 练习 E14: 可变状态
// 目标: 用 Mutex / RwLock 管理多个可变状态并跨命令共享
// 知识点: manage() / State<T> / Mutex / RwLock / 多 State 并存
// ============================================================

use std::sync::{Mutex, RwLock};
use tauri::State;

/// 计数器：用 Mutex 保护。
struct Counter(Mutex<i32>);

/// 标签集合：用 RwLock 保护（读多写少场景）。
struct Tags(RwLock<Vec<String>>);

/// 计数 +1 并返回新值。
#[tauri::command]
fn increment(state: State<Counter>) -> Result<i32, String> {
    let mut counter = state.0.lock().map_err(|e| e.to_string())?;
    *counter += 1;
    Ok(*counter)
}

/// 添加一个标签，返回当前标签数量。
#[tauri::command]
fn add_tag(tag: String, state: State<Tags>) -> Result<usize, String> {
    let mut tags = state.0.write().map_err(|e| e.to_string())?;
    tags.push(tag);
    Ok(tags.len())
}

/// 清空所有标签。
#[tauri::command]
fn clear_tags(state: State<Tags>) -> Result<(), String> {
    let mut tags = state.0.write().map_err(|e| e.to_string())?;
    tags.clear();
    Ok(())
}

/// 返回全部标签。
#[tauri::command]
fn get_tags(state: State<Tags>) -> Result<Vec<String>, String> {
    let tags = state.0.read().map_err(|e| e.to_string())?;
    Ok(tags.clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Counter(Mutex::new(0)))
        .manage(Tags(RwLock::new(Vec::new())))
        .invoke_handler(tauri::generate_handler![
            increment,
            add_tag,
            clear_tags,
            get_tags
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}