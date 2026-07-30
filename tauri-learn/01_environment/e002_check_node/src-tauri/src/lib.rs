// ============================================================
// 练习 002: Node.js 环境检查
//
// 目标: 使用 std::process::Command 调用 node --version 检查 Node.js 版本
// 难度: ⭐
// ============================================================

#[tauri::command]
fn check_node_version() -> String {
    // TODO: 使用 Command::new("node").arg("--version").output() 获取 Node.js 版本
    // 提示: 参考 e001 的 check_rust_version 实现，将 "rustc" 改为 "node"
    // 需要先导入: use std::process::Command;
    todo!("实现 check_node_version 函数 - 调用 node --version")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // TODO: 在此注册命令
            // check_node_version,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
