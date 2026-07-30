// ============================================================
// 练习 003: WebView2 环境检查
//
// 目标: 检测当前系统 WebView2 的可用性（跨平台）
// 难度: ⭐⭐
// ============================================================

#[tauri::command]
fn check_webview_status() -> String {
    let os = std::env::consts::OS;
    // TODO: 使用 match 匹配 os，返回对应平台的 WebView 状态信息
    // - "windows" => "Windows 系统 - WebView2 已内置于系统"
    // - "macos"   => "macOS 系统 - 使用 WKWebView (系统内置)"
    // - "linux"   => "Linux 系统 - 使用 WebKitGTK (需安装)"
    // - _         => "未知平台: {os}"
    todo!("实现 check_webview_status 函数 - 检测当前平台和 WebView 状态")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // TODO: 在此注册命令
            // check_webview_status,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
