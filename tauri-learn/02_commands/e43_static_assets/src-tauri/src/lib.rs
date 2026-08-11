// ============================================================
// 练习 E43: 静态资源
// 目标: 掌握 public/、src/assets、asset 协议三种资源使用方式
// 知识点: public 静态资源 / convertFileSrc / resource_dir
// TODO: 按照注释提示补全代码
// ============================================================


// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// TODO: 完成填空后取消注释（app.path() 需要该 trait）
// use tauri::Manager;

// === 步骤 1: resource_info 命令 ————————————————————————————————
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全函数：返回 resource_dir 路径字符串
// 提示: let dir = app.path().resource_dir().map_err(|e| e.to_string())?;
//       Ok(dir.to_string_lossy().to_string())
// 说明: resource_dir 是打包后存放额外资源的位置（开发模式下是 target 目录）
fn resource_info(_app: tauri::AppHandle) -> Result<String, String> {
    // TODO: 取 resource_dir 并转字符串返回
    // 当前为占位（完成填空后替换）
    Ok(String::new())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 注册 resource_info 命令
        // 提示: .invoke_handler(tauri::generate_handler![resource_info])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}