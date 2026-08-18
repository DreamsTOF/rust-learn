// ============================================================
// 练习 A02: 记事本 —— 练习版
// 目标: 路径 API、fs 插件读/写、Result 错误处理、React
// TODO: 按注释提示补全（共 6 处）
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
    // === 步骤 1 ————————————————————————————————————————————
    // TODO: 用 fs 插件打开文件（只读）并读成字符串返回
    // 提示: let mut opts = OpenOptions::new(); opts.read(true);
    //       app.fs().open(&path, opts)
    //           .map_err(|e| format!("打开文件失败：{e}"))?   → 得到一个 std::fs::File
    //       再 file.read_to_string(&mut content) 读出内容
    Ok(String::from("（TODO：读取文件内容）")) // ← 替换成你的代码
}

/// 保存笔记内容（目录不存在则先创建）
#[tauri::command]
fn save_note(app: AppHandle, content: String) -> Result<(), String> {
    let path = note_path(&app)?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("创建目录失败：{e}"))?;
    }
    // === 步骤 2 ————————————————————————————————————————————
    // TODO: 用 fs 插件以"可写 + 不存在则创建 + 先清空"打开文件并写入 content
    // 提示: let mut opts = OpenOptions::new();
    //       opts.write(true).create(true).truncate(true);
    //       然后 app.fs().open(&path, opts)，再 file.write_all(content.as_bytes())
    Ok(()) // ← 替换成你的代码
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 3 ————————————————————————————————————————————
        // TODO: 注册 fs 插件（load_note / save_note 依赖它）
        // 提示: .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            // === 步骤 4 ————————————————————————————————————————————
            // TODO: 登记 note_file_path / load_note / save_note
            // 提示: note_file_path, load_note, save_note,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
