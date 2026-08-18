# 练习 A03 答案讲解：番茄钟

> **用法**：卡住时再看本页。每一处 TODO 按三步走——先看"练习版缺什么"，再看"答案版填了什么"，最后回查原理文档对应小节。
> **作业范围**：本练习只需动 2 个文件：`src-tauri/src/lib.rs`（后端）与 `src/App.tsx`（前端 React），共 10 处 TODO。

## 对照总览

| 文件 | 练习版状态 | 你缺的 |
|---|---|---|
| `src-tauri/src/lib.rs` | 状态、停止命令、框架已给 | 倒计时循环、通知、插件/状态注册、托盘、关闭拦截、命令登记 |
| `src/App.tsx` | 界面、格式化、按钮已给 | `listen` 三事件 + start/stop 的 `invoke` |

## lib.rs TODO 1：倒计时循环

### 练习版这里是什么

```rust
let total = minutes * 60;
// TODO: 每秒 emit tick + sleep 1s，被 stop_requested 打断就 break
let _ = total; // 占位：完成后删除
```

### 答案版填了什么

```rust
let total = minutes * 60;
for remaining in (1..=total).rev() {
    if state.stop_requested.load(Ordering::SeqCst) {
        break;
    }
    let _ = app.emit("pomodoro-tick", remaining);
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### 为什么

- `(1..=total).rev()`：从 `total` 倒数到 1，每秒一轮
- **先 emit 再 sleep**：这一秒的剩余值立刻送到前端，然后才开始等
- `stop_requested.load(...)` 每秒检查一次停止信号，被打断就 `break`
- `sleep(...).await`：把线程让出去，不阻塞其他请求——**这是"不卡界面"的关键**
- 循环结束后 `*state.running.lock().unwrap() = false;` 复位，无论正常结束还是被打断

### 回查文档

[第 1 节：async 命令 + tokio sleep](a03_pomodoro.md#sec-a03-async)、[第 2 节：事件推进度](a03_pomodoro.md#sec-a03-tick)。

## lib.rs TODO 2：发系统通知

### 练习版这里是什么

```rust
let _ = app.emit("pomodoro-done", ());
// TODO: app.notification().builder()...show()
Ok(())
```

### 答案版填了什么

```rust
let _ = app.emit("pomodoro-done", ());
app.notification()
    .builder()
    .title("番茄钟")
    .body("时间到！休息一下吧。")
    .show()
    .map_err(|e| format!("通知失败：{e}"))?;
Ok(())
```

### 为什么

- `app.notification()` 来自 `NotificationExt` trait（练习版已 import）
- `.builder().title().body().show()`：链式描述并弹出通知
- `.show()` 返回 `Result`，`map_err` 成人话后 `?` 传播——通知失败也要告诉前端
- 前提：插件已注册（TODO 3）+ capabilities 有 `notification:default`（已配好）

### 回查文档

[第 3 节：通知插件](a03_pomodoro.md#sec-a03-notify)。

## lib.rs TODO 3：注册插件 + 状态

### 练习版这里是什么

```rust
tauri::Builder::default()
    // TODO: .plugin(...) + .manage(...)
    .setup(...)
```

### 答案版填了什么

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_notification::init())
    .manage(PomodoroState::default())
```

### 为什么

- 通知插件不注册，`app.notification()` 直接 panic
- `.manage` 沿用 A01：命令里的 `State<'_, PomodoroState>` 才拿得到状态

### 回查文档

