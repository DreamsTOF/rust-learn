// ============================================================
// 练习 A07: 批量重命名 —— 练习版
// 目标: 后台任务（async_runtime::spawn）、Channel 进度、dialog
// TODO: 按注释提示补全（共 5 处）
// ============================================================

use std::path::PathBuf;
use serde::Serialize;
use tauri::ipc::Channel;

/// 预览项：旧名 → 新名
#[derive(Serialize, Clone)]
pub struct PreviewItem {
    old: String,
    new: String,
}

/// 进度载荷：每处理完一个文件推一次
#[derive(Serialize, Clone)]
pub struct RenameProgress {
    done: u64,
    total: u64,
    current: String,
    finished: bool,
}

/// 计算新文件名：把 find 全部替换成 replace
fn build_new_name(file_name: &str, find: &str, replace: &str) -> String {
    // === 步骤 1 ————————————————————————————————————————————
    // TODO: find 为空则原样返回；否则返回 file_name.replace(find, replace)
    // 提示: if find.is_empty() { file_name.to_string() } else { file_name.replace(find, replace) }
    file_name.to_string() // ← 替换成你的代码
}

/// 列出目录里的全部文件（排序）
fn list_files(dir: &str) -> Result<Vec<PathBuf>, String> {
    // === 步骤 2 ————————————————————————————————————————————
    // TODO: 用 std::fs::read_dir 遍历目录，把 is_file() 的路径收进 files，排序后返回
    // 提示: for entry in std::fs::read_dir(dir).map_err(|e| format!("打开目录失败：{e}"))? {
    //         let entry = entry.map_err(|e| format!("读取目录项失败：{e}"))?;
    //         let path = entry.path();
    //         if path.is_file() { files.push(path); }
    //       }
    //       files.sort(); Ok(files)
    let mut files = Vec::new();
    Ok(files) // ← 替换成你的代码
}

/// 预览：返回会改名的文件（旧名 → 新名）
#[tauri::command]
fn preview_rename(dir: String, find: String, replace: String) -> Result<Vec<PreviewItem>, String> {
    // === 步骤 3 ————————————————————————————————————————————
    // TODO: 遍历文件，算新名，只收集"会改名"的（new_name != file_name）
    // 提示: for path in list_files(&dir)? {
    //         if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
    //           let new_name = build_new_name(file_name, &find, &replace);
    //           if new_name != file_name {
    //             items.push(PreviewItem { old: file_name.to_string(), new: new_name });
    //           }
    //         }
    //       }
    let mut items = Vec::new();
    Ok(items) // ← 替换成你的代码
}

/// 执行重命名：spawn 后台任务，进度通过 Channel 推给前端
#[tauri::command]
fn run_rename(
    dir: String,
    find: String,
    replace: String,
    on_progress: Channel<RenameProgress>,
) -> Result<(), String> {
    let files = list_files(&dir)?;
    let total = files.len() as u64;

    tauri::async_runtime::spawn(async move {
        let mut done = 0u64;
        for path in files {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .map(String::from)
                .unwrap_or_default();
            let new_name = build_new_name(&file_name, &find, &replace);
            // === 步骤 4 ————————————————————————————————————————————
            // TODO: 需要改名（new_name != file_name）就 std::fs::rename，
            //       然后 done += 1，把进度 send 到 on_progress
            // 提示: if new_name != file_name {
            //         if let Some(parent) = path.parent() {
            //           let new_path = parent.join(&new_name);
            //           let _ = std::fs::rename(&path, &new_path);
            //         }
            //       }
            //       done += 1;
            //       let _ = on_progress.send(RenameProgress {
            //         done, total, current: new_name, finished: done == total,
            //       });
            done += 1;
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // === 步骤 5 ————————————————————————————————————————————
        // TODO: 注册 dialog 插件，登记 preview_rename / run_rename
        // 提示: .plugin(tauri_plugin_dialog::init())
        //       .invoke_handler(tauri::generate_handler![preview_rename, run_rename])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
