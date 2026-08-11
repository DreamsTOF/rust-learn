// ============================================================
// 练习 E47: 打包发布（packaging）
// 目标: 读取打包元信息，理解多平台产物与发布流程
// 知识点: bundle_info / 多平台产物 / 图标 / 体积优化
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 1: 定义 BundleInfo 结构体 ————————————————————————
// TODO: 补全字段并派生 serde::Serialize
// 提示: identifier: String, product_name: String,
//       version: String, platform: String
#[derive(Debug, serde::Serialize)]
struct BundleInfo {
    // TODO: 字段说明——identifier 来自 app.config().identifier
    //       product_name / version 来自 app.package_info()
    //       platform 来自 std::env::consts::OS
    identifier: String,
}

/// 读取当前应用的打包元信息。
/// 注意：identifier 在 `app.config().identifier`（PackageInfo 没有 identifier 字段）；
/// 名称/版本在 `app.package_info()`。
// === 步骤 2: 编写 bundle_info 命令 ——————————————————————————
// TODO: 添加 #[tauri::command] 属性并补全命令体
// 提示: identifier: app.config().identifier.clone(),
//       product_name: app.package_info().name.to_string(),
//       version: app.package_info().version.to_string(),
//       platform: std::env::consts::OS.to_string()
#[tauri::command]
fn bundle_info(_app: tauri::AppHandle) -> Result<BundleInfo, String> {
    Ok(BundleInfo {
        // TODO: 补全四个字段的取值（见上方提示）
        identifier: String::new(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 bundle_info 命令
        // 提示: .invoke_handler(tauri::generate_handler![bundle_info])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}