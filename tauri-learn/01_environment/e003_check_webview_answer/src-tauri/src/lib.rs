// ============================================================
// 练习 003: WebView2 环境检查 (答案)
//
// 目标: 检测当前系统 WebView2 的可用性（跨平台）
// ============================================================

#[tauri::command]
fn check_webview_status() -> String {
    let os = std::env::consts::OS;
    match os {
        "windows" => "Windows 系统 - WebView2 已内置于系统".to_string(),
        "macos" => "macOS 系统 - 使用 WKWebView (系统内置)".to_string(),
        "linux" => "Linux 系统 - 使用 WebKitGTK (需安装)".to_string(),
        _ => format!("未知平台: {}", os),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_webview_status])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
