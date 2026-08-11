// ============================================================
// 练习 E03: 运行与构建
// 目标: 理解 tauri dev / tauri build 与 devUrl / frontendDist
// 知识点: Config（devUrl / frontendDist / identifier）/ PackageInfo
// ============================================================

#[derive(serde::Serialize)]
struct BuildInfo {
    dev_url: Option<String>,
    frontend_dist: String,
    identifier: String,
    product_name: String,
}

/// 读取运行时配置，展示 dev / build 两种模式的差异。
/// dev 模式：WebView 加载 devUrl（Vite dev server，热更新）
/// build 模式：WebView 加载 frontendDist 打包产物
#[tauri::command]
fn build_info(app: tauri::AppHandle) -> BuildInfo {
    let config = app.config();
    let package = app.package_info();

    BuildInfo {
        dev_url: config.build.dev_url.as_ref().map(|u| u.to_string()),
        frontend_dist: config
            .build
            .frontend_dist
            .as_ref()
            .map(|d| d.to_string())
            .unwrap_or_default(),
        identifier: config.identifier.clone(),
        product_name: package.name.to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![build_info])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}