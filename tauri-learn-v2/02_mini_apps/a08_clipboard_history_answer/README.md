# 练习 A08: 剪贴板历史（答案版）

**目标：** 后台盯着剪贴板，复制过的文字都记下来；点历史项就复制回剪贴板；按 **Ctrl+Shift+V** 呼出/隐藏窗口。历史用 **Store** 持久化，重启还在。

**对照练习版补全的内容：**

- `src-tauri/src/lib.rs`
  - `push_history`：去重（`position` + `remove`）→ `insert(0, ...)` → `truncate(MAX_ITEMS)` → 保存 + `emit`
  - `copy_text`：`app.clipboard().write_text(text)`
  - 监控循环：`current.is_empty() || current == last` 跳过，否则更新 `last` + `push_history`
  - `app.global_shortcut().register("ctrl+shift+v")`
  - `generate_handler![get_history, copy_text, clear_history]`
- `src/App.tsx`
  - `invoke<string[]>("get_history")` 启动拉取
  - `invoke("copy_text", { text })` / `invoke("clear_history")`

**完整讲解见：** `tauri-learn-book-v2/src/02_mini_apps/a08_clipboard_history_answer.md`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1437
- identifier: com.taurilearn.a08a
