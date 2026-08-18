// ============================================================
// 练习 A01: 待办清单 —— 练习版
// 目标: manage / State<T> / Mutex、结构体 + serde、listen/emit
// TODO: 按注释提示补全（共 6 处）
// ============================================================

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

/// 一条待办：id 唯一、text 内容、done 是否完成
// 理解即可，不用填：TodoItem 要作为命令返回值穿越进程边界，
// 所以必须派生 serde 序列化（字段名就是 JSON 的 key）。
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
    // === 步骤 3 ————————————————————————————————————————————
    // TODO: 后端主动通知前端：发一条 "todo-log" 事件，内容 "添加：{text}"
    // 提示: let _ = app.emit("todo-log", format!("添加：{text}"));
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
        // === 步骤 4 ————————————————————————————————————————————
        // TODO: 发一条 "todo-log" 事件，内容 "{action}：{item.text}"
        // 提示: 同步骤 3，把格式串换成 format!("{action}：{}", item.text)
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
        // === 步骤 5 ————————————————————————————————————————————
        // TODO: 发一条 "todo-log" 事件，内容 "删除：{item.text}"
        // 提示: 同步骤 3，把格式串换成 format!("删除：{}", item.text)
        let _ = app.emit("todo-log", format!("删除：{}", item.text));
    }
    items.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 2 ————————————————————————————————————————————
        // TODO: 用 .manage(...) 把 TodoState 注册成后端共享状态
        // 提示: .manage(TodoState::default())
        .manage(TodoState::default())
        .invoke_handler(tauri::generate_handler![
            list_todos,
            // === 步骤 6 ————————————————————————————————————
            // TODO: 登记 add_todo / toggle_todo / delete_todo
            // 提示: add_todo, toggle_todo, delete_todo,
            add_todo,
            toggle_todo,
            delete_todo,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
