// ============================================================
// 练习 A04: 图片查看器 —— 答案版
// 目标: 拖放、静态资源（asset 协议）、窗口操作
// ============================================================

use std::path::Path;

/// 判断是不是常见图片格式
fn is_image(path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase());
    matches!(ext.as_deref(), Some("png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp"))
}

/// 列出一个目录里的全部图片路径（排序后返回）
#[tauri::command]
fn list_images(dir: String) -> Result<Vec<String>, String> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(&dir).map_err(|e| format!("打开目录失败：{e}"))? {
        let entry = entry.map_err(|e| format!("读取目录项失败：{e}"))?;
        let path = entry.path();
        if path.is_file() && is_image(&path) {
            paths.push(path.to_string_lossy().into_owned());
        }
    }
    paths.sort();
    Ok(paths)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_images])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
