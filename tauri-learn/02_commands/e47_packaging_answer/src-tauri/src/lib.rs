// ============================================================
// 练习 E47: 打包发布（packaging）
// 目标: 读取打包元信息，理解多平台产物与发布流程
// 知识点: bundle_info / 多平台产物 / 图标 / 体积优化
// ============================================================

/// 打包元信息：identifier 来自 config，名称/版本来自 package_info。
#[derive(Debug, serde::Serialize)]
struct BundleInfo {
    identifier: String,
    product_name: String,
    version: String,
    platform: String,
}

/// 读取当前应用的打包元信息。
/// 注意：identifier 在 `app.config().identifier`（PackageInfo 没有 identifier 字段）；
/// 名称/版本在 `app.package_info()`。
#[tauri::command]
fn bundle_info(app: tauri::AppHandle) -> Result<BundleInfo, String> {
    Ok(BundleInfo {
        identifier: app.config().identifier.clone(),
        product_name: app.package_info().name.to_string(),
        version: app.package_info().version.to_string(),
        platform: std::env::consts::OS.to_string(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![bundle_info])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}