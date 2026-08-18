# 练习 A04: 图片查看器（答案版）

**目标：** 把图片（或整个图片文件夹）拖进窗口，就能一张张翻看。本课学习三种"桌面感"能力：**拖放**（`onDragDropEvent`）、**静态资源**（asset 协议 + `convertFileSrc`）、**窗口操作**（缩放/置顶/全屏/居中）。

**对照练习版补全的内容：**

- `src-tauri/src/lib.rs`
  - `is_image`：扩展名 + `matches!` 判断
  - `list_images`：`std::fs::read_dir` 遍历 + 过滤 + `sort()`
  - `generate_handler![list_images]`
- `src/App.tsx`
  - 拖放处理：图片直接收；非图片路径当文件夹 `invoke("list_images", { dir })` 列出
  - `toggleAlwaysOnTop` / `toggleFullscreen`：`getCurrentWindow().setAlwaysOnTop/setFullscreen`

**配置文件（练习版已配好）：** `tauri.conf.json` 的 `assetProtocol`、`Cargo.toml` 的 `protocol-asset` feature、capabilities 的窗口操作权限。

**完整讲解见：** `tauri-learn-book-v2/src/02_mini_apps/a04_image_viewer_answer.md`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1429
- identifier: com.taurilearn.a04a
