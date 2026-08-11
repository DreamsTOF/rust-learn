// ============================================================
// 练习 E48: 自动更新（updater）
// 目标: 集成 updater 插件并实现"检查更新"命令
// 知识点: 更新流程 / endpoints 模板变量 / pubkey 签名验证
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// TODO: 完成填空后取消注释（app.updater() 来自 UpdaterExt trait）
// use tauri_plugin_updater::UpdaterExt;

// === 步骤 1: 编写 check_update 命令 ——————————————————————————
// TODO: 添加 #[tauri::command] 属性（命令为 async，返回 Result<String, String>）
// TODO: 补全命令体：获取 updater 实例并调用 check() 处理三分支
// 提示: let updater = app.updater().map_err(|e| e.to_string())?;
//       match updater.check().await {
//           Ok(Some(update)) => Ok(format!("发现新版本 v{}", update.version)),
//           Ok(None) => Ok("已是最新版本".into()),
//           Err(e) => Err(format!("检查失败（更新源不可达或签名校验失败）: {e}")),
//       }
// 当前为占位实现（保持可编译），完成填空后替换为完整逻辑
#[tauri::command]
async fn check_update(_app: tauri::AppHandle) -> Result<String, String> {
    Err("TODO: 尚未实现 check_update".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        // TODO: 注册 check_update 命令
        // 提示: .invoke_handler(tauri::generate_handler![check_update])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}