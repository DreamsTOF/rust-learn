# 练习 A03: 番茄钟（答案版）

**目标：** 做一个 25 分钟倒计时的番茄钟。倒计时跑在**后端**（`async fn` 命令 + `tokio::time::sleep`），每秒把剩余秒数 `emit` 给前端；到点发**系统通知**；关窗口不是退出而是**隐藏到托盘**。

**对照练习版补全的内容：**

- `src-tauri/src/lib.rs`
  - 步骤 1：`for remaining in (1..=total).rev() { ... app.emit("pomodoro-tick", remaining) ... tokio::time::sleep(1s).await }`
  - 步骤 2：`app.notification().builder().title(...).body(...).show()`
  - 步骤 3：`.plugin(tauri_plugin_notification::init())` + `.manage(...)`
  - 步骤 4：`setup` 里 `TrayIconBuilder` + 菜单（显示/隐藏、退出）+ 左键点击显示窗口
  - 步骤 5：`on_window_event` 拦截 `CloseRequested` → `prevent_close()` + `hide()`
  - 步骤 6：`generate_handler![start_pomodoro, stop_pomodoro]`
- `src/App.tsx`
  - `listen` 三个事件更新 state（注意返回的 unlisten 清理）
  - `start` / `stop` 里 `await invoke(...)`

**完整讲解见：** `tauri-learn-book-v2/src/02_mini_apps/a03_pomodoro_answer.md`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1427
- identifier: com.taurilearn.a03a
