# 练习 A07: 批量重命名（答案版）

**目标：** 选一个文件夹，把里面的文件批量改名（如把所有 `IMG_` 换成 `photo_`）。**先预览再执行**，执行时用 **Channel** 把进度推给前端显示进度条。

**对照练习版补全的内容：**

- `src-tauri/src/lib.rs`
  - `build_new_name`：`file_name.replace(find, replace)`
  - `list_files`：`std::fs::read_dir` + `is_file()` + `sort()`
  - `preview_rename`：只收集 `new_name != file_name` 的项
  - `run_rename`：`async_runtime::spawn` 里 `std::fs::rename` + `on_progress.send(...)`
  - `.plugin(tauri_plugin_dialog::init())` + 登记两个命令
- `src/App.tsx`
  - `open({ directory: true })` 选文件夹
  - `new Channel<RenameProgress>()` + `onmessage` + `invoke("run_rename", { onProgress: channel })`

**完整讲解见：** `tauri-learn-book-v2/src/02_mini_apps/a07_batch_rename_answer.md`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1435
- identifier: com.taurilearn.a07a
