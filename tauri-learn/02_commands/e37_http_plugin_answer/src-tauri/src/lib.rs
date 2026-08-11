// ============================================================
// 练习 E37: HTTP 插件
// 目标: 使用 tauri-plugin-http 在前端发起 GET/POST 请求并处理错误
// 知识点: 插件注册 / http:default 权限与 URL scope / fetch / Headers / 超时 / 非 2xx
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册 HTTP 插件（请求在 Rust 侧执行，绕过浏览器 CORS 限制）
        .plugin(tauri_plugin_http::init())
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}