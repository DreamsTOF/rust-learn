# 练习 A08: 剪贴板历史

## 为什么要学这个

前七道小菜做的都是"常规应用"：界面 + 数据。但桌面工具还有一类更"系统级"的能力，这一章一次补齐三件：

1. **剪贴板**——读写系统剪贴板（`clipboard-manager` 插件）
2. **全局快捷键**——应用在后台也能被快捷键唤起（`global-shortcut` 插件）
3. **应用菜单**——窗口菜单栏（`tauri::menu`）

做成什么？**剪贴板历史**：你复制过的文字都被记下来，按快捷键呼出窗口，点一下历史项就能再复制。这是最典型的"后台常驻小工具"。

---

## 从问题出发

练习 A08 要做的事：**后台盯着剪贴板，复制过的文字都记下来；按 Ctrl+Shift+V 呼出窗口；点历史项复制回剪贴板；历史重启还在。**

**核心矛盾：** 剪贴板是**系统级资源**，应用不能"订阅它的变化"，只能**轮询**（定时读一遍，和上次比，变了就记）。而要"呼之即来"，又需要**全局快捷键**（应用窗口隐藏时，按键仍能被操作系统送到应用手里）。最后，历史要**持久化**（复用 A06 的 Store），否则重启全丢。

```text
系统剪贴板
    │ 后台轮询（每 800ms 读一次，变了才记）
    ▼
Rust: push_history（去重 + 插最前 + 截断 + 存 Store + emit）
    │
    ├─► 全局快捷键 Ctrl+Shift+V → 显示/隐藏窗口
    ├─► 应用菜单：清空历史 / 退出
    └─► 命令：get_history / copy_text / clear_history
```

---

<a id="sec-a08-clipboard"></a>
## 1. 剪贴板插件 — 读写系统剪贴板

```rust
tauri::Builder::default().plugin(tauri_plugin_clipboard_manager::init())
```

```rust
use tauri_plugin_clipboard_manager::ClipboardExt;

// 读：把系统剪贴板里的文字拿出来
let Ok(current) = app.clipboard().read_text() else { continue };

// 写：把文字放回剪贴板（相当于"复制"）
app.clipboard().write_text(text.clone()).map_err(|e| format!("写入剪贴板失败：{e}"))?;
```

- `app.clipboard()`：`ClipboardExt` trait 提供的访问器
- `read_text()`：读剪贴板文本，可能失败（剪贴板里是图片/文件时返回错误）
- `write_text(str)`：写入文本——点击历史项"复制回剪贴板"就是写一次
- Rust 端调用不需要权限（A02 的结论继续适用）

<a id="sec-a08-watch"></a>
## 2. 后台监控 — 轮询 + 去重

剪贴板没有"变化回调"，用 `setup` 里 spawn 的循环任务轮询：

```rust
fn spawn_clipboard_watcher(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut last = String::new();
        loop {
            tokio::time::sleep(Duration::from_millis(800)).await;
            let Ok(current) = app.clipboard().read_text() else { continue };
            let current = current.trim().to_string();
            if current.is_empty() || current == last {
                continue;              // 空内容 / 没变化 → 跳过
            }
            last = current.clone();    // 记住这次，下次才能判断"变了没"
            push_history(&app, &current);
        }
    });
}
```

- **轮询粒度**：800ms 一次——太快浪费、太慢漏内容，折中即可
- **`last` 是"上次看到的内容"**：只有变了才记，避免同一段文字被重复记几十条
- 去重逻辑在 `push_history`：重复的移除再插到最前（最近使用的排最前）

> **说明：** 轮询是教学级的简化方案。真实剪贴板工具会用系统 API 监听（如 Windows 的剪贴板序列号），但"轮询 + 去重"已经把原理讲透了。

<a id="sec-a08-shortcut"></a>
## 3. 全局快捷键 — 后台也能唤起应用

普通快捷键只有窗口聚焦才有效。**全局快捷键**由操作系统监听，应用在后台也能收到。

### 插件注册 + 全局处理器

```rust
.plugin(
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(|app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                if let Some(win) = app.get_webview_window("main") {
                    if win.is_visible().unwrap_or(false) { let _ = win.hide(); }
                    else { let _ = win.show(); let _ = win.set_focus(); }
                }
            }
        })
        .build(),
)
```

### setup 里注册具体快捷键

```rust
app.global_shortcut().register("ctrl+shift+v")?;
```

