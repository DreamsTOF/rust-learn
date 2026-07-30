# Leptos 踩坑记录

> 所有 agent 在编写练习前务必阅读此文件！避免重复踩坑。

## Leptos 版本

当前工作区使用 **Leptos ~0.8 nightly**（非 0.9 stable）。API 与旧版 leptos 有差异。

---

## 通用 API 变更

| 旧 API | 新 API | 说明 |
|--------|--------|------|
| `create_signal` | `signal()` | 返回 `(ReadSignal, WriteSignal)` |
| `create_rw_signal` | `RwSignal::new()` | 返回 `RwSignal<T>` |
| `create_effect` | `Effect::new()` | |
| `create_memo` | `Memo::new()` | |
| `For` children 闭包 | `let:item` 语法 | `<For each=move\|\| items.get() key=\|item\| *item let:item>` |
| `ref=` | `node_ref=` | `ref` 是 Rust 2024 edition 关键字 |
| `Callback<T>.call()` | `.run()` 方法 | 用 `callback.run(input)` 调用；`Callback::new()` 可用 |
| `Callback<T>` (0.8 nightly) | 支持 `.run()` | `Callback<T>` 可直接作为组件 prop 类型，无需泛型 |

## Resource API (0.8)

- `Resource::new(|| source, \|_\| async { ... })` — 最新构造函数
- **没有 `.loading()` 方法** → 用 `.map()` 返回 `Option<T>`，`None`=加载中
- **没有 `.error()` 方法** → 用 `Resource<Result<T, E>>` + `.map()` 匹配 Ok/Err
- `.map(|v| v.clone())` 是标准读取方式，返回 `Option<T>`（同步、响应式）
- `.refetch()` 手动触发重新加载
- `LocalResource::new()` 用于纯客户端资源（无需 serde）
- `Suspend::new(async { resource.await })` 展平 Resource 到视图层
- `FromStream::from_stream()` 将 Stream 转为 Signal

## Props

- `#[prop(optional)]` 会剥离 `Option` 包装 → 调用方传 `T` 而非 `Some(T)`
- `#[prop(into)]` 自动调用 `.into()`
- `#[prop(default = expr)]` 设置默认值
- Props 结构体需要 `#[derive(Clone)]`
- 泛型组件需 `'static` 约束：`T: 'static`

## DOM 操作

- `NodeRef<T>` 不能直接接受闭包回调 → 用 `NodeRef::get()` 在 `Effect::new()` 中检测挂载
- `html` 模块需显式导入 `use leptos::html;`（不在 prelude 中）
- `get_bounding_client_rect()` 需要额外 `web-sys` feature → 改用 `offset_width()/offset_height()`
- `set_scroll_top()` 接受 `i32`
- 使用 `html::Div`、`html::Input` 等类型时需 `use leptos::html::*;`
- `window_event_listener(ev::resize, ...)` 从 leptos 直接使用

## 浏览器 API

- `document()`、`window()`、`set_interval()`、`request_animation_frame()` 通过 `leptos::prelude` 导出
- `spawn_local` 来自 `leptos::task::spawn_local`
- `wasm-bindgen` 必须作为**直接依赖**（`wasm-bindgen.workspace = true`），因为 `#[wasm_bindgen]` 宏生成代码引用 `::wasm_bindgen`
- `web-sys` features 不会自动由 leptos 启用 → 要么在 Cargo.toml 添加 feature flag，要么用 `js_sys::Reflect` + 内联 JS
- Observer 模式需要 `wasm_bindgen::closure::Closure::wrap` (或 `Closure::new`) + `cb.forget()`
- `Effect::new` 回调创建时立即执行一次（非延迟），之后在响应式依赖变化时重新执行
- `Cell::new(false)` + `Effect::new` 是实现一次性初始化（如 DOM 挂载后的 observer 设置）的常用模式
- `#[wasm_bindgen(inline_js = r#"... "#)]` 使用 raw string literal 避免 JS 代码中的转义问题

## 错误处理

- `<ErrorBoundary>` 捕获子组件中的 `Result::Err` 类型错误
- 错误冒泡：子组件 `throw()` → 最近的 `<ErrorBoundary>` 捕获
- `Result<T, E>` 在 ErrorBoundary 中，`T` 需实现 `IntoView`

## 其他

- `view!` 宏中不能直接渲染 `&String` → 使用 owned `String` 或 `.clone()`
- `match` 分支返回不同类型需要用 `.into_any()` 统一
- `#[prop(optional)]` + `Option<T>`：调用方直接传 `T`，不传时 prop 为 `None`
- `console_error_panic_hook::set_once()` 可选，需添加依赖

## 答案目录 (answer)

- 答案目录的 `Cargo.toml` 必须使用与练习相同的 `edition`（当前为 `"2024"`），否则编译可能失败
- 答案目录的 `index.html` 必须使用 UTF-8 编码保存；非 UTF-8 编码会导致中文标题乱码
- 答案目录的 `Cargo.toml` **不会自动继承练习的额外依赖**。如果练习代码使用了 `wasm-bindgen`、`js-sys` 等依赖，必须在答案的 `Cargo.toml` 中手动添加：
  - `wasm-bindgen.workspace = true`（当代码包含 `#[wasm_bindgen]` 或 `use wasm_bindgen::` 时必需）
  - `js-sys = "0.3"`（当代码使用 `js_sys::Reflect`、`js_sys::global()` 时必需）
- **组合多个练习到 `_answer/src/main.rs`** 时需注意：
  - 每个 `#[wasm_bindgen] extern "C"` 块必须自包含；组件名必须唯一（不能有多个 `Exercise` 组件）
  - 需要在 crate root 添加 `#![feature(extern_types)]`，因为多个 `extern "C"` 块中的类型定义需要该 feature gate
  - `use wasm_bindgen::closure::Closure;` 需要显式导入（`prelude::*` 不包含 `Closure`）
  - `edition = "2021"` 配合 `#![feature(extern_types)]` 比 `edition = "2024"` 更稳定——后者会导致 `#[wasm_bindgen]` 函数被标记为 `unsafe` 需要 `unsafe {}` 块

## Scroll API

- `scroll_to_with_x_and_y(x, y)` 参数类型为 `f64`
- `set_scroll_top(y)` 参数类型为 `i32`

## web-sys

- `leptos` 重导出 `web_sys` → 使用 `use leptos::web_sys;` 无需添加 `web-sys` 为直接依赖
- `web_sys::window()` 返回 `Option<Window>`，需要 `if let Some(win) = web_sys::window()` 解包
- `inner_width()` / `inner_height()` 返回 `Result<f64>`，需 `.unwrap().as_f64()` 获取 f64

## DOM 属性绑定

- `prop:value={value}` 设置 DOM 属性（而非 HTML 属性）用于受控输入组件 — `prop:` 前缀直接操作 DOM property
- `event_target_value(&ev)` 是 leptos prelude 导出的函数，用于从 `on:input` 事件中提取 `<input>` 的当前值
- 泛型回调 prop 模式：`fn BindedInput<F>(value: String, on_change: F) where F: Fn(String) + 'static` — 替代已废弃的 `Callback<T>`
