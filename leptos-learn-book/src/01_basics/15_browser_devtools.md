# 练习 15: 浏览器开发者工具

## 为什么要学这个

到目前为止，你的 Rust 代码在编辑器中加 `println!` 打印日志，在命令行中看输出。一切正常——直到你把代码编译成 WASM 扔进浏览器。

**WASM 没有 stdout。** `println!` 在 WASM 环境中不会在终端打印，它要么什么都不做，要么直接 panic——取决于目标平台和绑定情况。

写 Web 应用时，你的代码跑在浏览器里，唯一的控制台就是**浏览器的开发者工具**（DevTools）。你需要一种方式把调试信息输出到浏览器的 Console 面板。

这个问题引出更深一层的问题：

1. **Rust 程序跑在浏览器里，它的"控制台"在哪里？** — WASM 运行时和浏览器环境的接口是什么？
2. **`tracing` 是什么？为什么比直接用 `console.log` 好？** — 日志框架之于应用程序，就像仪表盘之于汽车
3. **Panic 了怎么办？** — Rust 的 unwrap 在 WASM 中是什么表现？

---

## 从问题出发

### 浏览器控制台：WASM 的 stdout

浏览器提供了 `console.log`，但那是 JavaScript 的 API。Rust 代码不能直接调用 JavaScript 函数——需要通过 **FFI（外部函数接口）** 或者 **WASM 绑定**。

```
Rust 代码                    JS 桥接                     浏览器
┌───────────────┐    ┌─────────────────┐    ┌──────────────────────┐
│ println!()     │───►│ 无绑定 → 无效   │───►│ （无声无息）         │
│                │    │                 │    │                      │
│ web_sys::console│──►│ JS console.log  │───►│ Console 面板可见     │
│   .log_1(&s)   │    │                 │    │                      │
│                │    │                 │    │                      │
│ tracing::info! │───►│ tracing-wasm    │───►│ Console 面板 + 级别  │
│                │    │ 自动转发         │    │ （info/warn/error）  │
└───────────────┘    └─────────────────┘    └──────────────────────┘
```

这个练习引入了三个工具来打通这条路：

| 工具 | 作用 |
|------|------|
| `console_error_panic_hook` | 把 Rust 的 panic 信息输出到浏览器控制台 |
| `tracing_wasm` | 把 `tracing` 日志框架的输出重定向到 `console.log` |
| `tracing::info!` | Rust 端的日志宏，最终出现在浏览器的 Console 面板 |

---

## 1. `console_error_panic_hook` — 让 panic 可见

### 没有它的时候

```rust
let items = vec![1, 2, 3];
let _ = items[5]; // 越界，panic!
```

在原生 Rust 中，这会打印：

```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 5'
```

但在 WASM 中，没有这个"thread main"的概念。没有 `console_error_panic_hook` 的情况下，panic 会**静默失败**——你的应用突然"卡住了"，但控制台没有任何错误信息。这是 Web 开发中最令人沮丧的体验之一：**应用死了，但没人告诉你为什么。**

### `console_error_panic_hook` 做了什么

```rust
fn main() {
    console_error_panic_hook::set_once();
    // 之后任何 panic 都会被捕获并打印到浏览器控制台
}
```

它用 `std::panic::set_hook` 注册了一个自定义 panic 处理器。当 panic 发生时，这个处理器把 panic 消息和堆栈信息通过 `console.error` 输出到 DevTools。

```
console_error_panic_hook 的工作流：

1. 你的代码 panic（例如 unwrap() 一个 None）
        ↓
2. set_hook 的自定义处理器被调用
        ↓
3. 处理器格式化 panic 信息（文件、行号、panic 消息）
        ↓
4. 调用 web_sys::console::error_1()
        ↓
5. 浏览器 Console 面板显示红色错误消息
```

> **`set_once()` 的命名含义：** 你只能设置一次 panic hook。多次调用会 panic。所以命名 `set_once` 提醒你："如果你可能多次调用它，先检查是否已经设置过。"在单入口的 WASM 应用中，`fn main` 中调用一次就够了。

---

## 2. `tracing` — 结构化的日志框架

### 为什么不用直接的 `console.log`？

你可以直接用 `web_sys::console::log_1(&"hello".into())` 在浏览器中打印日志。但这有几个问题：

1. **调用繁琐** — 需要引入 `wasm_bindgen`、`web_sys`，手动转换类型
2. **无级别区分** — 全是 `log`，你分不清 info/warn/error
3. **无法过滤** — 生产环境你不能关掉调试日志
4. **无结构化数据** — 只能打印字符串，无法方便地记录结构化上下文

`tracing` 框架解决了所有这些问题。它是一个**结构化的、带有级别和跨度（span）的日志框架**，在 Rust 生态中广泛使用。

### 基本用法

```rust
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();  // ← 关键：把 tracing 连接到 browser console
    tracing::info!("应用已启动");            // ← 这会出现在浏览器控制台

    mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    tracing::info!("初始计数: {}", count.get());  // 带格式化的日志

    view! {
        <button on:click=move |_| {
            set_count.set(count.get() + 1);
            tracing::info!("计数增加至: {}", count.get());  // 事件日志
        }>
            "增加"
        </button>
    }
}
```

