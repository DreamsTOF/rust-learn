// ============================================================
// 练习 A01: 待办清单 —— 答案版
// 目标: manage / State<T> / Mutex、结构体 + serde、listen/emit
// ============================================================

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

/// 一条待办：id 唯一、text 内容、done 是否完成
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TodoItem {
    pub id: u64,
    pub text: String,
    pub done: bool,
}

/// 后端记住的"整个应用的数据"：待办列表 + 自增 id
#[derive(Default)]
struct TodoState {
    items: Mutex<Vec<TodoItem>>,
    next_id: Mutex<u64>,
}

/// 返回当前全部待办
#[tauri::command]
fn list_todos(state: State<'_, TodoState>) -> Vec<TodoItem> {
    state.items.lock().unwrap().clone()
}

/// 新增一条待办，返回更新后的列表
#[tauri::command]
fn add_todo(app: AppHandle, state: State<'_, TodoState>, text: String) -> Vec<TodoItem> {
    let text = text.trim().to_string();
    let mut items = state.items.lock().unwrap();
    let mut next_id = state.next_id.lock().unwrap();
    let item = TodoItem { id: *next_id, text: text.clone(), done: false };
    *next_id += 1;
    items.push(item);
    let _ = app.emit("todo-log", format!("添加：{text}"));
    items.clone()
}

/// 勾选/取消一条待办，返回更新后的列表
#[tauri::command]
fn toggle_todo(app: AppHandle, state: State<'_, TodoState>, id: u64) -> Vec<TodoItem> {
    let mut items = state.items.lock().unwrap();
    if let Some(item) = items.iter_mut().find(|i| i.id == id) {
        item.done = !item.done;
        let action = if item.done { "完成" } else { "重开" };
        let _ = app.emit("todo-log", format!("{action}：{}", item.text));
    }
    items.clone()
}

/// 删除一条待办，返回更新后的列表
#[tauri::command]
fn delete_todo(app: AppHandle, state: State<'_, TodoState>, id: u64) -> Vec<TodoItem> {
    let mut items = state.items.lock().unwrap();
    if let Some(idx) = items.iter().position(|i| i.id == id) {
        let item = items.remove(idx);
        let _ = app.emit("todo-log", format!("删除：{}", item.text));
    }
    items.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(TodoState::default())
        .invoke_handler(tauri::generate_handler![list_todos, add_todo, toggle_todo, delete_todo])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
