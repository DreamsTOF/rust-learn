# 练习 A02: 记事本（答案版）

**目标：** 做一个"关掉软件字还在"的记事本。内容存成**文件**（`app_data_dir` 下的 `note.txt`），用**fs 插件**读写，前后端都带 `Result` 错误处理。本课起前端改用 **React**。

**对照练习版补全的内容：**

- `src-tauri/src/lib.rs`
  - `load_note`：`app.fs().open(&path, OpenOptions { read: true, .. })` + `read_to_string`
  - `save_note`：`OpenOptions { write: true, create: true, truncate: true, .. }` + `write_all`
  - `.plugin(tauri_plugin_fs::init())` 注册插件
  - `generate_handler![note_file_path, load_note, save_note]`
- `src/App.tsx`
  - `useEffect` 里 `invoke<string>("note_file_path")` / `invoke<string>("load_note")`
  - `save` 里 `await invoke("save_note", { content })`

**完整讲解见：** `tauri-learn-book-v2/src/02_mini_apps/a02_notepad_answer.md`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1425
- identifier: com.taurilearn.a02a