### 日志级别

`tracing` 提供五个级别，对应浏览器的不同 console 方法：

| tracing 级别 | 浏览器 console 方法 | Console 显示样式 |
|-------------|-------------------|-----------------|
| `trace!` | `console.debug` | 灰色，常被 DevTools 默认隐藏 |
| `debug!` | `console.debug` | 灰色 |
| `info!` | `console.info` | 默认可见，带 ℹ️ 图标 |
| `warn!` | `console.warn` | 黄色背景，带 ⚠️ 图标 |
| `error!` | `console.error` | 红色背景，带 ❌ 图标 |

在开发中，你应该：用 `info!` 记录常规流程（组件初始化、用户操作），用 `warn!` 记录预期外但非致命的情况（API 返回空数据），用 `error!` 记录异常（网络请求失败）。

### `tracing_wasm` 的桥梁作用

```
tracing::info!("消息")            ← 你的代码
        ↓
tracing 库的 subscriber 机制       ← 日志框架内部
        ↓
tracing_wasm::WasmLayer           ← wasm 适配层
        ↓
web_sys::console::info_1(...)     ← JS API 调用
        ↓
浏览器 Console 面板显示日志        ← 开发者看到的结果
```

`tracing_wasm::set_as_global_default()` 把 tracing 框架的输出目标设置为"WASM 控制台"。从此所有 `tracing::info!` 调用都会自动流向浏览器的 Console 面板。

---

## 3. 调试工作流

### 如何在浏览器中调试 Leptos 应用

```
步骤 1：在代码中插入 tracing::info!
        追踪关键路径（初始化、事件处理、状态变化）

步骤 2：编译并运行 (trunk serve)
        浏览器打开 http://localhost:8080

步骤 3：打开 DevTools (F12) → Console 面板
        查看 tracing 输出的日志

步骤 4：在 Console 中过滤
        按级别过滤（Info / Warn / Error）
        按关键字搜索（输入 "计数" 筛选相关日志）
```

### 实用的调试模式

```rust
// 1. 追踪组件生命周期
#[component]
fn MyComponent() -> impl IntoView {
    tracing::info!("MyComponent 已挂载");
    // ...
}

// 2. 追踪事件
<button on:click=move |_| {
    tracing::info!("按钮被点击");
    // ...
}>"点击"</button>

// 3. 追踪状态变化
let (count, set_count) = signal(0);
// 在影响 UI 的地方
tracing::info!("count: {}", count.get());

// 4. 追踪错误路径
let result = fallible_operation();
match result {
    Ok(data) => { /* ... */ },
    Err(e) => tracing::error!("操作失败: {:?}", e),
}
```

> **关键原则：** 日志是**审计痕迹**——你应该能在不打断点的情况下，通过日志理解用户操作流的全过程。好的日志是"程序的故事"，而不是"程序的颅内独白"。

---

## 实战贴士

### 如何避免日志被过滤掉？

某些浏览器默认会隐藏 `trace` 和 `debug` 级别的日志。在 Chrome DevTools 的 Console 面板中，找到日志级别过滤下拉菜单（通常在 Console 标签页的顶部），确保勾选了 "Verbose" 或你需要的级别。

### 不要把敏感信息写入日志

`tracing::info!` 的日志在浏览器 Console 中任何人都能看到（只需 F12）。不要把密码、Token、个人身份信息等写入日志。

### `console_error_panic_hook` 只在 debug 模式生效？

不是。它无论 debug 还是 release 都会生效。实际上在生产环境中它更有用——因为 release 模式下的 panic 信息更难捕获。不过如果你担心日志泄露，可以在条件编译中包裹：

```rust
#[cfg(debug_assertions)]
console_error_panic_hook::set_once();
```

但大多数 Leptos 应用不会在 release 时 panic，所以这个配置通常不是必需的。

---

## 一通百通

```
调试 WASM 应用 = logging 框架 + WASM 桥接 + 浏览器 Console

你学会的其实是：
  你的代码跑的"环境"不是终端，而是浏览器
  ↓
  调试工具链要从"终端打印"切换到"浏览器 Console"
  ↓
  tracing + tracing_wasm 是这种切换的标准方案
```

| 知识点 | 核心理解 |
|--------|---------|
| `console_error_panic_hook` | 把 Rust panic 映射到浏览器 console.error |
| `tracing_wasm::set_as_global_default()` | 将 tracing 输出目标设为浏览器 Console |
| `tracing::info!(...)` | 在浏览器中输出 info 级别日志 |
| 日志级别 | trace/debug/info/warn/error 对应不同 console 方法 |
| WASM 没有 stdout | 所有调试输出必须通过 JS 桥接到达浏览器 |

这个练习是一个"方法论"章节——它不讲新的 UI 技术，而是教你在新的环境中生存下去。你会发现在后面的练习中，`tracing::info!` 是你最频繁使用的调试工具——比断点更轻量，比 `println!` 更符合 WASM 的范式。
