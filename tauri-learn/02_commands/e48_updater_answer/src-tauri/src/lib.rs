// ============================================================
// 练习 E48: 自动更新（updater）
// 目标: 集成 updater 插件并实现"检查更新"命令
// 知识点: 更新流程 / endpoints 模板变量 / pubkey 签名验证
// ============================================================

use tauri_plugin_updater::UpdaterExt;

/// 检查更新：本练习没有真实更新源，预期返回 Err（错误信息即教学点）。
#[tauri::command]
async fn check_update(app: tauri::AppHandle) -> Result<String, String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    match updater.check().await {
        Ok(Some(update)) => Ok(format!("发现新版本 v{}", update.version)),
        Ok(None) => Ok("已是最新版本".into()),
        Err(e) => Err(format!("检查失败（更新源不可达或签名校验失败）: {e}")),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![check_update])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}