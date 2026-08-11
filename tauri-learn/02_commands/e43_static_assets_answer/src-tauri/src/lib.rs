// ============================================================
// 练习 E43: 静态资源
// 目标: 掌握 public/、src/assets、asset 协议三种资源使用方式
// 知识点: public 静态资源 / convertFileSrc / resource_dir
// ============================================================

use tauri::Manager;

/// 返回应用 resource 目录（打包后存放额外资源的位置）
#[tauri::command]
fn resource_info(app: tauri::AppHandle) -> Result<String, String> {
    // app.path() 需要 use tauri::Manager
    let dir = app.path().resource_dir().map_err(|e| e.to_string())?;
    Ok(dir.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![resource_info])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}