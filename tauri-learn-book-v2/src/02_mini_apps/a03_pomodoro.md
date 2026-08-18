# 练习 A03: 番茄钟

## 为什么要学这个

到目前为止，所有命令都是"点了立刻返回"。但真实的桌面应用大量场景是**长时间任务**——倒计时、下载、后台同步。这一章要回答三个问题：

1. **为什么倒计时不能让界面卡死？** —— `async` 命令和 `tokio::time::sleep` 怎么让"等待"发生在后台？
2. **时间到了怎么提醒用户？** —— 系统通知（notification 插件）怎么发？
3. **关窗口就等于退出吗？** —— 番茄钟这种"后台也要存在"的应用，怎么做到"关窗隐藏、托盘恢复"？

学完你会发现：**界面永远不卡，是"任务放后台 + 进度推给前端"** 这套组合做到的。

---

## 从问题出发

练习 A03 要做的事：**一个番茄钟——开始倒计时 25 分钟，每秒刷新剩余时间，到点弹系统通知；关掉窗口应用还在（托盘里），随时能恢复。**

**核心矛盾：** 倒计时是一个"长时间运行"的任务。如果它跑在前端，页面一刷新计时就没了；如果它跑在后端同步命令里，命令执行期间整个 UI 都会卡住。所以倒计时必须是一个 **`async` 后端命令**——它在后台"睡着"（`sleep`），每秒醒一次，把剩余秒数 `emit` 给前端，然后继续睡。

```text
前端 (React)                        Rust 进程（后台）
┌──────────────────────┐           ┌──────────────────────────────┐
│ 显示剩余时间 (listen) │ ◄──tick── │ async start_pomodoro          │
│ 开始/停止按钮 (invoke)│ ────────► │  for ... in (1..=total).rev() │
│ 到点提示 (listen)     │ ◄──done── │    emit tick; sleep(1s).await │
└──────────────────────┘           │  通知插件 → 系统通知           │
                                   └──────────────────────────────┘
```

本课新增四块：**`async` 命令**（后台跑）、**`emit` 推进度**（沿用 A01 的事件）、**通知插件**（到点提醒）、**托盘 + 关闭拦截**（后台常驻）。

---

<a id="sec-a03-async"></a>
## 1. `async fn` 命令 + `tokio::time::sleep` — 在后台"等待"

### 同步命令为什么不行

```rust
#[tauri::command]
fn start_pomodoro(minutes: u64) -> String {
    let total = minutes * 60;
    std::thread::sleep(Duration::from_secs(total)); // 界面卡死 total 秒！
    "done".into()
}
```

如果命令里 `sleep`，它会**阻塞命令所在的线程**——Tauri 处理 invoke 的线程被占住，其他所有请求（包括刷新界面）都在排队，窗口直接无响应。

### async 命令：等待时让出线程

```rust
#[tauri::command]
async fn start_pomodoro(...) -> Result<(), String> {
    for remaining in (1..=total).rev() {
        let _ = app.emit("pomodoro-tick", remaining);
        tokio::time::sleep(Duration::from_secs(1)).await; // 关键
    }
    Ok(())
}
```

- `async fn`：命令可以 `await` 了。`tokio::time::sleep(...).await` 的意思是"**先睡 1 秒，这段时间线程空闲，可以去干别的**"——Tauri 的运行时就是 tokio，async 命令就跑在它的线程池上
- 等到点醒来，继续执行循环，再 emit 下一次 tick
- **命令"看起来"是连续跑的，实际上线程一直在被复用**——这就是"不卡界面"的原理

> **关键理解：** `async` 不等于"快"，而是"**等待时不占着线程**"。`sleep().await` 把控制权交还运行时，别的任务插进来执行，到时间了再回来继续。

<a id="sec-a03-tick"></a>
## 2. 用事件推进度 — 命令的返回值给不了"中间过程"

倒计时的**中间状态**（剩余 1499 秒、1498 秒……）怎么给前端？命令只能返回一次结果（结束时）。所以中间过程走**事件**——沿用 A01 的 `emit` / `listen`：

