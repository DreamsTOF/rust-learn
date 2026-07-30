// ============================================================
// 练习 006: 项目结构详解 (练习版)
//
// 目标: 使用 std::fs::read_dir 读取项目目录结构
// 难度: ⭐
//
// 说明:
//   实现 get_structure_overview 命令，列出 src/ 和 src-tauri/src/
//   目录下的文件，返回格式化的目录树字符串。
// ============================================================

#[tauri::command]
fn get_structure_overview() -> String {
    todo!("实现目录树遍历：读取 src/ 和 src-tauri/src/ 目录并格式化输出")

    // 提示：使用 std::fs::read_dir 遍历目录
    // let mut info = String::new();
    // info.push_str("📁 项目结构:\n\n");
    //
    // // 列出 src/ 目录
    // if let Ok(entries) = std::fs::read_dir("src") {
    //     info.push_str("src/\n");
    //     for entry in entries.flatten() {
    //         if let Some(name) = entry.file_name().to_str() {
    //             info.push_str(&format!("  ├── {}\n", name));
    //         }
    //     }
    // }
    //
    // // 列出 src-tauri/src/ 目录
    // if let Ok(entries) = std::fs::read_dir("src-tauri/src") {
    //     info.push_str("src-tauri/src/\n");
    //     for entry in entries.flatten() {
    //         if let Some(name) = entry.file_name().to_str() {
    //             info.push_str(&format!("  ├── {}\n", name));
    //         }
    //     }
    // }
    //
    // info
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // TODO: 在此注册命令
            // get_structure_overview,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
