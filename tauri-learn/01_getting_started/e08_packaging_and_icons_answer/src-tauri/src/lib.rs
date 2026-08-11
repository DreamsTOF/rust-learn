// ============================================================
// 练习 E08: 打包与图标
// 目标: 理解 bundle 产物、图标与 identifier 规范
// 知识点: Config.identifier / PackageInfo / cargo tauri icon
// ============================================================

#[derive(serde::Serialize)]
struct BundleInfo {
    identifier: String,
    product_name: String,
    version: String,
    icon_files: Vec<String>,
}

/// 返回打包相关元数据（来自运行时配置与清单）。
/// identifier 在 app.config() 中（反向域名风格），
/// 图标用 `cargo tauri icon <图片>` 一键生成全套。
#[tauri::command]
fn bundle_info(app: tauri::AppHandle) -> BundleInfo {
    let config = app.config();
    let package = app.package_info();

    BundleInfo {
        identifier: config.identifier.clone(),
        product_name: package.name.to_string(),
        version: package.version.to_string(),
        icon_files: vec![
            "icons/icon.ico          # Windows 可执行文件嵌入图标".into(),
            "icons/icon.png          # 通用 256×256 图标".into(),
            "icons/32x32.png         # 小尺寸图标".into(),
            "icons/128x128.png       # 中尺寸图标".into(),
            "icons/128x128@2x.png    # 高分屏图标".into(),
        ],
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![bundle_info])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}