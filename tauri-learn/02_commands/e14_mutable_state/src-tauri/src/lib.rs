// ============================================================
// 练习 E14: 可变状态
// 目标: 用 Mutex / RwLock 管理多个可变状态并跨命令共享
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

use std::sync::{Mutex, RwLock};

/// 计数器：用 Mutex 保护。
struct Counter(Mutex<i32>);

/// 标签集合：用 RwLock 保护（读多写少场景）。
struct Tags(RwLock<Vec<String>>);

// === 步骤 1: 编写计数器命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性，lock 后 +1 并返回新值
// 提示: let mut c = state.0.lock().map_err(|e| e.to_string())?; *c += 1; Ok(*c)
fn increment(_state: tauri::State<Counter>) -> Result<i32, String> {
    // TODO: 补全 lock 与自增逻辑（当前返回 0 占位）
    Ok(0)
}

// === 步骤 2: 编写标签命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性，补全 add_tag / clear_tags / get_tags
// 提示: add_tag:   state.0.write().map_err(|e| e.to_string())? 后 push，返回 tags.len()
//       clear_tags: write lock 后 clear
//       get_tags:   read lock 后 clone 返回
fn add_tag(_tag: String, _state: tauri::State<Tags>) -> Result<usize, String> {
    // TODO: 补全 push 逻辑（当前返回 0 占位）
    Ok(0)
}

fn clear_tags(_state: tauri::State<Tags>) -> Result<(), String> {
    // TODO: 补全 clear 逻辑（当前直接返回）
    Ok(())
}

fn get_tags(_state: tauri::State<Tags>) -> Result<Vec<String>, String> {
    // TODO: 补全读取并返回标签（当前返回空数组占位）
    Ok(Vec::new())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 3: 注入两个共享状态 ————————————————————————————————————
        // TODO: 注入 Counter（初始 0）与 Tags（空 Vec）
        // 提示: .manage(Counter(Mutex::new(0)))
        //       .manage(Tags(RwLock::new(Vec::new())))
        // === 步骤 4: 注册命令 ————————————————————————————————————
        // TODO: 注册 increment / add_tag / clear_tags / get_tags
        // 提示: .invoke_handler(tauri::generate_handler![
        //         increment, add_tag, clear_tags, get_tags
        //       ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}