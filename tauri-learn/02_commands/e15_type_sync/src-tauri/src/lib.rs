// ============================================================
// 练习 E15: 前后端类型同步
// 目标: 定义前后端一致的接口类型并利用自动字段转换
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 定义返回结构体 ————————————————————————————————————
// TODO: 定义 UserProfile 并派生 serde::Serialize（Rust 侧用 snake_case）
// 提示: #[derive(serde::Serialize)]
//       struct UserProfile {
//           user_id: u32,
//           display_name: String,
//           tags: Vec<String>,
//       }
#[derive(serde::Serialize)]
struct UserProfile {}

// === 步骤 2: 编写 get_profile 命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 构造 UserProfile 返回（演示数据即可）
// 提示: Ok(UserProfile {
//         user_id,
//         display_name: format!("用户 #{user_id}"),
//         tags: vec!["rust".into(), "tauri".into(), "typescript".into()],
//       })
fn get_profile(_user_id: u32) -> Result<UserProfile, String> {
    Ok(UserProfile {})
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 3: 注册命令 ————————————————————————————————————
        // TODO: 注册 get_profile
        // 提示: .invoke_handler(tauri::generate_handler![get_profile])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}