# 练习 A08: 剪贴板历史（练习版）

**目标：** 后台盯着剪贴板，复制过的文字都记下来；点历史项就复制回剪贴板；按 **Ctrl+Shift+V** 呼出/隐藏窗口。历史用 **Store** 持久化，重启还在。

**新增知识：** 剪贴板插件（`app.clipboard().read_text/write_text`）、全局快捷键（`global-shortcut` 插件 + `register`）、应用菜单（`Menu` / `MenuItem` / `on_menu_event`）。

**TODO（共 9 处）：**

- `src-tauri/src/lib.rs`（5 处）
  - 步骤 1：`push_history` 去重 + 插最前 + 限长
  - 步骤 2：`copy_text` 里 `write_text` 写回剪贴板
  - 步骤 3：监控循环里"内容变了才记入历史"
  - 步骤 4：注册全局快捷键
  - 步骤 5：登记三个命令
- `src/App.tsx`（4 处）
  - 步骤 1：导入 `invoke`
  - 步骤 2：启动时拉取历史
  - 步骤 3：点击历史项调 `copy_text`
  - 步骤 4：清空调 `clear_history`

**配置文件（练习版已配好）：** 三个插件依赖（clipboard / global-shortcut / store）。Rust 端调用插件 API 不需要额外权限。

**卡住了？** 先看练习讲解，再对照答案讲解：`tauri-learn-book-v2/src/02_mini_apps/`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1436
- identifier: com.taurilearn.a08
