// ============================================================
// 练习 A02: 记事本 —— 答案版
// 目标: 路径 API、fs 插件读/写、Result 错误处理、React
// ============================================================

use std::io::{Read, Write};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_fs::{FsExt, OpenOptions};

/// 笔记文件的位置：应用数据目录下的 note.txt
fn note_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取应用数据目录失败：{e}"))?;
    Ok(dir.join("note.txt"))
}

/// 返回笔记文件的完整路径（前端显示用）
#[tauri::command]
fn note_file_path(app: AppHandle) -> Result<String, String> {
    note_path(&app).map(|p| p.to_string_lossy().into_owned())
}

/// 读取笔记内容；文件不存在时当作空笔记（首次打开）
#[tauri::command]
fn load_note(app: AppHandle) -> Result<String, String> {
    let path = note_path(&app)?;
    if !path.exists() {
        return Ok(String::new()); // 首次打开：没有文件，就当空笔记
    }
    let mut opts = OpenOptions::new();
    opts.read(true);
    let mut file = app
        .fs()
        .open(&path, opts)
        .map_err(|e| format!("打开文件失败：{e}"))?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|e| format!("读取失败：{e}"))?;
    Ok(content)
}

/// 保存笔记内容（目录不存在则先创建）
#[tauri::command]
fn save_note(app: AppHandle, content: String) -> Result<(), String> {
    let path = note_path(&app)?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("创建目录失败：{e}"))?;
    }
    let mut opts = OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    let mut file = app
        .fs()
        .open(&path, opts)
        .map_err(|e| format!("创建文件失败：{e}"))?;
    file.write_all(content.as_bytes()).map_err(|e| format!("写入失败：{e}"))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![note_file_path, load_note, save_note])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
