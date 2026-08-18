# 练习 A07: 批量重命名（练习版）

**目标：** 选一个文件夹，把里面的文件批量改名（如把所有 `IMG_` 换成 `photo_`）。**先预览再执行**，执行时用 **Channel** 把进度推给前端显示进度条。

**新增知识：** `Channel` 流式推送（`new Channel()` / `onmessage` / `on_progress.send`）、后台任务（`tauri::async_runtime::spawn`）、dialog 插件选文件夹（`open({ directory: true })`）。

**TODO（共 9 处）：**

- `src-tauri/src/lib.rs`（5 处）
  - 步骤 1：`build_new_name` 字符串替换
  - 步骤 2：`list_files` 遍历目录
  - 步骤 3：`preview_rename` 收集会改名的文件
  - 步骤 4：`run_rename` 里改名 + `on_progress.send` 推进度
  - 步骤 5：注册 dialog 插件 + 登记命令
- `src/App.tsx`（4 处）
  - 步骤 1：导入 `Channel` / `open`
  - 步骤 2：选文件夹
  - 步骤 3：预览
  - 步骤 4：Channel 进度 + 执行

**配置文件（练习版已配好）：** capabilities 加 `dialog:default`；`package.json` 加 `@tauri-apps/plugin-dialog`。

**卡住了？** 先看练习讲解，再对照答案讲解：`tauri-learn-book-v2/src/02_mini_apps/`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1434
- identifier: com.taurilearn.a07