```rust
// 后端：每秒推一次剩余秒数
let _ = app.emit("pomodoro-tick", remaining);

// 前端：收到就更新显示
listen<number>("pomodoro-tick", (e) => setRemaining(e.payload));
```

- `emit` 的载荷是 `u64`（剩余秒数），前端 `listen<number>` 收
- 事件名三个：`pomodoro-tick`（每秒进度）、`pomodoro-done`（正常结束）、`pomodoro-stopped`（被停止）

### 中途停止：AtomicBool

倒计时一旦开始，`invoke` 的返回要等它跑完。想中途停，得有个"开关"让循环自己停下来：

```rust
#[derive(Default)]
struct PomodoroState {
    running: Mutex<bool>,          // 防止重复开始（沿用 A01 的 Mutex）
    stop_requested: AtomicBool,    // 停止信号：无需锁的布尔
}

// 循环里每秒检查一次
if state.stop_requested.load(Ordering::SeqCst) {
    break; // 被要求停止，跳出循环
}

// 停止命令：把开关置位
#[tauri::command]
fn stop_pomodoro(state: tauri::State<'_, PomodoroState>) {
    state.stop_requested.store(true, Ordering::SeqCst);
}
```

- `AtomicBool`：跨线程共享的布尔，`store`/`load` 不需要 `Mutex`（单值原子操作）
- **优雅停止**：不是杀掉线程，而是让任务自己发现"该停了"，在安全的点退出

<a id="sec-a03-notify"></a>
## 3. 通知插件 — 到点弹系统通知

时间到，应用可能在后台、窗口被隐藏——光靠前端显示不够，要发**系统通知**（托盘上方弹出的消息）。这是 `tauri-plugin-notification` 的活。

**三步接入：**

```rust
// ① 注册插件
tauri::Builder::default().plugin(tauri_plugin_notification::init())

// ② capabilities 加权限（练习版已配好）
"permissions": ["core:default", "notification:default"]

// ③ 发通知
use tauri_plugin_notification::NotificationExt;

app.notification()
    .builder()
    .title("番茄钟")
    .body("时间到！休息一下吧。")
    .show()
    .map_err(|e| format!("通知失败：{e}"))?;
```

- `NotificationExt` trait 给 `AppHandle` 提供 `notification()` 方法
- `.builder().title().body().show()`：链式描述一条通知
- 权限 `notification:default` 是插件的最小权限集——**插件 API 走权限系统，自己写的命令不需要**（这点和 A01 相反）

<a id="sec-a03-tray"></a>
## 4. 系统托盘 — 关窗不退出，图标常驻

番茄钟的特点是"后台也要存在"。用户点了关闭按钮，我们不退出，而是**隐藏窗口、留下托盘图标**。

### 创建托盘（在 `setup` 钩子里）

> **前置：** 托盘依赖 `tauri` crate 的 `tray-icon` feature——练习版的 `Cargo.toml` 已配好（`tauri = { workspace = true, features = ["tray-icon"] }`）。不加的话 `use tauri::tray` 直接编译报错。

