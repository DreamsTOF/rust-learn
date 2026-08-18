# 练习 A08 答案讲解：剪贴板历史

> **用法**：卡住时再看本页。每一处 TODO 按三步走——先看"练习版缺什么"，再看"答案版填了什么"，最后回查原理文档对应小节。
> **作业范围**：本练习只需动 2 个文件：`src-tauri/src/lib.rs`（后端）与 `src/App.tsx`（前端 React），共 9 处 TODO。

## 对照总览

| 文件 | 练习版状态 | 你缺的 |
|---|---|---|
| `src-tauri/src/lib.rs` | 存取历史、监控循环骨架已给 | 去重插最前、写剪贴板、变化判断、注册快捷键、登记命令 |
| `src/App.tsx` | 界面已给 | `invoke` 导入、拉历史、复制、清空 |

## lib.rs TODO 1：push_history 去重 + 插最前

### 练习版这里是什么

```rust
fn push_history(app: &AppHandle, text: &str) {
    let mut history = load_history(app);
    // TODO: 去重、插最前、截断
    save_history(app, &history);
    let _ = app.emit("clipboard-history", history);
}
```

### 答案版填了什么

```rust
    if let Some(pos) = history.iter().position(|s| s == text) {
        history.remove(pos);   // 去重：把旧位置删掉
    }
    history.insert(0, text.to_string());  // 插到最前（最近使用优先）
    history.truncate(MAX_ITEMS);          // 限长：最多保留 50 条
```

### 为什么

- `position` + `remove`：如果这条内容已经在历史里，先删掉旧位置——**避免重复，且保证"最近复制"排最前**
- `insert(0, ...)`：新的放最前
- `truncate(MAX_ITEMS)`：历史无限膨胀会拖垮 Store，截断到 50 条

### 回查文档

[第 2 节：后台监控与去重](a08_clipboard_history.md#sec-a08-watch)。

## lib.rs TODO 2：copy_text 写剪贴板

### 答案版填了什么

```rust
#[tauri::command]
fn copy_text(app: AppHandle, text: String) -> Result<(), String> {
    app.clipboard()
        .write_text(text.clone())
        .map_err(|e| format!("写入剪贴板失败：{e}"))?;
    push_history(&app, &text);
    Ok(())
}
```

### 为什么

- `app.clipboard()` 来自 `ClipboardExt`；`write_text` 把内容放回系统剪贴板（= 用户点了"复制"）
- `map_err` + `?`：剪贴板被占用等失败要变成人话
- 写回后也 `push_history`：这条内容被用到，提到最前

### 回查文档

[第 1 节：剪贴板插件](a08_clipboard_history.md#sec-a08-clipboard)。

## lib.rs TODO 3：监控循环"变了才记"

### 答案版填了什么

```rust
let current = current.trim().to_string();
if current.is_empty() || current == last {
    continue;
}
last = current.clone();
push_history(&app, &current);
```

### 为什么

- `last` 是"上次看到的内容"：`current == last` 说明没变化，跳过——**否则同一段文字每 800ms 记一条**
- `is_empty()` 跳过空内容（清空剪贴板不算"复制"）
- 变化了才更新 `last` 并记入历史

### 回查文档

[第 2 节：后台监控](a08_clipboard_history.md#sec-a08-watch)。

## lib.rs TODO 4：注册全局快捷键

### 答案版填了什么

```rust
app.global_shortcut().register("ctrl+shift+v")?;
```

### 为什么

- `GlobalShortcutExt` 提供 `global_shortcut()`；`register` 把快捷键交给系统
- 按键事件由插件 `Builder::with_handler` 里的全局处理器收到（练习版已给），`event.state() == Pressed` 时切换窗口显隐
- **必须注册才有效**——不注册，快捷键事件根本不会发生

### 回查文档

[第 3 节：全局快捷键](a08_clipboard_history.md#sec-a08-shortcut)。

## lib.rs TODO 5：登记命令

```rust
.invoke_handler(tauri::generate_handler![get_history, copy_text, clear_history])
```

老规矩：命令必须登记。

## App.tsx TODO 1-4：invoke + 拉历史 + 复制 + 清空

### 答案版填了什么

```typescript
import { invoke } from "@tauri-apps/api/core";
// useEffect 里：
invoke<string[]>("get_history").then(setHistory).catch(() => {});
// copy 里：
await invoke("copy_text", { text });
// clear 里：
await invoke("clear_history");
```

### 为什么

- 启动时 `get_history` 拉一次存量（Store 里的历史），之后靠 `listen("clipboard-history")` 增量更新
- 点历史项 → `copy_text`（后端写回剪贴板 + 提到最前）
- 清空 → `clear_history`（后端清 Store + emit 空列表，前端因事件自动清空）

### 回查文档

[第 4 节：应用菜单](a08_clipboard_history.md#sec-a08-menu)。

## 验收标准

```bash
cd 02_mini_apps/a08_clipboard_history
cargo tauri dev
```

复制一段文字 → 约 1 秒内出现在历史里；点它 → "已复制回剪贴板"；Ctrl+Shift+V → 窗口隐藏/显示（在其他程序里也有效）；重启应用 → 历史还在；窗口菜单"退出" → 真正退出。

**破坏性验证**（确认你是理解了，而不是碰巧对）：

- 删掉 `current == last` 判断 → 同一段文字被疯狂记录（验证去重）
- 注释掉 `register("ctrl+shift+v")` → 快捷键无效（验证注册的必要性）
- 把 `push_history` 里 `emit` 删掉 → 列表不更新，但 Store 里有数据（验证 emit 是"通知前端刷新"的通道）
- 把 `truncate(MAX_ITEMS)` 删掉 → 复制很多次后 Store 文件膨胀（验证限长）

## 升级挑战（可选）

- 把轮询改成"剪贴板序列号检测"：Windows 上用 `GetClipboardSequenceNumber` 判断变化，省去 800ms 轮询
- 加"搜索历史"：顶部加个输入框过滤列表（纯前端过滤即可）
