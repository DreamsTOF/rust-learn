// ============================================================
// 练习 004: 安装 Tauri CLI
//
// 目标: 检查 Tauri CLI 是否已安装
// 难度: ⭐⭐
// ============================================================

#[tauri::command]
fn check_tauri_cli() -> String {
    // TODO: 使用 Command::new("cargo").args(["tauri", "--version"]).output()
    // 检查 Tauri CLI 是否已安装
    //
    // 提示:
    // 1. 需要导入 use std::process::Command;
    // 2. 用 .output() 获取结果，返回 Result
    // 3. 使用 match 处理 Ok/Err 两种情况
    // - 成功: String::from_utf8_lossy(&output.stdout).trim().to_string()
    // - 失败: "未检测到 Tauri CLI，请运行: cargo install tauri-cli".to_string()
    todo!("实现 check_tauri_cli 函数 - 检查 Tauri CLI 是否已安装")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // TODO: 在此注册命令
            // check_tauri_cli,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
