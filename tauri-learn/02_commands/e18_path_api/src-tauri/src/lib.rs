// ============================================================
// 练习 E18: 路径 API
// 目标: 用 PathResolver 获取各系统目录并展示
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 定义返回结构体 ————————————————————————————————————
// TODO: 定义 PathItem 并派生 serde::Serialize
// 提示: #[derive(serde::Serialize)]
//       struct PathItem { name: String, path: String }
#[derive(serde::Serialize)]
struct PathItem {}

// === 步骤 2: 编写 list_paths 命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 用 app.path() 获取 6 个目录，组装 Vec<PathItem> 返回
// 提示: let p = app.path();  // 需要 use tauri::Manager;
//       ("app_data_dir", p.app_data_dir()) 等 6 项均返回 Result<PathBuf>，
//       转字符串: res.map(|x| x.display().to_string()).unwrap_or_else(|e| format!("错误: {e}"))
fn list_paths(_app: tauri::AppHandle) -> Result<Vec<PathItem>, String> {
    // TODO: 补全目录数组构造（当前返回空数组占位）
    Ok(Vec::new())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 3: 注册命令 ————————————————————————————————————
        // TODO: 注册 list_paths
        // 提示: .invoke_handler(tauri::generate_handler![list_paths])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}