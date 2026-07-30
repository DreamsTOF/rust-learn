// ============================================================
// 练习 001: Rust 环境检查
//
// 目标: 使用 std::process::Command 调用 rustc --version 检查 Rust 版本
// 难度: ⭐
// ============================================================

#[tauri::command]
fn check_rust_version() -> String {
    // TODO: 使用 Command::new("rustc").arg("--version").output() 获取 Rust 版本
    // 提示: 使用 String::from_utf8_lossy(&output.stdout).trim().to_string() 处理结果
    // 需要先导入: use std::process::Command;
    todo!("实现 check_rust_version 函数 - 调用 rustc --version")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // TODO: 在此注册命令
            // check_rust_version,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
