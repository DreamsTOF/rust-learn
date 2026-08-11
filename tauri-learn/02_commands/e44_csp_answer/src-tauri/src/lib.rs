// ============================================================
// 练习 E44: 内容安全策略（CSP）
// 目标: 理解 security.csp 配置，并用命令读取当前 CSP 字符串
// 知识点: app.security.csp / default-src / style-src / connect-src / 配置读取
// ============================================================

/// 返回 tauri.conf.json 中配置的 CSP 字符串。
/// `app.config().app.security.csp` 类型是 `Option<Csp>`（`Csp` 实现了 `Display`），
/// 未配置时返回空字符串（由前端提示"未配置"）。
#[tauri::command]
fn get_csp(app: tauri::AppHandle) -> Result<String, String> {
    Ok(app
        .config()
        .app
        .security
        .csp
        .as_ref()
        .map(|c| c.to_string())
        .unwrap_or_default())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_csp])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}