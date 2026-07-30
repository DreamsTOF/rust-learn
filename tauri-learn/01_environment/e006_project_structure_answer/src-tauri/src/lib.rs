// ============================================================
// 练习 006: 项目结构详解 (答案版)
//
// 目标: 使用 std::fs::read_dir 读取项目目录结构
// 难度: ⭐
// ============================================================

#[tauri::command]
fn get_structure_overview() -> String {
    let mut info = String::new();
    info.push_str("📁 项目结构:\n\n");

    // 列出 src/ 目录
    if let Ok(entries) = std::fs::read_dir("src") {
        info.push_str("src/\n");
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                info.push_str(&format!("  ├── {}\n", name));
            }
        }
    }

    // 列出 src-tauri/src/ 目录
    if let Ok(entries) = std::fs::read_dir("src-tauri/src") {
        info.push_str("src-tauri/src/\n");
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                info.push_str(&format!("  ├── {}\n", name));
            }
        }
    }

    info
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_structure_overview])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