- `Builder::with_handler`：**所有已注册快捷键**的公共处理器（这里只注册了一个）
- `event.state() == ShortcutState::Pressed`：只在"按下"时响应（避免抬起也触发一次）
- `register("ctrl+shift+v")`：把快捷键交给系统监听
- Rust 端注册**不需要权限**（`global-shortcut:default` 权限集是空的，但那是给前端 JS 调用的；Rust 端直通）

<a id="sec-a08-menu"></a>
## 4. 应用菜单 — 窗口菜单栏

A03 做过托盘菜单；本课的"应用菜单"是**窗口顶部菜单栏**（Windows 上显示在标题栏下方）：

```rust
use tauri::menu::{Menu, MenuItem};

// setup 里：
let clear_item = MenuItem::with_id(app, "clear", "清空历史", true, None::<&str>)?;
let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
let menu = Menu::with_items(app, &[&clear_item, &quit_item])?;
app.set_menu(menu)?;   // 挂到窗口
app.on_menu_event(|app, event| match event.id.as_ref() {
    "clear" => { /* 清空历史 + emit */ }
    "quit" => app.exit(0),
    _ => {}
});  // 注意：on_menu_event 返回 ()，不加 ?
```

- `MenuItem::with_id(app, id, 文字, 是否可用, 快捷键)`：和 A03 托盘菜单同一个 API
- `app.set_menu(menu)`：设为应用菜单
- `app.on_menu_event(handler)`：按菜单项 `id` 分发点击事件——**"清空历史"既能从菜单触发，也能从前端按钮触发**（两者都调 `save_history` + `emit`）

> **对比 A03 托盘：** 托盘 = 系统托盘图标 + 右键菜单；应用菜单 = 窗口自己的菜单栏。一个是"系统级入口"，一个是"窗口内入口"。API 同源（`Menu` / `MenuItem`），注册方式不同。

---

## 练习指引

**作业范围：** 动 2 个文件，共 9 处 TODO。

| 文件 | 步骤 | 内容 |
|------|------|------|
| `src-tauri/src/lib.rs` | 1 | `push_history` 去重 + 插最前 + 限长 |
| `src-tauri/src/lib.rs` | 2 | `copy_text` 写回剪贴板 |
| `src-tauri/src/lib.rs` | 3 | 监控循环"变了才记" |
| `src-tauri/src/lib.rs` | 4 | 注册全局快捷键 |
| `src-tauri/src/lib.rs` | 5 | 登记三个命令 |
| `src/App.tsx` | 1-4 | 导入 invoke、拉历史、复制、清空 |

**怎么验证：**

```bash
cd 02_mini_apps/a08_clipboard_history
cargo tauri dev
```

随便复制一段文字 → 窗口自动出现这条历史（约 1 秒内）；点它 → 状态行"已复制回剪贴板"；点"清空历史" → 列表清空。按 **Ctrl+Shift+V** → 窗口隐藏/显示（全局快捷键，焦点在别的程序里也有效）。关闭应用再启动 → 历史还在（Store 持久化）。窗口菜单栏"退出" → 真正退出。

**故意踩坑看效果：** 把 `current == last` 判断删掉 → 同一段文字每 800ms 被记一条，列表瞬间爆炸（验证去重）；把 `register` 注释掉 → Ctrl+Shift+V 没反应（验证快捷键必须注册）。

---

## 知识点连起来看

```text
clipboard 插件 read_text/write_text   ← 剪贴板：系统级读写
        │
后台轮询 + last 去重                  ← 监控：变了才记
        │
Store 持久化                          ← 历史：重启还在（复用 A06）
        │
global-shortcut register              ← 快捷键：后台唤起窗口
        │
Menu + on_menu_event                  ← 菜单：窗口内入口
```

| 层 | 解决的问题 | 关键概念 |
|----|-----------|---------|
| 剪贴板 | 读写系统剪贴板 | `ClipboardExt`、`read_text` / `write_text` |
| 监控 | 怎么发现"复制了新东西" | 轮询、`last` 去重 |
| 快捷键 | 后台怎么被唤起 | `global-shortcut`、`register`、`with_handler` |
| 菜单 | 窗口内入口 | `Menu`、`MenuItem`、`on_menu_event` |

**一通百通的核心：** 八道小菜到此收尾——状态、事件、文件、后台、托盘、数据库、网络、剪贴板、快捷键、菜单，桌面应用需要的骨架已经全齐了。**从 A01 到 A08，你做的每个应用都是"界面 + 命令 + 系统能力"的组合。**

**递进关系：** 下一站是**超级项目**——Markdown 编辑器。26 步会把八道小菜的所有能力组合成一个可安装分发的完整产品。
