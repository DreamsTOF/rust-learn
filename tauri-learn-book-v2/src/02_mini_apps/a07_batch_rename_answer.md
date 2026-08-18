# 练习 A07 答案讲解：批量重命名

> **用法**：卡住时再看本页。每一处 TODO 按三步走——先看"练习版缺什么"，再看"答案版填了什么"，最后回查原理文档对应小节。
> **作业范围**：本练习只需动 2 个文件：`src-tauri/src/lib.rs`（后端）与 `src/App.tsx`（前端 React），共 9 处 TODO。

## 对照总览

| 文件 | 练习版状态 | 你缺的 |
|---|---|---|
| `src-tauri/src/lib.rs` | 类型、`list_files` 骨架已给 | 替换逻辑、遍历、预览、spawn+进度、注册 |
| `src/App.tsx` | 界面已给 | `Channel`/`open` 导入、选文件夹、预览、进度+执行 |

## lib.rs TODO 1：build_new_name

### 答案版填了什么

```rust
fn build_new_name(file_name: &str, find: &str, replace: &str) -> String {
    if find.is_empty() {
        file_name.to_string()
    } else {
        file_name.replace(find, replace)
    }
}
```

### 为什么

- `find` 为空时原样返回（否则 `replace("", ...)` 会在每个字符间插入，行为诡异）
- `file_name.replace(find, replace)`：替换全部出现，如 `IMG_001.jpg` + `IMG_`→`photo_` = `photo_001.jpg`

## lib.rs TODO 2：list_files

### 答案版填了什么

```rust
let mut files = Vec::new();
for entry in std::fs::read_dir(dir).map_err(|e| format!("打开目录失败：{e}"))? {
    let entry = entry.map_err(|e| format!("读取目录项失败：{e}"))?;
    let path = entry.path();
    if path.is_file() {
        files.push(path);
    }
}
files.sort();
Ok(files)
```

### 为什么

- `std::fs::read_dir` 每一步都返回 `Result`，`map_err` + `?`
- 只收 `is_file()`（跳过子目录，不递归）
- `sort()` 保证处理顺序稳定（进度条的数字有规律）

## lib.rs TODO 3：preview_rename

### 答案版填了什么

```rust
let mut items = Vec::new();
for path in list_files(&dir)? {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        let new_name = build_new_name(file_name, &find, &replace);
        if new_name != file_name {
            items.push(PreviewItem { old: file_name.to_string(), new: new_name });
        }
    }
}
Ok(items)
```

### 为什么

- 预览**只计算、不改文件**——这是"先看清再执行"的安全设计
- 只收集 `new_name != file_name`（没变化的文件不需要出现在清单里）
- `path.file_name()` 返回 `Option<&OsStr>`，`to_str()` 转 UTF-8，失败跳过

## lib.rs TODO 4：run_rename — spawn + 进度

### 答案版填了什么

```rust
tauri::async_runtime::spawn(async move {
    let mut done = 0u64;
    for path in files {
        let file_name = path.file_name().and_then(|n| n.to_str()).map(String::from).unwrap_or_default();
        let new_name = build_new_name(&file_name, &find, &replace);
        if new_name != file_name {
            if let Some(parent) = path.parent() {
                let new_path = parent.join(&new_name);
                let _ = std::fs::rename(&path, &new_path);
            }
        }
        done += 1;
        let _ = on_progress.send(RenameProgress {
            done, total, current: new_name, finished: done == total,
        });
    }
});
Ok(())
```

### 为什么

- `async_runtime::spawn`：任务丢到后台，**命令立刻返回 `Ok(())`**——前端不卡
- `std::fs::rename(&path, parent.join(new_name))`：只改名不挪目录
- **每处理完一个文件就 `send` 一次进度**：`done/total` 给进度条，`current` 给"正在处理 xxx"，最后一个 `finished: true`
- `on_progress`（Channel）move 进任务，从后台线程往前端推——这就是 Channel 的意义（点对点、跨线程、多次）

### 回查文档

[第 1 节：Channel](a07_batch_rename.md#sec-a07-channel)、[第 2 节：后台任务](a07_batch_rename.md#sec-a07-spawn)。

## lib.rs TODO 5：注册插件 + 命令

```rust
.plugin(tauri_plugin_dialog::init())
.invoke_handler(tauri::generate_handler![preview_rename, run_rename])
```

dialog 插件不注册，前端 `open()` 调不到；命令老规矩必须登记。

## App.tsx TODO 1-2：导入 + 选文件夹

### 答案版填了什么

```typescript
import { invoke, Channel } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
// ...
const selected = await open({ directory: true });
if (typeof selected === "string") {
  setDir(selected);
  setStatus(`已选择: ${selected}`);
}
```

### 为什么

- `open({ directory: true })`：系统目录选择器；取消返回 `null`（`typeof` 判断兜底）
- `Channel` 和 `invoke` 都来自 `@tauri-apps/api/core`

### 回查文档

[第 3 节：dialog 插件](a07_batch_rename.md#sec-a07-dialog)。

## App.tsx TODO 3-4：预览 + 进度

### 答案版填了什么

```typescript
const items = await invoke<PreviewItem[]>("preview_rename", { dir, find, replace });
setPreview(items);

// 进度
const channel = new Channel<RenameProgress>();
channel.onmessage = (m) => {
  setProgress(m);
  if (m.finished) setStatus(`完成：共处理 ${m.total} 个文件`);
};
await invoke("run_rename", { dir, find, replace, onProgress: channel });
```

### 为什么

- **`onProgress: channel` 是 invoke 的参数**——后端 `on_progress: Channel<RenameProgress>` 就拿到了这根管道
- `channel.onmessage`：每收到一条进度更新一次 state，React 自动重渲染进度条
- 预览先执行：`preview.length === 0` 时"开始重命名"按钮禁用（UI 上防误操作）

### 回查文档

[第 1 节：Channel 前端用法](a07_batch_rename.md#sec-a07-channel)、[第 4 节：预览→执行](a07_batch_rename.md#sec-a07-preview)。

## 验收标准

```bash
cd 02_mini_apps/a07_batch_rename
cargo tauri dev
```

选文件夹 → 预览显示"旧名 → 新名"清单 → 开始重命名 → 进度条推进、显示"正在处理 xxx" → 完成提示。到文件夹确认文件名已改。

**破坏性验证**（确认你是理解了，而不是碰巧对）：

- 把 `invoke("run_rename", { ..., onProgress: channel })` 里的 `onProgress` 删掉 → 后端参数缺失报错（验证 Channel 走 invoke 参数）
- 把 `spawn` 换成命令里直接循环 → 点"开始"按钮界面卡住直到全部改完（验证后台任务）
- 把 `new_name != file_name` 的判断删掉 → 没变化的文件也被 `rename`（虽然同名 rename 无害，但预览会显示所有文件）（验证过滤）
- 把 `preview.length === 0` 的 disabled 删掉 → 没预览也能点执行（验证预览门禁）

## 升级挑战（可选）

- 加"撤销"：执行前把 `(旧路径, 新路径)` 存下来，提供 `undo_rename` 命令反着改回去
- 支持正则替换：把 `replace` 换成正则匹配（用 Rust 的 `regex` crate）
