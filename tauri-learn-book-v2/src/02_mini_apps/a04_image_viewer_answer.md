# 练习 A04 答案讲解：图片查看器

> **用法**：卡住时再看本页。每一处 TODO 按三步走——先看"练习版缺什么"，再看"答案版填了什么"，最后回查原理文档对应小节。
> **作业范围**：本练习只需动 2 个文件：`src-tauri/src/lib.rs`（后端）与 `src/App.tsx`（前端 React），共 11 处 TODO。配置文件已配好，不用动。

## 对照总览

| 文件 | 练习版状态 | 你缺的 |
|---|---|---|
| `src-tauri/src/lib.rs` | 命令框架已给 | `is_image`、`list_images` 遍历、排序、登记 |
| `src/App.tsx` | 拖图片、翻页、放大缩小已给 | `invoke` 导入、拖文件夹分支、置顶/全屏 |

## lib.rs TODO 1：is_image

### 练习版这里是什么

```rust
fn is_image(path: &Path) -> bool {
    // TODO: 扩展名判断
    false // ← 替换成你的代码
}
```

### 答案版填了什么

```rust
fn is_image(path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase());
    matches!(ext.as_deref(), Some("png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp"))
}
```

### 为什么

- `path.extension()` 返回 `Option<&OsStr>`，`to_str()` 转 `&str`（非 UTF-8 文件名会失败，返回 `None` 安全跳过）
- `to_ascii_lowercase()`：`.PNG` 也要能识别
- `matches!(ext.as_deref(), Some("png" | "jpg" | ...))`：`Option<&str>` 与一组字面量匹配，一行搞定

### 回查文档

[第 4 节：列目录](a04_image_viewer.md#sec-a04-list)。

## lib.rs TODO 2-3：list_images 遍历 + 排序

### 练习版这里是什么

```rust
#[tauri::command]
fn list_images(dir: String) -> Result<Vec<String>, String> {
    let mut paths = Vec::new();
    // TODO: 遍历目录 + 过滤图片 + 排序
    Ok(paths) // ← 替换成你的代码
}
```

### 答案版填了什么

```rust
    for entry in std::fs::read_dir(&dir).map_err(|e| format!("打开目录失败：{e}"))? {
        let entry = entry.map_err(|e| format!("读取目录项失败：{e}"))?;
        let path = entry.path();
        if path.is_file() && is_image(&path) {
            paths.push(path.to_string_lossy().into_owned());
        }
    }
    paths.sort();
    Ok(paths)
```

### 为什么

- `std::fs::read_dir` 返回迭代器，每一步都可能失败（`Result<DirEntry>`），所以要 `map_err` + `?`
- `path.is_file()`：跳过子目录（不递归，保持简单）
- `is_image(&path)`：只收图片
- `to_string_lossy().into_owned()`：`PathBuf` → `String`（`into_owned` 是"把借用转成拥有的 String"）
- `paths.sort()`：文件名排序，翻页顺序稳定
- 返回 `Result<Vec<String>, String>`：目录打不开时给一句人话错误

### 回查文档

[第 4 节：列目录](a04_image_viewer.md#sec-a04-list)。

## App.tsx TODO 1：导入 invoke

```typescript
import { invoke } from "@tauri-apps/api/core";
```

同前几课——`invoke` 不导入就是未定义变量。

## App.tsx TODO 2：拖文件夹分支

### 练习版这里是什么

```typescript
setImages(paths.filter(isImagePath)); // ← 先实现"拖图片"
setIndex(0);
```

### 答案版填了什么

```typescript
const imagePaths: string[] = [];
for (const p of paths) {
  if (isImagePath(p)) {
    imagePaths.push(p);
  } else {
    // 不是图片 → 当作文件夹，交给后端列出其中的图片
    try {
      const listed = await invoke<string[]>("list_images", { dir: p });
      imagePaths.push(...listed);
    } catch (e) {
      setStatus(`不是图片也不是目录: ${p}`);
    }
  }
}
if (imagePaths.length > 0) {
  setImages(imagePaths);
  setIndex(0);
  setStatus(`${imagePaths.length} 张图片`);
}
```

### 为什么

- 遍历每个拖入路径：图片直接收；非图片当文件夹 `invoke("list_images", { dir: p })`
- 后端返回 `Vec<String>`（图片路径数组），`push(...listed)` 摊开追加
- 拖入的不是目录也会报错 → `.catch` 提示"不是图片也不是目录"
- 这个分支必须 `await`，所以拖放回调是 `async` 函数

### 回查文档

[第 1 节：拖放](a04_image_viewer.md#sec-a04-drag)、[第 4 节：列目录](a04_image_viewer.md#sec-a04-list)。

## App.tsx TODO 3-4：置顶 / 全屏

### 答案版填了什么

```typescript
async function toggleAlwaysOnTop() {
  const next = !alwaysOnTop;
  setAlwaysOnTop(next);                       // 先改 UI 状态
  await getCurrentWindow().setAlwaysOnTop(next); // 再调窗口 API
}

async function toggleFullscreen() {
  const next = !fullscreen;
  setFullscreen(next);
  await getCurrentWindow().setFullscreen(next);
}
```

### 为什么

- 注意"先改 React 状态、再调窗口 API"的顺序：按钮文字立刻切换，窗口行为随后生效
- 两个函数都有 TODO 前对应的权限（capabilities 里 `core:window:allow-set-always-on-top` / `allow-set-fullscreen`）——**没加权限，调用会被拒绝**（可在破坏性验证里试）

### 回查文档

[第 3 节：窗口操作](a04_image_viewer.md#sec-a04-window)。

## 验收标准

```bash
cd 02_mini_apps/a04_image_viewer
cargo tauri dev
```

拖图片 → 显示 + 底部缩略图；上一张/下一张翻页；拖文件夹 → 自动列出其中图片；放大/缩小窗口变化；置顶后窗口在最前；全屏/退出全屏切换；居中回到屏幕中间。

**破坏性验证**（确认你是理解了，而不是碰巧对）：

- 把 `tauri.conf.json` 的 `assetProtocol.enable` 改回 `false` → 图片空白（验证 asset 协议的必要性）
- 把 `src={convertFileSrc(current)}` 改成 `src={current}` → 图片加载失败（验证路径必须转 URL）
- 把 capabilities 里的 `core:window:allow-set-fullscreen` 删掉 → 点"全屏"报错（验证窗口操作要权限）
- 把 `paths.sort()` 删掉 → 翻页顺序不固定（验证排序的意义）

## 升级挑战（可选）

- 加方向键翻页：监听 `keydown`，左右键调上一张/下一张
- 加"适应窗口/实际大小"切换：`viewer` 的 `img` 尺寸在 `object-fit: contain` 与原始尺寸之间切换