```rust
.setup(|app| {
    let show_item = MenuItem::with_id(app, "toggle", "显示/隐藏", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone()) // 用应用图标
        .menu(&menu)                                     // 右键菜单
        .show_menu_on_left_click(false)                  // 左键不弹菜单（用来显示窗口）
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle" => { /* 显示/隐藏窗口 */ }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            // 左键单击 → 显示窗口
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
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

- **`setup` 钩子**：应用启动、窗口创建后执行一次。托盘、后台服务这类"一次性初始化"放这里（比在 `main` 里拿不到 `AppHandle`）
- `MenuItem::with_id(app, id, 文字, 是否可用, 快捷键)`：菜单项，`id` 是程序里找它的钥匙
- `on_menu_event`：菜单被点击 → 按 `event.id` 分发
- `on_tray_icon_event`：托盘图标被点击 → `TrayIconEvent::Click`（注意区分左右键和按下/抬起）
- `app.get_webview_window("main")`：按 label 拿到窗口（label 在 `tauri.conf.json` 里配置）

<a id="sec-a03-close"></a>
## 5. 关闭拦截 — 点 × 是"隐藏"不是"退出"

托盘建好了，还差最后一步：**让关闭按钮变成隐藏**。

```rust
.on_window_event(|window, event| {
    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();  // 告诉框架"这次关闭我不批准"
        let _ = window.hide(); // 改成隐藏
    }
})
```

- `on_window_event`：窗口产生事件（移动、缩放、关闭请求……）时回调
- `CloseRequested`：用户点了关闭按钮。**默认行为是退出应用**；调用 `api.prevent_close()` 取消退出
- 退出只能走托盘菜单的"退出"——这是"托盘常驻应用"的经典交互

> **对比：** 在 `run()` 里 `.on_window_event(...)` 是**全局窗口事件**；每个窗口也可以用 `WebviewWindowBuilder` 单独注册。这里只有主窗口，用全局的即可。

---

## 练习指引

**作业范围：** 动 2 个文件，共 10 处 TODO。

| 文件 | 步骤 | 内容 |
|------|------|------|
| `src-tauri/src/lib.rs` | 1 | 倒计时循环：每秒 `emit("pomodoro-tick", remaining)` + `sleep(1s).await`，`stop_requested` 打断 |
| `src-tauri/src/lib.rs` | 2 | `app.notification().builder()...show()` 发"时间到"通知 |
| `src-tauri/src/lib.rs` | 3 | `.plugin(tauri_plugin_notification::init())` + `.manage(PomodoroState::default())` |
| `src-tauri/src/lib.rs` | 4 | `setup` 里建托盘（菜单 + 左键点击显示窗口） |
| `src-tauri/src/lib.rs` | 5 | `CloseRequested` → `prevent_close()` + `hide()` |
| `src-tauri/src/lib.rs` | 6 | 登记两个命令 |
| `src/App.tsx` | 1-4 | 导入、`listen` 三事件、start/stop 的 invoke |

**怎么验证：**

```bash
cd 02_mini_apps/a03_pomodoro
cargo tauri dev
```

点"工作 25 分"，大数字每秒跳动；点"停止"立即停止并显示"已停止"。点窗口 × → 窗口消失，托盘出现图标；左键点图标 → 窗口回来；右键菜单"退出"才真正退出。等满一个周期（可以先把 `minutes` 改成 1 试）→ 系统弹通知。

**故意踩坑看效果：** 把 `async` 去掉（改成同步）→ `sleep` 编译报错或窗口卡死；把 `prevent_close` 删掉 → 点 × 应用直接退出。

---

## 知识点连起来看

```text
async fn 命令 + sleep().await     ← 后台：等待时不占线程
        │
app.emit("pomodoro-tick", n)     ← 进度：中间状态走事件
        │
AtomicBool + Mutex               ← 控制：running 防重入，stop 优雅停
        │
notification 插件                 ← 提醒：系统通知（需权限）
        │
TrayIconBuilder + on_window_event ← 常驻：关窗隐藏、托盘恢复
```

| 层 | 解决的问题 | 关键概念 |
|----|-----------|---------|
| 后台 | 长任务不卡界面 | `async fn`、`tokio::time::sleep().await` |
| 进度 | 中间状态怎么给前端 | `emit` / `listen`（沿用 A01） |
| 控制 | 怎么安全地开始/停止 | `Mutex`、`AtomicBool` |
| 提醒 | 到点怎么喊用户 | notification 插件、`notification:default` |
| 常驻 | 关窗后应用还在 | 托盘、`CloseRequested` + `prevent_close` |

**一通百通的核心：** 这一课把"**后台 + 推送 + 常驻**"三件套补齐了。凡是"长时间运行的应用"（下载器、监视器、音乐播放器）都是这套骨架。超级项目 P10 自动保存、P15 托盘，直接复用本课代码。

**递进关系：** 练习 A04（图片查看器）转向"**桌面感**"的另一面——拖放文件进窗口、在界面里显示本地图片、直接操控窗口大小/置顶/全屏。
