// ============================================================
// 练习 E39: 全局快捷键
// 目标: 用 global-shortcut 插件注册/注销/查询全局快捷键，并接收按键事件
// 知识点: 插件注册 / register / unregister / is_registered / with_handler 事件回调 / emit 广播
// TODO: 按照注释提示补全代码
// ============================================================


// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// TODO: 完成填空后取消注释（解析快捷键字符串用）
// use std::str::FromStr;
// TODO: 完成填空后取消注释（app.emit 广播事件用）
// use tauri::Emitter;
// TODO: 完成填空后取消注释（插件 API：GlobalShortcutExt / Shortcut / ShortcutState）
// use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

// === 步骤 2: 注册快捷键命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全函数：解析 combo 并调用 register 注册全局快捷键，返回 "已注册: {combo}"
// 提示: app.global_shortcut().register(Shortcut::from_str(&combo).map_err(|e| e.to_string())?)
//       （GlobalShortcutExt 提供 global_shortcut()；Shortcut::from_str 解析组合键字符串）
fn register_shortcut(_app: tauri::AppHandle, _combo: String) -> Result<String, String> {
    // TODO: 注册后返回 Ok(format!("已注册: {combo}"))
    // 当前为占位（完成填空后替换）
    Ok(String::new())
}

// === 步骤 3: 注销快捷键命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全函数：解析 combo 并调用 unregister 注销，返回 "已注销: {combo}"
// 提示: app.global_shortcut().unregister(Shortcut::from_str(&combo).map_err(|e| e.to_string())?)
fn unregister_shortcut(_app: tauri::AppHandle, _combo: String) -> Result<String, String> {
    // TODO: 注销后返回 Ok(format!("已注销: {combo}"))
    // 当前为占位（完成填空后替换）
    Ok(String::new())
}

// === 步骤 4: 查询快捷键状态命令 ——————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全函数：解析 combo 并返回 is_registered 的布尔结果
// 提示: is_registered 直接返回 bool（无需 map_err）：
//       let shortcut = Shortcut::from_str(&combo).map_err(|e| e.to_string())?;
//       Ok(app.global_shortcut().is_registered(shortcut))
fn is_shortcut_registered(_app: tauri::AppHandle, _combo: String) -> Result<bool, String> {
    // TODO: 返回注册状态
    // 当前为占位（完成填空后替换）
    Ok(false)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册 global-shortcut 插件，并设置快捷键事件回调（保持两版一致）
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    // === 步骤 1: 处理快捷键按下事件 ——————————————————————————
                    // TODO: 判断 event.state() == ShortcutState::Pressed 时：
                    //   1) println!("快捷键被按下: {:?}", shortcut);
                    //   2) app.emit("shortcut-pressed", shortcut.to_string());
                    // 提示: event 是 ShortcutEvent（按值传入），state() 返回 ShortcutState；
                    //       app.emit 需要 use tauri::Emitter；emit 返回 Result，可用 let _ = 忽略
                    // 当前为占位（完成填空后替换，以下一行仅用于消除未使用警告）
                    let _ = (app, shortcut, event);
                })
                .build(),
        )
        // TODO: 注册 register_shortcut / unregister_shortcut / is_shortcut_registered
        // 提示: .invoke_handler(tauri::generate_handler![register_shortcut, unregister_shortcut, is_shortcut_registered])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}