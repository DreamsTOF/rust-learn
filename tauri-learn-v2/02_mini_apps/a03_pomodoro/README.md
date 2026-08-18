# 练习 A03: 番茄钟（练习版）

**目标：** 做一个 25 分钟倒计时的番茄钟。倒计时跑在**后端**（`async fn` 命令 + `tokio::time::sleep`），每秒把剩余秒数 `emit` 给前端；到点发**系统通知**；关窗口不是退出而是**隐藏到托盘**。

**新增知识：** `async fn` 命令 + `tokio::time::sleep`（不卡界面）、通知插件（`notification`）、系统托盘 + 隐藏窗口（`TrayIconBuilder` / `CloseRequested`，依赖 `tray-icon` feature）。

**配置文件（练习版已配好）：** capabilities 加 `notification:default`；`Cargo.toml` 的 `tauri` 加 `tray-icon` feature。

**TODO（共 10 处）：**

- `src-tauri/src/lib.rs`（6 处）
  - 步骤 1：倒计时循环（每秒 emit tick + sleep）
  - 步骤 2：`app.notification().builder()...show()` 发通知
  - 步骤 3：注册通知插件 + `.manage(PomodoroState)`
  - 步骤 4：托盘（菜单 + 左键点击显示窗口）
  - 步骤 5：`CloseRequested` 时 `prevent_close()` + `hide()`
  - 步骤 6：登记两个命令
- `src/App.tsx`（4 处）
  - 步骤 1：导入 `invoke` / `listen`
  - 步骤 2：`listen` 三个事件（tick / done / stopped）
  - 步骤 3：`start` 里调 `start_pomodoro`
  - 步骤 4：`stop` 里调 `stop_pomodoro`

**卡住了？** 先看练习讲解，再对照答案讲解：`tauri-learn-book-v2/src/02_mini_apps/`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1426
- identifier: com.taurilearn.a03