[第 3 节：通知插件三步接入](a03_pomodoro.md#sec-a03-notify)、[A01 第 1 节：manage + State](../02_mini_apps/a01_todo.md#sec-a01-manage)。

## lib.rs TODO 4：系统托盘

### 练习版这里是什么

```rust
.setup(|app| {
    // TODO: 创建托盘（菜单 + 左键点击显示窗口）
    Ok(())
})
```

### 答案版填了什么

```rust
.setup(|app| {
    let show_item = MenuItem::with_id(app, "toggle", "显示/隐藏", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle" => {
                if let Some(win) = app.get_webview_window("main") {
                    if win.is_visible().unwrap_or(false) { let _ = win.hide(); }
                    else { let _ = win.show(); let _ = win.set_focus(); }
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left, button_state: MouseButtonState::Up, ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.show();
                    let _ = win.set_focus();
                }
            }
        })
        .build(app)?;
    Ok(())
})
```

### 为什么

- `MenuItem::with_id(app, id, 文字, enabled, 快捷键)`：`id` 是程序里分发用的钥匙
- `show_menu_on_left_click(false)`：左键点击留给"显示窗口"（`on_tray_icon_event`），右键才弹菜单
- `on_menu_event` 按 `event.id.as_ref()` 匹配 `"toggle"` / `"quit"`
- `TrayIconEvent::Click { button, button_state, .. }`：模式匹配出"左键抬起"
- `app.get_webview_window("main")`：按 label 找窗口；`"main"` 来自 `tauri.conf.json` 的窗口配置
- `let _tray = ...`：绑定托盘句柄（防止被当作未使用变量）

### 回查文档

[第 4 节：系统托盘](a03_pomodoro.md#sec-a03-tray)。

## lib.rs TODO 5：关闭拦截

### 练习版这里是什么

```rust
.on_window_event(|window, event| {
    // TODO: CloseRequested → prevent_close + hide
})
```

### 答案版填了什么

```rust
.on_window_event(|window, event| {
    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        let _ = window.hide();
    }
})
```

### 为什么

- `CloseRequested` 是"用户请求关闭"，默认直接退出应用
- `api.prevent_close()` 取消默认退出；`window.hide()` 改成隐藏
- 退出唯一入口：托盘菜单"退出"（`app.exit(0)`）

### 回查文档

[第 5 节：关闭拦截](a03_pomodoro.md#sec-a03-close)。

## lib.rs TODO 6：登记命令

```rust
.invoke_handler(tauri::generate_handler![start_pomodoro, stop_pomodoro])
```

同前几课：命令必须登记，前端按名字找。

## App.tsx TODO 1-4：listen + invoke

### 答案版填了什么

```tsx
useEffect(() => {
  const unTick = listen<number>("pomodoro-tick", (e) => setRemaining(e.payload));
  const unDone = listen("pomodoro-done", () => { setRunning(false); setStatus("时间到！"); });
  const unStop = listen("pomodoro-stopped", () => { setRunning(false); setStatus("已停止"); });
  return () => {
    unTick.then((f) => f());
    unDone.then((f) => f());
    unStop.then((f) => f());
  };
}, []);

async function start(minutes: number) {
  setRunning(true);
  setStatus("计时中…");
  setRemaining(minutes * 60);
  try {
    await invoke("start_pomodoro", { minutes });
  } catch (e) { setRunning(false); setStatus(`启动失败: ${e}`); }
}

async function stop() {
  await invoke("stop_pomodoro");
}
```

### 为什么

- `listen` 返回 **Promise\<UnlistenFn\>**：`.then((f) => f())` 拿到并调用取消函数——**React StrictMode 在开发模式会挂载→卸载→再挂载，不清理会注册两次监听、tick 双倍触发**（这是本课最容易踩的 React 坑）
- `start` 的 `await invoke(...)` 会一直等到倒计时结束才 resolve——期间按钮已被 `disabled` 锁住
- `pomodoro-done` / `pomodoro-stopped` 事件负责把 `running` 复位回 `false`

### 回查文档

[第 2 节：事件推进度](a03_pomodoro.md#sec-a03-tick)。

## 验收标准

```bash
cd 02_mini_apps/a03_pomodoro
cargo tauri dev
```

点"工作 25 分"→ 大数字每秒递减；点"停止"→ 立即停止、显示"已停止"。点窗口 × → 窗口隐藏、托盘有图标；左键点托盘图标 → 窗口恢复；右键"退出"→ 真正退出。把 `minutes` 改成 1 测试：倒计时结束后弹出系统通知。

**破坏性验证**（确认你是理解了，而不是碰巧对）：

- 把命令改成同步 `fn` + `std::thread::sleep` → 界面卡死（验证 async 的意义）
- 删掉 `api.prevent_close()` → 点 × 应用直接退出（验证关闭拦截）
- 删掉 `unTick.then((f) => f())` 清理 → 开发模式 HMR 后 tick 变双倍（验证 listen 必须清理）
- 把 `stop_requested.load` 的检查删掉 → 点"停止"没反应（验证优雅停止的原理）

## 升级挑战（可选）

- 加"连续循环"：完成一阶段自动开始休息 5 分钟（复用 start 命令）
- 托盘菜单加"开始 25 分钟"快捷项（不用打开窗口就能启动）
