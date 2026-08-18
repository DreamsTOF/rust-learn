# 练习 A02: 记事本（练习版）

**目标：** 做一个"关掉软件字还在"的记事本。内容存成**文件**（`app_data_dir` 下的 `note.txt`），用**fs 插件**读写，前后端都带 `Result` 错误处理。本课起前端改用 **React**。

**新增知识：** 路径 API（`app.path().app_data_dir()`）、fs 插件读/写（`app.fs().open` + `OpenOptions`）、`Result<T, String>` 错误处理、React（`useState` + `invoke`）。

**TODO（共 10 处）：**

- `src-tauri/src/lib.rs`（6 处）
  - 步骤 1：`load_note` 用 fs 插件打开文件并读成字符串
  - 步骤 2：`save_note` 用 fs 插件以"可写 + 创建 + 清空"打开并写入
  - 步骤 3：注册 fs 插件
  - 步骤 4：登记三个命令
- `src/App.tsx`（4 处）
  - 步骤 1：导入 `invoke`
  - 步骤 2：启动时获取文件路径
  - 步骤 3：启动时读取内容
  - 步骤 4：保存按钮调 `save_note`

**卡住了？** 先看练习讲解，再对照答案讲解：`tauri-learn-book-v2/src/02_mini_apps/`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1424
- identifier: com.taurilearn.a02
