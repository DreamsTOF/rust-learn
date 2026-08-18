# 练习 A04: 图片查看器（练习版）

**目标：** 把图片（或整个图片文件夹）拖进窗口，就能一张张翻看。本课学习三种"桌面感"能力：**拖放**（`onDragDropEvent`）、**静态资源**（asset 协议 + `convertFileSrc`）、**窗口操作**（缩放/置顶/全屏/居中）。

**新增知识：** 拖放事件、`assetProtocol` + `convertFileSrc`、窗口 API（`setSize` / `setAlwaysOnTop` / `setFullscreen` / `center`）。

**TODO（共 11 处）：**

- `src-tauri/src/lib.rs`（7 处）
  - 步骤 1：`is_image` 判断扩展名
  - 步骤 2：`list_images` 遍历目录收图片
  - 步骤 3：排序后返回
  - 步骤 4：登记命令
- `src/App.tsx`（4 处）
  - 步骤 1：导入 `invoke`
  - 步骤 2：拖放处理里支持"拖文件夹"（调 `list_images`）
  - 步骤 3：置顶按钮调 `setAlwaysOnTop`
  - 步骤 4：全屏按钮调 `setFullscreen`

**注意：** 本课改了三个配置文件（练习版已配好，不用动）——`tauri.conf.json` 开了 `assetProtocol`、`Cargo.toml` 给 `tauri` 加了 `protocol-asset` feature、capabilities 加了窗口操作权限。

**卡住了？** 先看练习讲解，再对照答案讲解：`tauri-learn-book-v2/src/02_mini_apps/`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1428
- identifier: com.taurilearn.a04
