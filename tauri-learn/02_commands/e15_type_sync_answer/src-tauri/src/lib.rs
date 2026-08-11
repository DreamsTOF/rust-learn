// ============================================================
// 练习 E15: 前后端类型同步
// 目标: 定义前后端一致的接口类型并利用自动字段转换
// 知识点: TS 接口 / invoke<T> 泛型 / snake_case ↔ camelCase 自动转换
// ============================================================

/// 用户资料：Rust 侧使用 snake_case 字段名。
#[derive(serde::Serialize)]
struct UserProfile {
    user_id: u32,
    display_name: String,
    tags: Vec<String>,
}

/// 按用户 ID 返回资料（演示数据）。
#[tauri::command]
fn get_profile(user_id: u32) -> Result<UserProfile, String> {
    Ok(UserProfile {
        user_id,
        display_name: format!("用户 #{user_id}"),
        tags: vec!["rust".into(), "tauri".into(), "typescript".into()],
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_profile])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}