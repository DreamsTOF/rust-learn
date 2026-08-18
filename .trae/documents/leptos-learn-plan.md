# Leptos 练习项目 — 实现计划

## 概述

在 `c:\code\testruetlearn\` 下创建 `leptos-learn/` 目录，Cargo workspace 内含约 **400+ 道练习题**，覆盖从 view! 宏到全栈实战的 8 章内容。每道题都是独立可运行的 Leptos CSR 应用（`trunk serve` 一键预览）。

参照 **100 Exercises to Learn Rust** 风格：每个练习是一个独立的 Cargo crate，可编译可运行，所见即所得。同一个知识点会用多道题展示不同解法（比如 `set()` vs `update()` vs `write()` 三种改值方式各一题）。

---

## 项目结构

```
c:\code\testruetlearn\leptos-learn\
├── Cargo.toml                   # workspace（管理所有练习 crate）
├── rust-toolchain.toml          # nightly 工具链锁定
│
├── 00_preface/                  # 练习导航首页 + 使用说明
│   ├── Cargo.toml
│   ├── index.html
│   └── src/main.rs
│
├── 01_basics/                   # 第 1 章目录（每组练习一个目录）
│   ├── Cargo.toml               # workspace member 声明（在 workspace 统一管理）
│   ├── e01_hello_world/
│   │   ├── Cargo.toml
│   │   ├── index.html
│   │   └── src/main.rs          # 练习代码（含 TODO 引导 + 参考答案注释）
│   ├── e02_html_elements/
│   └── ...
│
├── 02_signals/
├── 03_components/
├── 04_async/
├── 05_router/
├── 06_advanced/
│
├── 07_ssr/                      # SSR 章节（独立 cargo-leptos 项目）
│   ├── e57_ssr_setup/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs          # server entry
│   │   │   ├── lib.rs           # app 定义
│   │   │   └── app.rs           # 组件
│   │   └── index.html
│   └── ...
│
└── projects/                    # 综合实战（完整应用级练习）
    ├── todo_app/
    ├── weather_dashboard/
    └── blog_system/
```

### 练习调用方式

```bash
# CSR 练习（1-6 章 + projects）
cd 01_basics/e01_hello_world
trunk serve --port 3000 --open

# SSR 练习（第 7 章）
cd 07_ssr/e57_ssr_setup
cargo leptos serve
```

---

## 技术选型

| 项目        | 选型                                                 | 原因                                                                            |
| ----------- | ---------------------------------------------------- | ------------------------------------------------------------------------------- |
| Leptos 版本 | **0.9.x**                                      | 2026 年最新，支持稳定和 nightly 双轨                                            |
| Rust 通道   | **nightly**                                    | 函数调用语法`foo()` / `set_foo()` 更简洁                                    |
| 构建工具    | **Trunk** (CSR) / **cargo-leptos** (SSR) | 官方推荐                                                                        |
| 工具链锁定  | `rust-toolchain.toml`                              | 团队一致性                                                                      |
| UI 组件库   | **Thaw UI**                                    | 开箱即用的 Leptos 组件库（Table/Form/Button/Toast 等）                          |
| 工具库      | **leptos-use**                                 | 响应式工具集（`use_media_query`、`use_debounce`、`use_local_storage` 等） |
| 路由        | **leptos_router**                              | 官方路由                                                                        |

### 依赖管理

Workspace `Cargo.toml` 统一声明版本号，各 crate 的 `Cargo.toml` 只写 `workspace = true`：

```toml
# workspace/Cargo.toml
[workspace]
members = [
    "00_preface",
    "01_basics/e01_hello_world",
    "01_basics/e02_html_elements",
    # ... 所有 crate
]

[workspace.dependencies]
leptos = { version = "0.9", features = ["nightly"] }
leptos_router = "0.9"
leptos-use = "0.9"
thaw = "0.9"
serde = { version = "1", features = ["derive"] }
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.2"
tracing-wasm = "0.2"
console_error_panic_hook = "0.1"
gloo-net = "0.6"
```

```toml
# 01_basics/e01_hello_world/Cargo.toml
[package]
name = "e01_hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos.workspace = true
```

### Leptos 0.9 API 风格

```rust
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    // 函数调用语法（nightly）：
    // count() 等价于 count.get()
    // set_count(42) 等价于 set_count.set(42)

    view! {
        <button on:click=move |_| set_count(count() + 1)>
            "Click me: " {count}
        </button>
        // 函数调用 set_count 不需要 .set() / .update()
        // 但 .set() / .update() / .write() 仍然可用
    }
}

fn main() {
    mount_to_body(App);
}
```

---

## 难度分布规则

每 5 题一组，难度递增：

| 偏移 | 难度   | 引导程度               | 代码完成度     |
| ---- | ------ | ---------------------- | -------------- |
| +0   | ⭐     | 每行都有详细 TODO 注释 | 只需填空       |
| +1   | ⭐     | 每行都有详细 TODO 注释 | 只需填空       |
| +2   | ⭐⭐   | 关键位置有 TODO        | 补全 50%       |
| +3   | ⭐⭐   | 少量提示               | 补全 50%       |
| +4   | ⭐⭐⭐ | 仅描述目标             | 几乎全部自己写 |

---

## 内容大纲（8 章，约 385 题）

### 第 1 章：基础与环境（20 题）

**目标：** 搭建开发环境，理解 `view!` 宏和组件基本概念。

| #  | 题目                     | 难度   | 核心知识点                                                       |
| -- | ------------------------ | ------ | ---------------------------------------------------------------- |
| 01 | Hello World              | ⭐     | `mount_to_body`, `view!` 宏, `#[component]`                |
| 02 | HTML 文本节点            | ⭐     | 字符串文本`"..."`、多行文本                                    |
| 03 | HTML 元素与属性          | ⭐     | `class`, `id`, `style`, `<a>`, `<img>`                 |
| 04 | 元素嵌套                 | ⭐     | `<div>`, `<section>`, 层级结构                               |
| 05 | 组件定义与调用           | ⭐⭐   | 函数组件、`impl IntoView`                                      |
| 06 | 组件嵌套                 | ⭐⭐   | `<App/>` 嵌套 `<Header/>` `<Main/>` `<Footer/>`          |
| 07 | Fragment 语法            | ⭐⭐   | `<></>`, `view! { ... }` 多根节点                            |
| 08 | 注释写法                 | ⭐     | `view!` 内的 `/* comment */`                                 |
| 09 | Rust 表达式嵌入          | ⭐⭐   | `{ }` 块、变量插值                                             |
| 10 | 块级表达式               | ⭐⭐   | `{ let x = 1; x + 2 }`                                         |
| 11 | 条件`if` 在 view 中    | ⭐⭐⭐ | `{ if cond { "A" } else { "B" } }`                             |
| 12 | 匹配`match` 在 view 中 | ⭐⭐⭐ | `{ match x { 1 => "一", _ => "其他" } }`                       |
| 13 | 索引/方法调用            | ⭐⭐   | `{ items.len() }`, `{ items[0] }`                            |
| 14 | 无宏构建器模式           | ⭐⭐⭐ | `div().child("text").build()`                                  |
| 15 | 调试：浏览器开发者工具   | ⭐⭐   | WASM 调试、console.log                                           |
| 16 | SVG 元素                 | ⭐⭐   | view! 中创建`<svg>`、`<circle>`、`<rect>`、`<text path>` |
| 17 | 原始 HTML 渲染           | ⭐⭐   | `dangerous_inner_html`、XSS 防范、`evil:` 转义               |
| 18 | view! 多根节点 Fragment  | ⭐     | `<></>` 的多种使用场景、嵌套 Fragment                          |
| 19 | 动态标签名               | ⭐⭐   | 根据变量值动态渲染不同 HTML 标签（h1/h2/h3）                     |
| 20 | 构建器模式高级           | ⭐⭐⭐ | 纯构建器 API 构建含事件监听器和样式的完整组件                    |

---

### 第 2 章：响应式系统（75 题）

**目标：** 深入理解 Leptos 的细粒度响应式模型，掌握所有信号 API。

#### 2.1 信号创建与读取（17 题）

| #  | 题目                                   | 难度   | 核心知识点                                                   |
| -- | -------------------------------------- | ------ | ------------------------------------------------------------ |
| 21 | `signal()` 创建                      | ⭐     | `let (read, write) = signal(val)`                          |
| 22 | `.get()` 克隆读取                    | ⭐     | `count.get()` 返回 `T`                                   |
| 23 | 函数调用语法读取                       | ⭐     | `count()` 等价于 `.get()`                                |
| 24 | `.with()` 引用读取                   | ⭐⭐   | `count.with(\|n\| *n)` 避免克隆                              |
| 25 | `.read()` guard 读取                 | ⭐⭐   | `let guard = count.read(); guard.len()`                    |
| 26 | 三种读取方式对比                       | ⭐⭐   | `.get()` vs `.with()` vs `.read()` 性能差异            |
| 27 | `.set()` 设置                        | ⭐     | `count.set(42)`                                            |
| 28 | 函数调用语法设置                       | ⭐     | `set_count(42)`                                            |
| 29 | `.update()` 原地更新                 | ⭐⭐   | `count.update(\|n\| *n += 1)`                                |
| 30 | `.write()` guard 更新                | ⭐⭐   | `*count.write() = 42`                                      |
| 31 | `.try_update()` 带返回值             | ⭐⭐⭐ | `let old = count.try_update(\|n\| std::mem::replace(n, 42))` |
| 32 | 四种写入方式对比                       | ⭐⭐   | `set` vs 函数调用 vs `update` vs `write`               |
| 33 | 多信号创建                             | ⭐     | `let (a, b) = (signal(0), signal(""))`                     |
| 34 | 信号类型推断                           | ⭐⭐   | 泛型参数、类型推导                                           |
| 35 | `ReadSignal` vs `WriteSignal` 分离 | ⭐⭐   | 只读/只写权限分离                                            |
| 36 | 信号默认值懒初始化                     | ⭐⭐   | `signal(                                                     |
| 37 | 信号 Drop 语义                         | ⭐⭐   | 超出作用域自动清理、Scope 生命周期                           |

#### 2.2 派生信号与 Memo（16 题）

| #  | 题目                                 | 难度   | 核心知识点                                        |
| -- | ------------------------------------ | ------ | ------------------------------------------------- |
| 38 | 简单派生`move \|\|`                  | ⭐     | `let double = move \|\| count() * 2`              |
| 39 | 多信号派生                           | ⭐     | `let sum = move \|\| a() + b()`                   |
| 40 | `Memo` 基础                        | ⭐⭐   | `Memo::new()`, 缓存派生                         |
| 41 | `Memo` vs 原始闭包                 | ⭐⭐   | Memo 只在其依赖变化时重算                         |
| 42 | `Memo` 的 `.get()`               | ⭐     | `double.get()`                                  |
| 43 | `Memo` 链式                        | ⭐⭐   | `a -> memo1 -> memo2 -> ...`                    |
| 44 | `.with()` 在 Memo 上               | ⭐⭐   | 避免克隆 memo 值                                  |
| 45 | 条件派生（非均匀更新）               | ⭐⭐⭐ | 只在一定条件下触发重算                            |
| 46 | 惰性派生                             | ⭐⭐   | 闭包捕获信号但不立即计算                          |
| 47 | 派生中调用函数                       | ⭐⭐   | `move \|\| format!("{}", count())`                |
| 48 | 闭包中使用`.with()`                | ⭐⭐   | `move \|\| count.with(\|n\| n.to_string())`         |
| 49 | 信号数组派生                         | ⭐⭐⭐ | `(0..10).map(\|i\| move \|\| base() + i).collect()` |
| 50 | 响应式 Eq 判断                       | ⭐⭐⭐ | `move \|\| a() == b()` 的响应式行为               |
| 51 | 派生信号作 prop                      | ⭐⭐   | `value=move \|\| count() * 2` 传子组件            |
| 52 | 何时用 Signal 派生 vs`create_memo` | ⭐⭐⭐ | 性能权衡                                          |
| 53 | 派生信号的条件传播                   | ⭐⭐⭐ | 只有依赖变化时才通知下游消费者                    |

#### 2.3 Effect 与生命周期（21 题）

| #  | 题目                      | 难度   | 核心知识点                            |
| -- | ------------------------- | ------ | ------------------------------------- |
| 54 | `Effect::new()`         | ⭐     | `Effect::new(move \|\| { ... })`      |
| 55 | Effect 响应信号变化       | ⭐     | 信号改变 → Effect 重新执行           |
| 56 | Effect 依赖追踪           | ⭐⭐   | Effect 只追踪内部读取的信号           |
| 57 | Effect 不追踪外部变更     | ⭐⭐   | 信号在 Effect 外改变不会触发          |
| 58 | 多个 Effect               | ⭐⭐   | 多个生命期互不影响                    |
| 59 | `watch()` 函数          | ⭐⭐   | `watch(                               |
| 60 | `watch()` vs `Effect` | ⭐⭐   | watch 显式指定依赖                    |
| 61 | Effect 中的条件分支       | ⭐⭐   | `if count() > 0 { ... }`            |
| 62 | Effect 清理               | ⭐⭐⭐ | `on_cleanup()` 释放资源             |
| 63 | Effect 中调用异步         | ⭐⭐⭐ | `spawn_local(async { ... })`        |
| 64 | 避免 Effect 死循环        | ⭐⭐   | 不在 Effect 中写被自己读取的信号      |
| 65 | Batched 更新              | ⭐⭐⭐ | `batch(\|\| { set_a(1); set_b(2); })` |
| 66 | `untrack()` 取消追踪    | ⭐⭐⭐ | `untrack(move \|\| count())`          |
| 67 | `with_untrack()`        | ⭐⭐⭐ | 读值但不建立依赖                      |
| 68 | Effect 调试               | ⭐⭐   | `tracing::info!` 观察重新执行       |
| 69 | `StoredValue` 基础      | ⭐⭐⭐ | 不触发更新的存储                      |
| 70 | `StoredValue` vs Signal | ⭐⭐⭐ | 何时用 StoredValue                    |
| 71 | `RwSignal`              | ⭐⭐⭐ | 同时读写同一个 handle                 |
| 72 | `ArcSignal` 线程安全    | ⭐⭐⭐ | 多线程/Web Worker 共享                |
| 73 | 响应式图析构              | ⭐⭐⭐ | Scope 生命周期、节点销毁              |
| 74 | Effect 嵌套与作用域       | ⭐⭐⭐ | 子 Scope 中的 Effect 独立追踪和清理   |

#### 2.4 条件与列表渲染（21 题）

| #  | 题目                            | 难度   | 核心知识点                            |
| -- | ------------------------------- | ------ | ------------------------------------- |
| 75 | `Show` 基础                   | ⭐     | `<Show when=cond>`                  |
| 76 | Show + fallback                 | ⭐     | `<Show when fallback=\|\| "loading">` |
| 77 | Show 嵌套条件                   | ⭐⭐   | `a && b` 组合条件                   |
| 78 | Show 替换组件                   | ⭐⭐   | `when=true` 显示 A，否则 B          |
| 79 | 三元表达式                      | ⭐⭐   | `{if cond { "A" } else { "B" }}`    |
| 80 | 多级`Show`                    | ⭐⭐   | 多个 Show 组件级联                    |
| 81 | `For` 列表渲染                | ⭐     | `<For each=items>`                  |
| 82 | For + key                       | ⭐⭐   | keyed 更新提高性能                    |
| 83 | For with index                  | ⭐⭐   | `.enumerate()` 获取索引             |
| 84 | 空列表处理                      | ⭐     | `fallback=\|\| "No items"`            |
| 85 | For 嵌套列表                    | ⭐⭐   | 二维列表渲染                          |
| 86 | For 的`.keys()` 闭包          | ⭐⭐   | `key=\|item\| item.id`                |
| 87 | 列表动态增删                    | ⭐⭐   | `push` / `remove` 自动更新 DOM    |
| 88 | 列表重排序                      | ⭐⭐⭐ | `sort_by` / `reverse` 触发重排    |
| 89 | `FilterMap` 模式              | ⭐⭐⭐ | 列表过滤派生                          |
| 90 | 列表分页                        | ⭐⭐⭐ | `.skip().take()` 子集渲染           |
| 91 | `<Portal/>`                   | ⭐⭐⭐ | Portal 渲染到 body                    |
| 92 | 动态标签名                      | ⭐⭐⭐ | 根据变量渲染不同 HTML 标签            |
| 93 | `<ErrorBoundary/>` 基础       | ⭐⭐   | 捕获组件渲染错误                      |
| 94 | ErrorBoundary fallback          | ⭐⭐   | `fallback=\|\| "出错了"`              |
| 95 | `<TransitionGroup/>` 列表动画 | ⭐⭐⭐ | 列表增删移时的过渡动画                |

---

### 第 3 章：组件进阶（60 题）

#### 3.1 Props 与通信（22 题）

| #   | 题目                                | 难度   | 核心知识点                                     |
| --- | ----------------------------------- | ------ | ---------------------------------------------- |
| 96  | 必需 Props                          | ⭐     | `#[component] fn Greet(name: String)`        |
| 97  | 可选 Props`Option`                | ⭐⭐   | `name: Option<String>`                       |
| 98  | 默认值 Props                        | ⭐⭐   | `#[prop(default = 0)] count: i32`            |
| 99  | `#[prop(into)]`                   | ⭐⭐⭐ | 自动转换`Into<T>`                            |
| 100 | 闭包 Props                          | ⭐⭐   | `on_click: impl Fn() + 'static`              |
| 101 | `Children` 插槽                   | ⭐⭐   | `children: Children`                         |
| 102 | 具名插槽                            | ⭐⭐   | 多个插槽，`children: ChildrenFn`             |
| 103 | Children 作为闭包                   | ⭐⭐⭐ | `children: ChildrenFn` 动态渲染子节点        |
| 104 | 回调 prop 模式                      | ⭐⭐   | 父子通信                                       |
| 105 | `Callback` 类型                   | ⭐⭐   | `Callback::new()`                            |
| 106 | 回调参数                            | ⭐⭐   | `on_input: Callback<String>`                 |
| 107 | 父子组件双向绑定                    | ⭐⭐⭐ | `value + on_change` 模式                     |
| 108 | Props 解构                          | ⭐⭐   | `fn App(SomeProps { name, age }: SomeProps)` |
| 109 | 透传 Props                          | ⭐⭐⭐ | 将 props 传给子元素                            |
| 110 | 泛型组件                            | ⭐⭐   | `fn List<T: 'static>(items: Vec<T>)`         |
| 111 | 泛型约束 +`'static`               | ⭐⭐   | 泛型 + trait bound                             |
| 112 | Props Struct 独立定义               | ⭐⭐   | 分离 prop 结构体                               |
| 113 | 组件文档注释                        | ⭐     | `/// 组件说明` 文档                          |
| 114 | 条件 props                          | ⭐⭐⭐ | `#[prop(default)]` 的骚操作                  |
| 115 | `attrs` 透传                      | ⭐⭐⭐ | 任意属性透传                                   |
| 116 | `#[prop(optional)]` vs `Option` | ⭐⭐   | 二者在默认值和类型签名上的区别                 |
| 117 | Props 编译期验证                    | ⭐⭐   | 缺失必填 prop 的编译报错、类型安全检查         |

#### 3.2 Context 与依赖注入（12 题）

| #   | 题目                       | 难度   | 核心知识点                                            |
| --- | -------------------------- | ------ | ----------------------------------------------------- |
| 118 | `provide_context`        | ⭐⭐   | 父组件提供值                                          |
| 119 | `use_context`            | ⭐⭐   | 子组件消费值                                          |
| 120 | Context 类型安全           | ⭐     | 泛型参数严格匹配                                      |
| 121 | Context 覆盖               | ⭐⭐   | 嵌套 provide 覆盖                                     |
| 122 | 多层 Context               | ⭐⭐   | 同时提供不同类型                                      |
| 123 | Context + Signal           | ⭐⭐   | 提供可变的信号                                        |
| 124 | Context 在路由中           | ⭐⭐   | layout 提供，page 消费                                |
| 125 | Context 替代 prop drilling | ⭐⭐⭐ | 跨多层传递                                            |
| 126 | 全局状态模式               | ⭐⭐⭐ | `AppState` 全局单例                                 |
| 127 | Context + Memo             | ⭐⭐⭐ | 提供派生状态                                          |
| 128 | Context 默认值兜底         | ⭐⭐   | `use_context::<T>()` 返回 `Option` 时的默认值处理 |
| 129 | Context 类型擦除           | ⭐⭐⭐ | `AnyMap` 模式、不同类型 Context 共存原理            |

#### 3.3 DOM 操作与 NodeRef（13 题）

| #   | 题目                              | 难度   | 核心知识点                                 |
| --- | --------------------------------- | ------ | ------------------------------------------ |
| 130 | `NodeRef` 基础                  | ⭐⭐   | `let el: NodeRef<html::Div>`             |
| 131 | NodeRef 回调                      | ⭐⭐   | `ref=move \|el\| ...`                      |
| 132 | 输入框`.focus()`                | ⭐⭐   | 组件挂载后聚焦                             |
| 133 | 测量元素尺寸                      | ⭐⭐   | `.get_bounding_client_rect()`            |
| 134 | 滚动控制                          | ⭐⭐   | `.scroll_to()`                           |
| 135 | 事件监听`window_event_listener` | ⭐⭐⭐ | `window_event_listener(ev::resize, ...)` |
| 136 | `document` / `window` 访问    | ⭐⭐   | `document()` / `window()`              |
| 137 | 定时器`set_interval`            | ⭐⭐   | 轮询 / 动画                                |
| 138 | 动画帧`request_animation_frame` | ⭐⭐⭐ | 动画循环                                   |
| 139 | 第三方 JS 库集成                  | ⭐⭐⭐ | `wasm_bindgen` 调用 JS                   |
| 140 | IntersectionObserver 懒加载       | ⭐⭐⭐ | 元素进入视口检测、图片懒加载               |
| 141 | ResizeObserver                    | ⭐⭐⭐ | 元素尺寸变化响应式监听                     |
| 142 | MutationObserver                  | ⭐⭐⭐ | DOM 子树变化监听、属性变更观察             |

#### 3.4 自定义 Hooks（13 题）

| #   | 题目                         | 难度   | 核心知识点                                    |
| --- | ---------------------------- | ------ | --------------------------------------------- |
| 143 | Hook 基础：`use_counter`   | ⭐⭐   | 封装状态逻辑                                  |
| 144 | Hook 返回信号                | ⭐⭐   | `(count, set_count, increment)`             |
| 145 | Hook 参数化                  | ⭐⭐   | `use_counter(start: i32)`                   |
| 146 | Hook 依赖注入                | ⭐⭐   | `use_theme()` 读取 context                  |
| 147 | `use_local_storage`        | ⭐⭐⭐ | 封装 localStorage 读写                        |
| 148 | `use_media_query`          | ⭐⭐⭐ | 响应媒体查询                                  |
| 149 | `use_geolocation`          | ⭐⭐⭐ | Browser API 封装                              |
| 150 | `use_websocket`            | ⭐⭐⭐ | WebSocket 连接管理                            |
| 151 | `use_debounce`             | ⭐⭐⭐ | 防抖封装                                      |
| 152 | 组合多个 Hook                | ⭐⭐⭐ | `use_counter` + `use_local_storage`       |
| 153 | `use_interval` 自定义 Hook | ⭐⭐   | 封装`set_interval` + `on_cleanup` 的 Hook |
| 154 | `use_clipboard`            | ⭐⭐⭐ | 封装剪贴板读写 API                            |
| 155 | 复杂 Hook 组合               | ⭐⭐⭐ | `use_form_state` 组合验证/提交/重置逻辑     |

---

### 第 4 章：异步与资源加载（45 题）

#### 4.1 Resource（17 题）

| #   | 题目                      | 难度   | 核心知识点                              |
| --- | ------------------------- | ------ | --------------------------------------- |
| 156 | `Resource::new()` 基础  | ⭐     | 异步数据加载                            |
| 157 | Resource 依赖追踪         | ⭐⭐   | 参数变化自动重新加载                    |
| 158 | `.get()` 获取数据       | ⭐     | `resource.get()`                      |
| 159 | `.loading()` 状态       | ⭐     | `resource.loading()`                  |
| 160 | `.is_idle()` 状态       | ⭐⭐   | 空闲/加载中判断                         |
| 161 | Resource 错误处理         | ⭐⭐   | `Result` 类型                         |
| 162 | 带参数的 Resource         | ⭐⭐   | `\|id\| async move { fetch(id).await }` |
| 163 | 多参数 Resource           | ⭐⭐   | `\|(a, b)\|` 元组参数                   |
| 164 | 并行 Resource             | ⭐     | 两个 Resource 独立加载                  |
| 165 | Resource 条件触发         | ⭐⭐⭐ | 只在特定条件时发起请求                  |
| 166 | 手动`.refetch()`        | ⭐⭐   | 按钮点击刷新                            |
| 167 | Resource + Signal         | ⭐⭐   | `source=move \|\| url()`                |
| 168 | `LocalResource`         | ⭐⭐   | 非序列化数据、浏览器独有                |
| 169 | Resource vs LocalResource | ⭐⭐   | 序列化要求差异                          |
| 170 | 资源缓存                  | ⭐⭐⭐ | `stale-while-revalidate`              |
| 171 | Resource 重试与超时       | ⭐⭐⭐ | 请求失败自动重试、超时取消              |
| 172 | Resource 链式依赖         | ⭐⭐⭐ | 一个 Resource 的输出作为另一个的输入    |

#### 4.2 Suspense 与 Transition（11 题）

| #   | 题目                     | 难度   | 核心知识点                   |
| --- | ------------------------ | ------ | ---------------------------- |
| 173 | `<Suspense/>` 基础     | ⭐     | `fallback=\|\| "Loading..."` |
| 174 | Suspense 嵌套            | ⭐⭐   | 子 Suspense 独立等待         |
| 175 | Suspense + Resource      | ⭐     | 资源加载时显示 fallback      |
| 176 | `<Transition/>` 基础   | ⭐⭐⭐ | 保持旧内容，不闪白           |
| 177 | Transition 切换          | ⭐⭐⭐ | 新内容加载时保留旧 UI        |
| 178 | Suspense List            | ⭐⭐⭐ | `<SuspenseList>` 编排      |
| 179 | 瀑布请求                 | ⭐⭐   | 串行资源依赖                 |
| 180 | 并行请求优化             | ⭐⭐   | 提前 fetch 多个资源          |
| 181 | Suspense + 路由          | ⭐⭐   | 路由过渡                     |
| 182 | 自定义 fallback          | ⭐⭐   | 骨架屏                       |
| 183 | Suspense + ErrorBoundary | ⭐⭐⭐ | 资源加载失败时显示错误边界   |

#### 4.3 Action（11 题）

| #   | 题目                     | 难度   | 核心知识点                                |
| --- | ------------------------ | ------ | ----------------------------------------- |
| 184 | `Action::new()`        | ⭐⭐   | `let action = Action::new(\|input\| ...)` |
| 185 | `.dispatch()`          | ⭐⭐   | `action.dispatch(input)`                |
| 186 | `.pending()`           | ⭐⭐   | 提交中状态                                |
| 187 | `.value()`             | ⭐⭐   | 最新返回值                                |
| 188 | Action 错误              | ⭐⭐   | Result 类型错误处理                       |
| 189 | 按钮禁用逻辑             | ⭐⭐   | `button.disabled=action.pending()`      |
| 190 | 乐观更新模式             | ⭐⭐⭐ | 先改 UI 再确认                            |
| 191 | Action 与表单            | ⭐⭐   | `<form on:submit>` 结合                 |
| 192 | `create_server_action` | ⭐⭐   | 服务端 Action                             |
| 193 | 多步骤 Action            | ⭐⭐⭐ | 进度状态                                  |
| 194 | Action 异步验证          | ⭐⭐⭐ | 表单提交前服务端校验、错误回显            |

#### 4.4 定时与延时（6 题）

| #   | 题目                        | 难度   | 核心知识点                                         |
| --- | --------------------------- | ------ | -------------------------------------------------- |
| 195 | `set_interval` 轮询       | ⭐⭐   | 定时刷新数据                                       |
| 196 | `set_timeout` 延时        | ⭐⭐   | 延迟执行                                           |
| 197 | `request_animation_frame` | ⭐⭐⭐ | 帧同步动画                                         |
| 198 | 倒计时组件                  | ⭐⭐   | 组合 set_interval + 信号                           |
| 199 | 清理定时器                  | ⭐⭐   | `on_cleanup` 释放                                |
| 200 | 防抖 vs 节流                | ⭐⭐⭐ | `use_debounce` + `use_throttle` 区别和适用场景 |

---

### 第 5 章：路由（50 题）

| #   | 题目                     | 难度   | 核心知识点                                |
| --- | ------------------------ | ------ | ----------------------------------------- |
| 201 | 路由器安装               | ⭐     | `<Router>`, `<Routes>`                |
| 202 | `<Route>` 基础         | ⭐     | `path=""` `view=Component`            |
| 203 | 静态路由                 | ⭐     | `/home`, `/about`, `/contact`       |
| 204 | 嵌套路由                 | ⭐⭐   | `<ParentRoute>` + `<Outlet/>`         |
| 205 | 多级嵌套                 | ⭐⭐   | `/users/:id/profile`                    |
| 206 | `<A/>` 导航            | ⭐     | 声明式链接                                |
| 207 | `<A/>` 激活样式        | ⭐⭐   | `class:active`                          |
| 208 | 编程式导航               | ⭐⭐   | `use_navigate()()`                      |
| 209 | 导航传参                 | ⭐⭐   | navigate + query                          |
| 210 | 重定向                   | ⭐⭐   | `<Redirect/>`                           |
| 211 | 路由守卫                 | ⭐⭐⭐ | 条件重定向                                |
| 212 | 404 兜底                 | ⭐     | `fallback=\|\| "Not Found"`               |
| 213 | `path!()` 宏           | ⭐     | 类型安全路径                              |
| 214 | 路径参数                 | ⭐⭐   | `:id`, `use_params_map()`             |
| 215 | 类型化路径参数           | ⭐⭐⭐ | `use_params::<T>()` + `Params` derive |
| 216 | 可选参数                 | ⭐⭐⭐ | `:id?` 语法                             |
| 217 | 通配符参数               | ⭐⭐   | `*_` catch-all                          |
| 218 | 查询参数                 | ⭐⭐   | `?q=search`, `use_query_map()`        |
| 219 | 类型化查询参数           | ⭐⭐⭐ | `use_query::<T>()`                      |
| 220 | 路由级布局               | ⭐⭐   | Layout 组件包裹子路由                     |
| 221 | 嵌套布局                 | ⭐⭐   | 多级 Layout 组合                          |
| 222 | 布局参数传递             | ⭐⭐   | Context 跨布局传值                        |
| 223 | 路由级`<Suspense/>`    | ⭐⭐⭐ | 路由切换加载态                            |
| 224 | 路由过渡动画             | ⭐⭐⭐ | CSS transition                            |
| 225 | 活动导航高亮             | ⭐⭐   | `<A/>` 激活检测                         |
| 226 | 面包屑导航               | ⭐⭐⭐ | 基于路由树生成                            |
| 227 | Tab 切换 + URL 同步      | ⭐⭐⭐ | Tab 绑定路由                              |
| 228 | 模态框 + 路由            | ⭐⭐   | URL 控制弹窗                              |
| 229 | 滚动恢复                 | ⭐⭐⭐ | 页面切换后恢复滚动位置                    |
| 230 | `leptos_meta` 标题     | ⭐     | `<Title/>`                              |
| 231 | `<Meta/>` 标签         | ⭐⭐   | SEO meta                                  |
| 232 | `<Link/>` 标签         | ⭐⭐   | 预加载、规范链接                          |
| 233 | 动态 meta                | ⭐⭐⭐ | 根据数据生成标题                          |
| 234 | `<Style/>` 组件        | ⭐⭐   | 组件级 CSS                                |
| 235 | `<Body/>` 属性         | ⭐⭐   | body class 管理                           |
| 236 | `<Html/>` 属性         | ⭐⭐   | html lang 属性                            |
| 237 | `use_is_routing`       | ⭐⭐⭐ | 路由过渡状态                              |
| 238 | 延迟加载                 | ⭐⭐   | 代码分割                                  |
| 239 | `<Await/>` 组件        | ⭐⭐   | 直接在 view 中 await                      |
| 240 | 路由嵌套 Resource        | ⭐⭐   | 路由参数变化 → Resource 重载             |
| 241 | 多语言路由               | ⭐⭐⭐ | `/zh/home`, `/en/home`                |
| 242 | 路由单元测试             | ⭐⭐⭐ | `RouterTestHarness`                     |
| 243 | 历史模式 vs Hash 模式    | ⭐⭐   | 部署差异                                  |
| 244 | 自定义 404 页面          | ⭐     | 全页 404 设计                             |
| 245 | 路由性能优化             | ⭐⭐⭐ | lazy 加载                                 |
| 246 | 路由懒加载组件           | ⭐⭐⭐ | `create_lazy` 按需加载页面组件          |
| 247 | 路由守卫 + 角色权限      | ⭐⭐⭐ | 基于用户角色条件重定向                    |
| 248 | 编程式导航 State         | ⭐⭐   | `navigate` 传递序列化状态对象           |
| 249 | 路由过渡动画（进阶）     | ⭐⭐⭐ | 结合`use_is_routing` 实现进出场动画     |
| 250 | `<Outlet/>` 上下文传递 | ⭐⭐   | Layout 通过 Outlet 向子路由传递 props     |

---

### 第 6 章：表单、样式与开发体验（40 题）

#### 6.1 表单与输入（22 题）

| #   | 题目                   | 难度   | 核心知识点                                     |
| --- | ---------------------- | ------ | ---------------------------------------------- |
| 251 | `on:input` 事件      | ⭐     | `event_target_value(&ev)`                    |
| 252 | `on:change` 事件     | ⭐     | 失焦时触发                                     |
| 253 | 受控输入框             | ⭐⭐   | input 值绑定 signal                            |
| 254 | 非受控输入             | ⭐⭐   | NodeRef 读取值                                 |
| 255 | 文本框`<textarea>`   | ⭐     | 多行文本                                       |
| 256 | 复选框 checkbox        | ⭐⭐   | `prop:checked`                               |
| 257 | 单选 radio             | ⭐⭐   | name 分组                                      |
| 258 | 下拉框`<select>`     | ⭐⭐   | `prop:value` + `on:change`                 |
| 259 | 多选 select            | ⭐⭐⭐ | `multiple` 属性                              |
| 260 | 文件上传 input:file    | ⭐⭐⭐ | `FileList`, FormData                         |
| 261 | `<form>` 提交        | ⭐⭐   | `on:submit`, `prevent_default`             |
| 262 | 表单验证               | ⭐⭐   | 正则/自定义验证                                |
| 263 | 实时验证               | ⭐⭐   | 输入时即时校验                                 |
| 264 | 验证消息显示           | ⭐⭐   | 错误信号                                       |
| 265 | 表单状态               | ⭐⭐   | 脏/已提交/验证中                               |
| 266 | 复杂表单（多字段）     | ⭐⭐   | 多个输入组合                                   |
| 267 | 动态表单字段           | ⭐⭐⭐ | 增删表单项                                     |
| 268 | 防抖提交               | ⭐⭐⭐ | 多次触发只提交一次                             |
| 269 | 键盘快捷键             | ⭐⭐   | `keydown` 事件                               |
| 270 | 拖拽事件               | ⭐⭐⭐ | `draggable`, `ondrop`                      |
| 271 | 剪贴板事件             | ⭐⭐   | `on:paste` `on:copy` `on:cut` 事件处理   |
| 272 | 富文本 contenteditable | ⭐⭐⭐ | `contenteditable` 元素与 inner_html 双向绑定 |

#### 6.2 样式（12 题）

| #   | 题目                  | 难度   | 核心知识点                        |
| --- | --------------------- | ------ | --------------------------------- |
| 273 | `class:` 动态类     | ⭐     | `class:active=cond`             |
| 274 | 多个动态类            | ⭐⭐   | 组合多个 class:                   |
| 275 | `style:` 动态样式   | ⭐     | `style:color=color_signal`      |
| 276 | 多个 style:           | ⭐⭐   | 多个行内样式                      |
| 277 | `<Style/>` 组件     | ⭐⭐   | scoped CSS                        |
| 278 | CSS 变量绑定          | ⭐⭐   | `style:--primary=color`         |
| 279 | Tailwind CSS 集成     | ⭐⭐   | Trunk + Tailwind                  |
| 280 | 暗黑模式              | ⭐⭐   | CSS 变量 + 信号                   |
| 281 | 响应式布局            | ⭐⭐   | CSS Grid/Flexbox                  |
| 282 | CSS-in-Rust           | ⭐⭐⭐ | 条件样式计算                      |
| 283 | CSS Container Queries | ⭐⭐   | 容器查询`@container` 响应式组件 |
| 284 | 打印样式管理          | ⭐⭐   | `@media print` 样式信号控制     |

#### 6.3 开发体验（6 题）

| #   | 题目                  | 难度   | 核心知识点                                    |
| --- | --------------------- | ------ | --------------------------------------------- |
| 285 | 热更新 HMR            | ⭐     | `trunk serve` 热重载                        |
| 286 | logging / tracing     | ⭐⭐   | `console_log`, `tracing-wasm`             |
| 287 | panic 处理            | ⭐⭐   | `console_error_panic_hook`                  |
| 288 | performance profiling | ⭐⭐⭐ | 性能分析                                      |
| 289 | 源码映射              | ⭐⭐   | WASM debug symbols                            |
| 290 | 组件性能 Profiling    | ⭐⭐⭐ | Leptos`spawn_local_sync` 计时、渲染次数追踪 |

---

### 第 7 章：SSR 与 Server Functions（55 题）

**注意：** 本章改用 `cargo-leptos` 项目结构，每道题是独立的全栈项目。

| #   | 题目                     | 难度   | 核心知识点                                                |
| --- | ------------------------ | ------ | --------------------------------------------------------- |
| 291 | cargo-leptos 项目        | ⭐     | 创建/运行 SSR 项目                                        |
| 292 | CSR vs SSR 源码结构      | ⭐⭐   | `lib.rs` / `main.rs` / `app.rs`                     |
| 293 | `#[server]` 基础       | ⭐     | 第一个 Server Function                                    |
| 294 | 服务端返回数据           | ⭐     | `Result<T, ServerFnError>`                              |
| 295 | 客户端调用 Server Func   | ⭐     | `create_server_action`                                  |
| 296 | `ActionForm` 组件      | ⭐⭐   | `<ActionForm action=.../>`                              |
| 297 | 表单提交到服务端         | ⭐⭐   | ActionForm + Server Func                                  |
| 298 | 服务端验证               | ⭐⭐   | 返回验证错误                                              |
| 299 | 数据库读取（SQLite）     | ⭐⭐   | `sqlx` + Server Func                                    |
| 300 | 数据库写入               | ⭐⭐   | INSERT/UPDATE                                             |
| 301 | 连接池管理               | ⭐⭐   | `Pool` 共享                                             |
| 302 | 认证：登录               | ⭐⭐⭐ | Cookie / Session                                          |
| 303 | 认证：注册               | ⭐⭐⭐ | 密码哈希                                                  |
| 304 | 认证：登出               | ⭐⭐   | 清除 Session                                              |
| 305 | 受保护路由               | ⭐⭐   | 未登录重定向                                              |
| 306 | 中间件基础               | ⭐⭐   | 请求拦截                                                  |
| 307 | CORS 配置                | ⭐⭐   | 跨域设置                                                  |
| 308 | 文件上传                 | ⭐⭐   | multipart                                                 |
| 309 | Axum 集成                | ⭐⭐   | 自定义 Axum 路由                                          |
| 310 | Axum 共享状态            | ⭐⭐   | `Extension`, State                                      |
| 311 | SSE 推送                 | ⭐⭐⭐ | `Server-Sent Events`                                    |
| 312 | WebSocket 集成           | ⭐⭐⭐ | `tokio-tungstenite`                                     |
| 313 | 静态资源处理             | ⭐⭐   | CSS / JS / 图片                                           |
| 314 | SSR 同步渲染             | ⭐⭐   | 同步模式                                                  |
| 315 | SSR async 渲染           | ⭐⭐   | 等全部数据再发                                            |
| 316 | 流式 SSR（in-order）     | ⭐⭐⭐ | 按顺序流式发送                                            |
| 317 | 流式 SSR（out-of-order） | ⭐⭐⭐ | 无序流式                                                  |
| 318 | Hydration 基础           | ⭐⭐   | 客户端水合                                                |
| 319 | Hydration 陷阱           | ⭐⭐⭐ | CSR vs SSR 不一致                                         |
| 320 | `<Suspense/>` SSR      | ⭐⭐   | 服务端 Suspense                                           |
| 321 | 预加载数据               | ⭐⭐⭐ | `preload_data`                                          |
| 322 | SEO meta 标签（SSR）     | ⭐⭐   | 服务端 meta                                               |
| 323 | `<Title/>` SSR         | ⭐⭐   | 动态标题                                                  |
| 324 | 环境变量                 | ⭐⭐   | `dotenv`, 配置                                          |
| 325 | Dockerfile 构建          | ⭐⭐   | 多阶段构建                                                |
| 326 | 部署到 VPS               | ⭐⭐   | nginx + systemd                                           |
| 327 | 部署到 Vercel            | ⭐⭐   | Serverless                                                |
| 328 | `islands` 架构         | ⭐⭐⭐ | Islands SSR                                               |
| 329 | 代码分割 SSR             | ⭐⭐⭐ | 懒加载优化                                                |
| 330 | 缓存策略                 | ⭐⭐⭐ | HTTP 缓存                                                 |
| 331 | 日志收集                 | ⭐⭐   | `tracing` + 结构化日志                                  |
| 332 | 错误上报                 | ⭐⭐   | 服务端错误收集                                            |
| 333 | 数据库迁移               | ⭐⭐   | `sqlx migrate`                                          |
| 334 | 压力测试                 | ⭐⭐⭐ | 并发测试                                                  |
| 335 | SSR 性能优化             | ⭐⭐⭐ | 内存/CPU 优化                                             |
| 336 | OAuth 社交登录（GitHub） | ⭐⭐⭐ | OAuth 2.0 授权码流程、第三方登录集成                      |
| 337 | 服务端请求限流           | ⭐⭐⭐ | Rate Limiting 中间件、基于 IP/用户的限流策略              |
| 338 | Redis 缓存层             | ⭐⭐⭐ | `redis-rs` 集成、Server Function 响应缓存、缓存失效策略 |
| 339 | Webhook 接收器           | ⭐⭐⭐ | `#[server]` 接收外部 webhook、签名验证、事件队列        |
| 340 | 图片上传 + 缩略图        | ⭐⭐⭐ | `multipart` 图片接收、`image` crate 缩略图生成        |
| 341 | 邮件发送（SMTP）         | ⭐⭐   | `lettre` 集成、模板邮件、异步发送                       |
| 342 | 定时任务 / 调度器        | ⭐⭐⭐ | `tokio-cron-scheduler`、周期性任务、数据清理            |
| 343 | 健康检查端点             | ⭐⭐   | `/health` `/_ready` `/_live` 端点、依赖服务状态检测 |
| 344 | 错误监控集成             | ⭐⭐   | Sentry / 自定义错误上报、panic 钩子                       |
| 345 | 请求追踪与日志           | ⭐⭐   | `tracing` span 串联请求上下文、结构化 JSON 日志         |

---

### 第 8 章：高级模式（40 题）

| #   | 题目                    | 难度   | 核心知识点                                             |
| --- | ----------------------- | ------ | ------------------------------------------------------ |
| 346 | 信号 vs 全局状态        | ⭐⭐⭐ | 状态管理选型                                           |
| 347 | 不可变状态模式          | ⭐⭐⭐ | 每次 set 新值                                          |
| 348 | Redux 模式              | ⭐⭐⭐ | reducer + dispatch                                     |
| 349 | 有限状态机              | ⭐⭐⭐ | `enum` + 状态转换                                    |
| 350 | 信号选择器模式          | ⭐⭐⭐ | 派生选择器                                             |
| 351 | 异步初始化              | ⭐⭐⭐ | 启动时加载                                             |
| 352 | 生命周期监控            | ⭐⭐⭐ | scope enter/exit                                       |
| 353 | 自定义渲染              | ⭐⭐⭐ | 直接操作 DOM                                           |
| 354 | Web Worker              | ⭐⭐⭐ | 多线程计算                                             |
| 355 | IndexedDB 封装          | ⭐⭐⭐ | 本地存储                                               |
| 356 | Offline 支持            | ⭐⭐⭐ | Service Worker                                         |
| 357 | PWA 清单                | ⭐⭐   | manifest.json                                          |
| 358 | 响应式设计系统          | ⭐⭐   | 组件库封装                                             |
| 359 | 可访问性 aria           | ⭐⭐   | ARIA 属性                                              |
| 360 | 键盘导航                | ⭐⭐   | tabindex, 快捷键                                       |
| 361 | 焦点管理                | ⭐⭐⭐ | Tab 顺序, 焦点陷阱                                     |
| 362 | 通知系统                | ⭐⭐   | Toast 组件                                             |
| 363 | 模态框组件              | ⭐⭐   | Portal + 焦点管理                                      |
| 364 | 工具提示组件            | ⭐⭐   | 定位 + 显示隐藏                                        |
| 365 | 级联选择器              | ⭐⭐⭐ | 多级下拉联动                                           |
| 366 | 无限滚动                | ⭐⭐⭐ | IntersectionObserver                                   |
| 367 | 虚拟滚动                | ⭐⭐⭐ | 只渲染可见项                                           |
| 368 | 动画系统                | ⭐⭐⭐ | CSS transition / animation                             |
| 369 | WebGL 集成              | ⭐⭐⭐ | canvas + wasm                                          |
| 370 | PDF 生成                | ⭐⭐⭐ | 服务端生成                                             |
| 371 | 导出 CSV                | ⭐⭐   | 浏览器下载                                             |
| 372 | 测试组件                | ⭐⭐⭐ | `leptos::testing`                                    |
| 373 | E2E 测试                | ⭐⭐⭐ | Playwright                                             |
| 374 | CI/CD 集成              | ⭐⭐   | GitHub Actions                                         |
| 375 | 性能分析工具            | ⭐⭐⭐ | 构建分析                                               |
| 376 | Feature Flag 特性开关   | ⭐⭐⭐ | 运行时特性开关、灰度发布                               |
| 377 | A/B 测试框架            | ⭐⭐⭐ | 实验分组、指标追踪、统计显著性                         |
| 378 | WebAuthn / Passkeys     | ⭐⭐⭐ | 无密码认证、`CredentialsContainer` API               |
| 379 | WASM 体积优化           | ⭐⭐⭐ | `twiggy` 分析、按需编译、LTO 优化                    |
| 380 | 构建产物分析            | ⭐⭐   | `wasm-opt`、代码分割策略、懒加载边界                 |
| 381 | 用户行为分析            | ⭐⭐   | 自定义事件埋点、会话追踪、热力图集成                   |
| 382 | Service Worker 消息通信 | ⭐⭐⭐ | SW ↔ 页面双向消息、缓存更新通知                       |
| 383 | 属性测试 (PBT)          | ⭐⭐⭐ | `proptest` / `quickcheck` 生成测试、状态不变性验证 |
| 384 | 负载测试                | ⭐⭐⭐ | `k6` / `artillery` 并发场景、性能瓶颈定位          |
| 385 | 微前端集成模式          | ⭐⭐⭐ | 模块联邦、iframe 通信、共享响应式状态                  |

---

## 终极项目（2 个递进式一体化项目，各 40 步）

不同于前 8 章的独立练习题，这两个项目采用**递进式一体化结构**——每个项目是一个 workspace crate 内的多页应用，每道题在前一道题基础上增量开发。**第 N 题没做好，第 N+5 题直接加载失败或 UI 异常**，真实模拟大型项目的演进重构过程。

依赖推进路线示意：

```
Step 1 → Step 2 → ... → Step 40
         ↕              ↕
   基础骨架 ← 依赖 → 业务功能 ← 依赖 → 高级特性
```

每个项目分为 **8 个阶段**，每阶段 4-6 步，后一阶段强依赖前一阶段。

---

### 项目 A：ShopOS — 全栈电商管理后台（40 步）

**技术栈：** Thaw UI + leptos_router + leptos-use + Server Functions + SQLite + Axum

一个完整的 B2C 电商后台，涵盖商品、订单、用户、营销、数据五大模块。前端用 Thaw UI 搭建，后端用 Leptos SSR + SQLite。最终产物是一个可直接部署的生产级后台。

#### 阶段 1：项目骨架与数据库（5 步）

| Step | 题目                           | 前置 | 难度 | 核心知识点                                                                        |
| ---- | ------------------------------ | ---- | ---- | --------------------------------------------------------------------------------- |
| A-01 | cargo-leptos 初始化 + 项目结构 | 无   | ⭐   | `cargo leptos new`、`lib.rs`/`main.rs`/`app.rs` 职责、Axum 集成           |
| A-02 | 路由布局 + Thaw UI 主题        | A-01 | ⭐   | `<Router/>` 嵌套布局、Thaw `<ConfigProvider/>` 主题定制、全局 CSS 变量        |
| A-03 | 侧边栏导航 + 顶部栏            | A-02 | ⭐   | Thaw`<Layout/>` `<Sider/>` `<Header/>` `<Content/>`、`<Menu/>` 递归渲染 |
| A-04 | 数据库 Schema 设计 + 迁移      | A-03 | ⭐⭐ | `sqlx` migrate、ER 设计（products/categories/orders/users 六表）、`Pool` 共享 |
| A-05 | 数据库初始化 + 种子数据        | A-04 | ⭐⭐ | 启动时自动迁移、种子数据脚本、`Resource` 健康检查                               |

> **验证：** A-05 完成时数据库 6 张表就绪，启动后控制台输出 migration 日志，侧边栏导航可点击展开收起。A-04 的 Schema 设计错误会导致后续所有 CRUD 崩。

#### 阶段 2：商品管理（6 步）

| Step | 题目                  | 前置 | 难度   | 核心知识点                                                                     |
| ---- | --------------------- | ---- | ------ | ------------------------------------------------------------------------------ |
| A-06 | 商品类目管理          | A-05 | ⭐⭐   | 无限级分类树、`#[server]` 递归查询、Thaw `<Tree/>` 展示                    |
| A-07 | 商品列表页            | A-06 | ⭐⭐   | Thaw`<Table/>`、`Resource` 分页查询、`use_params_map` 页码同步 URL       |
| A-08 | 商品搜索 + 多条件过滤 | A-07 | ⭐⭐   | leptos-use`use_debounce`、高级筛选面板、URL query 参数同步                   |
| A-09 | 新增商品表单          | A-08 | ⭐⭐   | Thaw`<Form/>` `<Input/>` `<Select/>` `<DatePicker/>`、复杂表单验证     |
| A-10 | 编辑商品 + SKU 管理   | A-09 | ⭐⭐⭐ | 多 SKU 动态表单（`For` 增删行）、库存字段联动、`Action` 乐观更新           |
| A-11 | 商品详情页 + 图片画廊 | A-10 | ⭐⭐   | 路由参数`:id`、动态 `<Title/>`、Thaw `<Image/>` `<Carousel/>` 画廊组件 |

> **验证：** A-07 列表页的"新增"按钮跳到 A-09 表单页，新增成功后列表自动刷新。A-10 的 SKU 表单如果验证有 Bug，添加商品会失败。A-11 的 404 状态没有处理的话，访问不存在的商品 ID 会白屏。

#### 阶段 3：用户与认证系统（5 步）

| Step | 题目                | 前置 | 难度 | 核心知识点                                                 |
| ---- | ------------------- | ---- | ---- | ---------------------------------------------------------- |
| A-12 | 用户注册 + 密码哈希 | A-11 | ⭐⭐ | Server Action 注册、`argon2` 密码哈希、唯一性校验        |
| A-13 | 登录 + Session 管理 | A-12 | ⭐⭐ | `create_server_action`、Cookie Session、`axum_session` |
| A-14 | 用户信息页 + 编辑   | A-13 | ⭐⭐ | 受保护路由（未登录重定向）、`use_context` 获取当前用户   |
| A-15 | 收货地址管理        | A-14 | ⭐⭐ | 多地址 CRUD、默认地址设置、Thaw`<Table/>` 行操作         |
| A-16 | 密码修改 + 账号安全 | A-15 | ⭐⭐ | 旧密码校验、Session 失效处理、登录日志记录                 |

> **验证：** A-13 写完才能登录，不登录的话 A-14 受保护路由直接重定向。A-12 注册的账号在 A-13 登不上说明 Session 有问题。A-16 修改密码后旧 Session 必须失效。

#### 阶段 4：购物车与订单（6 步）

| Step | 题目                 | 前置 | 难度   | 核心知识点                                                           |
| ---- | -------------------- | ---- | ------ | -------------------------------------------------------------------- |
| A-17 | 购物车（本地持久化） | A-16 | ⭐⭐   | leptos-use`use_local_storage`、购物车 Signal 派生计算（总价/数量） |
| A-18 | 购物车页 + 数量/删除 | A-17 | ⭐⭐   | Thaw`<Table/>` 编辑模式、`update` 原地改值、空购物车 fallback    |
| A-19 | 下单结算页           | A-18 | ⭐⭐⭐ | 地址选择、商品确认、Thaw`<Steps/>` 分步表单、金额汇总              |
| A-20 | 订单创建（事务）     | A-19 | ⭐⭐⭐ | 数据库事务、库存扣减 + 回滚、`ServerFnError` 自定义错误            |
| A-21 | 订单列表 + 筛选      | A-20 | ⭐⭐⭐ | 多状态查询（待付款/已付款/已发货/已完成/已取消）、Thaw`<Tabs/>`    |
| A-22 | 订单详情 + 状态流转  | A-21 | ⭐⭐⭐ | 状态机（`enum OrderState` + 合法转换）、操作按钮条件显示           |

> **验证：** A-17 购物车数据存在 localStorage，清空浏览器数据再打开购物车为空则正常。A-20 创建订单时如果库存扣减失败，事务不回滚会导致超卖。A-22 订单从"待付款"不能直接跳到"已完成"，状态机拦截失败的应该在 UI 上禁用按钮。

#### 阶段 5：运营功能（5 步）

| Step | 题目                   | 前置 | 难度   | 核心知识点                                                   |
| ---- | ---------------------- | ---- | ------ | ------------------------------------------------------------ |
| A-23 | 优惠券系统（管理员端） | A-22 | ⭐⭐⭐ | 优惠券 CRUD、有效期校验、使用条件、Thaw`<Form/>` 动态规则  |
| A-24 | 优惠券核销（用户端）   | A-23 | ⭐⭐⭐ | 结算页输入优惠码、服务端验证 + 折扣计算、`Action` 错误回显 |
| A-25 | 退货/退款流程          | A-24 | ⭐⭐⭐ | 售后单创建、审批流转（发起→审核→退款→完成）、状态机扩展   |
| A-26 | 物流追踪               | A-25 | ⭐⭐⭐ | 物流信息表设计、模拟物流进度、Thaw`<Timeline/>` 时间线     |
| A-27 | 发票管理               | A-26 | ⭐⭐⭐ | 发票申请、PDF 生成预览、下载链接                             |

> **验证：** A-24 优惠券过期了在结算页应该提示"已过期"，而不是在提交订单时报错。A-25 退款流程中如果商品已发货需要先拦截物流。

#### 阶段 6：数据分析与通知（4 步）

| Step | 题目            | 前置 | 难度   | 核心知识点                                                                       |
| ---- | --------------- | ---- | ------ | -------------------------------------------------------------------------------- |
| A-28 | 数据仪表盘      | A-27 | ⭐⭐⭐ | 聚合查询（GROUP BY）、Thaw`<Statistic/>` `<Card/>` 指标卡、ECharts/WASM 图表 |
| A-29 | 实时通知（SSE） | A-28 | ⭐⭐⭐ | Server-Sent Events、`EventSource`、Thaw `<Notification/>` 弹出通知           |
| A-30 | 操作审计日志    | A-29 | ⭐⭐   | 中间件拦截记录操作、日志列表查询与筛选、Thaw`<Table/>` 展示                    |
| A-31 | 系统配置        | A-30 | ⭐⭐   | KV 配置表读写、Thaw`<Form/>` 动态配置项渲染                                    |

> **验证：** A-28 仪表盘的数据来自前面所有业务的聚合，任何一步数据格式不对仪表盘就会显示异常。A-29 新订单产生时后台右上角弹出通知。

#### 阶段 7：工程化与部署（4 步）

| Step | 题目                  | 前置 | 难度   | 核心知识点                                                            |
| ---- | --------------------- | ---- | ------ | --------------------------------------------------------------------- |
| A-32 | 多语言 i18n           | A-31 | ⭐⭐⭐ | leptos-use`use_i18n`、语言切换 Signal、翻译 Key 管理                |
| A-33 | 暗黑模式 + 响应式布局 | A-32 | ⭐⭐   | CSS 变量切换、`use_media_query`、移动端侧边栏折叠                   |
| A-34 | 单元测试 + 集成测试   | A-33 | ⭐⭐⭐ | `#[cfg(test)]` Server Function 测试、`RouterTestHarness` 组件测试 |
| A-35 | Docker 构建 + CI/CD   | A-34 | ⭐⭐   | 多阶段 Dockerfile、`.github/workflows` 自动构建、nginx 配置         |

> **验证：** A-34 所有测试通过后才能部署。A-35 构建产物小于 10MB，启动后首页 < 1s 响应。

#### 阶段 8：增强功能与数据价值（5 步）

| Step | 题目             | 前置 | 难度   | 核心知识点                                                  |
| ---- | ---------------- | ---- | ------ | ----------------------------------------------------------- |
| A-36 | 商品批量导入     | A-35 | ⭐⭐⭐ | Excel/CSV 解析、`calamine` 读取、批量事务写入、进度条反馈 |
| A-37 | 商品评价系统     | A-36 | ⭐⭐   | 评价 CRUD、评分星选组件、Thaw`<Rate/>`、评价列表分页      |
| A-38 | 数据导出报表     | A-37 | ⭐⭐   | 聚合查询（销售/用户/商品）、CSV 流式下载、XLSX 格式支持     |
| A-39 | 支付流水对账     | A-38 | ⭐⭐⭐ | 支付记录表设计、对账差异比对、`Action` 标记异常           |
| A-40 | API 文档自动生成 | A-39 | ⭐⭐   | `utoipa` / OpenAPI 集成、Swagger UI 路由挂接、接口调试页  |

> **验证：** A-36 导入 1 万条商品数据应在 5 秒内完成。A-38 导出的 CSV 用 Excel 打开不乱码。A-40 OpenAPI 页面的每个端点点击后返回正确数据。

**ShopOS 完整递进依赖图：**

```
A-01(脚手架) → A-02(布局) → A-03(导航) → A-04(Schema) → A-05(种子)
                                                          ↓
A-12(注册) ← A-11(详情) ← A-10(SKU) ← A-09(新增) ← A-08(搜索) ← A-07(列表) ← A-06(类目)
    ↓           ↓                                 ↑                  ↓
A-13(登录) → A-14(信息) → A-15(地址) → A-16(安全)  A-17(购物车) → A-18(购物车页)
                                                          ↓
A-28(仪表盘) ← A-27(发票) ← A-26(物流) ← A-25(退款) ← A-24(优惠券核销) ← A-23(优惠券管理) ← A-22(详情+状态) ← A-21(订单列表) ← A-20(创建) ← A-19(结算)
    ↓                                                                                               ↑
A-29(通知) → A-30(审计) → A-31(配置) → A-32(i18n) → A-33(暗黑) → A-34(测试) → A-35(部署)
    ↓
A-36(批量导入) → A-37(评价) → A-38(报表) → A-39(对账) → A-40(API文档)
```

任何一步的 Bug 都会直接阻断后续 3-5 步的正常工作。

---

### 项目 B：NoteFlow — 实时协作知识库（40 步）

**技术栈：** Thaw UI + leptos_router + leptos-use + WebSocket + IndexedDB + PWA + Y.js (CRDT)

一个支持 Markdown 实时协作、离线编辑、多端同步的现代知识库。CSR 为主，SSR 辅助用于 SEO 和分享页。最终产物是一个 PWA，可在手机和电脑上离线使用。

#### 阶段 1：编辑器与文档核心（5 步）

| Step | 题目                    | 前置 | 难度 | 核心知识点                                                               |
| ---- | ----------------------- | ---- | ---- | ------------------------------------------------------------------------ |
| B-01 | 项目初始化 + 文档路由   | 无   | ⭐   | Trunk CSR 脚手架、`<Router/>` 嵌套布局、Thaw UI 安装                   |
| B-02 | 文档树侧边栏            | B-01 | ⭐⭐ | Thaw`<Tree/>` 递归组件、树形数据 Signal、折叠/展开状态                 |
| B-03 | Markdown 编辑器（受控） | B-02 | ⭐⭐ | 受控`<textarea>` + 实时预览（`comrak` WASM）、分屏布局               |
| B-04 | 代码高亮 + 数学公式     | B-03 | ⭐⭐ | 语法高亮（`syntect`/`highlight.js`）、KaTeX WASM 渲染                |
| B-05 | 本地持久化 IndexedDB    | B-04 | ⭐⭐ | leptos-use`use_indexed_db`、自动保存（`watch` + debounce）、草稿恢复 |

> **验证：** B-03 编辑器的输入/预览不同步则后续所有编辑功能全废。B-05 关闭页面再打开，文档内容应恢复。IndexedDB 写失败时应有 fallback。

#### 阶段 2：文档管理（5 步）

| Step | 题目         | 前置 | 难度   | 核心知识点                                                                   |
| ---- | ------------ | ---- | ------ | ---------------------------------------------------------------------------- |
| B-06 | 文档 CRUD    | B-05 | ⭐⭐   | 新建/删除/重命名、Thaw`<Modal/>` 确认弹窗、快捷键支持                      |
| B-07 | 多标签页编辑 | B-06 | ⭐⭐   | Tab 组件 + URL 路由同步、Tab 关闭/切换/拖拽排序                              |
| B-08 | 目录拖拽排序 | B-07 | ⭐⭐⭐ | HTML5 Drag & Drop API、`ondragstart`/`ondragover`/`ondrop`、树结构更新 |
| B-09 | 文档模板     | B-08 | ⭐⭐   | 预置模板（会议记录/周报/需求文档）、模板变量替换                             |
| B-10 | 导入/导出    | B-09 | ⭐⭐   | Markdown 文件导入拖拽上传、导出`.md`/`.pdf` 文件下载                     |

> **验证：** B-06 删除文档没有二次确认则可能误删。B-08 拖拽到错误位置不会自动恢复。B-10 导出的 Markdown 格式必须与编辑器中一致。

#### 阶段 3：组织与检索（5 步）

| Step | 题目              | 前置 | 难度   | 核心知识点                                                    |
| ---- | ----------------- | ---- | ------ | ------------------------------------------------------------- |
| B-11 | 标签系统          | B-10 | ⭐⭐   | 标签 CRUD、Thaw`<Tag/>` `<Select/>` 多选、标签颜色        |
| B-12 | 分类 + 嵌套文件夹 | B-11 | ⭐⭐   | 文件夹树、移动文档、面包屑`<Breadcrumb/>`                   |
| B-13 | 全文搜索          | B-12 | ⭐⭐⭐ | leptos-use`use_debounce` + IndexedDB 全文索引、搜索结果高亮 |
| B-14 | 高级筛选          | B-13 | ⭐⭐   | 标签/日期/类型组合筛选、保存筛选条件为"视图"、URL query       |
| B-15 | 最近访问 + 收藏   | B-14 | ⭐⭐   | leptos-use`use_local_storage` 记录历史、星标收藏列表        |

> **验证：** B-13 搜索"Rust"应该匹配标题和正文，搜索结果点击后路由跳转到对应文档。B-15 收藏列表跨页面保持一致。

#### 阶段 4：用户与团队（5 步）

| Step | 题目                 | 前置 | 难度   | 核心知识点                                              |
| ---- | -------------------- | ---- | ------ | ------------------------------------------------------- |
| B-16 | 用户注册/登录        | B-15 | ⭐⭐   | leptos-use`use_local_storage` JWT token、受保护路由   |
| B-17 | 工作区/团队管理      | B-16 | ⭐⭐   | 多 workspace 切换、邀请链接生成、Thaw`<Select/>`      |
| B-18 | 成员管理 + 角色      | B-17 | ⭐⭐   | 成员列表、角色（所有者/管理员/编辑者/查看者）、权限枚举 |
| B-19 | 文档级权限           | B-18 | ⭐⭐⭐ | 文档 ACL（继承+覆盖）、权限校验 Signal、UI 按钮条件禁用 |
| B-20 | 操作历史 + 动态 Feed | B-19 | ⭐⭐   | 活动日志（编辑/评论/移动）、Thaw`<Timeline/>` 时间线  |

> **验证：** B-16 未登录时 B-17 的页面应该重定向到登录页。B-19 查看者点击"编辑"按钮应该被禁用或隐藏。Token 过期后 API 返回 401 应自动跳转登录页。

#### 阶段 5：高级协作（7 步）

| Step | 题目                | 前置 | 难度   | 核心知识点                                                        |
| ---- | ------------------- | ---- | ------ | ----------------------------------------------------------------- |
| B-21 | WebSocket 连接管理  | B-20 | ⭐⭐⭐ | `ws://` 连接建立/重连/心跳、`on_cleanup` 断开清理             |
| B-22 | 在线状态 + 光标同步 | B-21 | ⭐⭐⭐ | 在线用户列表、远程光标位置渲染（`div` 绝对定位）                |
| B-23 | Y.js CRDT 集成      | B-22 | ⭐⭐⭐ | `y-crdt` WASM 绑定、`Y.Doc` 共享数据结构、操作合并            |
| B-24 | 实时内容同步        | B-23 | ⭐⭐⭐ | `y-textarea` 双向绑定、冲突自动解决、`watch` 响应远程变更     |
| B-25 | 离线编辑 + 同步     | B-24 | ⭐⭐⭐ | Y.js offline 模式、IndexedDB 持久化、同步冲突 UI 提示             |
| B-26 | 版本历史 + 时间旅行 | B-25 | ⭐⭐⭐ | Y.js`yjs/undo`、`Y.UndoManager`、版本滑动条、Diff 差异对比    |
| B-27 | 评论/批注系统       | B-26 | ⭐⭐⭐ | 选中文本批注、评论线程、Thaw`<Comment/>` `<Popover/>`、@ 提及 |

> **验证：** B-23/B-24 是整个项目最难的关卡——多人同时编辑时文字不能丢失。打开两个浏览器窗口编辑同一个文档，A 窗口输入的内容应在 < 200ms 内出现在 B 窗口。B-25 断网编辑的内容重连后应自动同步。B-26 撤销操作不影响其他人的编辑。

#### 阶段 6：PWA 与用户体验（4 步）

| Step | 题目                  | 前置 | 难度   | 核心知识点                                                               |
| ---- | --------------------- | ---- | ------ | ------------------------------------------------------------------------ |
| B-28 | PWA + Service Worker  | B-27 | ⭐⭐⭐ | `manifest.json`、Service Worker 缓存策略、离线启动页                   |
| B-29 | 桌面通知 + 后台同步   | B-28 | ⭐⭐⭐ | `Notification API`、`Background Sync`、`on_visibility_change`      |
| B-30 | 快捷键 + 命令面板     | B-29 | ⭐⭐⭐ | `window_event_listener(keydown)`、Thaw `<Command/>` 面板、快捷键映射 |
| B-31 | 暗黑模式 + 自定义主题 | B-30 | ⭐⭐   | CSS 变量体系、`use_media_query` 自动检测、主题 Signal 持久化           |

> **验证：** B-28 PWA 安装后打开 DevTools Application 面板应看到 Service Worker 激活。B-30 按 `Cmd+K` 弹出命令面板，输入"新建文档"回车应触发 B-06 的新建逻辑。

#### 阶段 7：工程化与部署（4 步）

| Step | 题目                 | 前置 | 难度   | 核心知识点                                                             |
| ---- | -------------------- | ---- | ------ | ---------------------------------------------------------------------- |
| B-32 | 看板视图（数据关联） | B-31 | ⭐⭐⭐ | 文档状态作为看板列、HTML5 Drag & Drop 跨列移动、状态同步回 Y.js        |
| B-33 | 统计分析             | B-32 | ⭐⭐   | 文档数/字数统计、编辑频率图表、Thaw`<Statistic/>`                    |
| B-34 | 测试 + 性能分析      | B-33 | ⭐⭐⭐ | `wasm-bindgen-test`、`twiggy` WASM 体积分析、lazy loading 代码分割 |
| B-35 | SSR 分享页 + 部署    | B-34 | ⭐⭐   | `cargo-leptos` 补充 SSR 路由、SEO meta、Docker 部署                  |

> **验证：** B-34 WASM 产物 > 5MB 则优化不达标。B-35 分享页在无 JS 环境下应渲染出 Markdown HTML 内容。

#### 阶段 8：高级功能与生态集成（5 步）

| Step | 题目             | 前置 | 难度   | 核心知识点                                                     |
| ---- | ---------------- | ---- | ------ | -------------------------------------------------------------- |
| B-36 | 文档目录大纲     | B-35 | ⭐⭐   | Markdown 标题解析生成 ToC、IntersectionObserver 滚动高亮       |
| B-37 | 文档内部链接图谱 | B-36 | ⭐⭐⭐ | `[[wikilink]]` 语法解析、反向链接列表、关联图可视化          |
| B-38 | 只读分享链接     | B-37 | ⭐⭐   | 加密 token 生成、过期时间、权限校验中间件、分享页 SSR          |
| B-39 | 专注写作模式     | B-38 | ⭐⭐   | 全屏编辑器、打字机滚动、字数目标、`use_media_query` 暗色主题 |
| B-40 | AI 辅助写作集成  | B-39 | ⭐⭐⭐ | LLM API 调用（`gloo-net`）、流式补全建议、翻译/润色命令面板  |

> **验证：** B-36 滚动时当前标题应在 ToC 中高亮。B-37 创建 `[[Other Doc]]` 后自动出现链接引用提示。B-38 未登录访问分享链接应展示只读内容而不是跳转登录页。B-40 AI 补全请求应在 < 2s 内返回首批 token。

**NoteFlow 完整递进依赖图：**

```
B-01(脚手架) → B-02(树) → B-03(编辑器) → B-04(高亮) → B-05(IndexedDB)
                                                          ↓
B-16(注册) ← B-15(收藏) ← B-14(筛选) ← B-13(搜索) ← B-12(分类) ← B-11(标签) ← B-10(导出) ← B-09(模板) ← B-08(拖拽) ← B-07(Tab) ← B-06(CRUD)
    ↓                                                                                                                                 ↓
B-17(工作区) → B-18(成员) → B-19(权限) → B-20(动态) → B-21(WS连接) → B-22(光标) → B-23(Y.js) → B-24(实时同步) → B-25(离线) → B-26(版本) → B-27(批注)
                                                                                                                            ↓
                                                                                                      B-32(看板) ← B-31(主题) ← B-30(快捷键) ← B-29(通知) ← B-28(PWA)
                                                                                                          ↓
                                                                                                      B-33(统计) → B-34(测试) → B-35(部署)
                                                                                                                                  ↓
                                                                                                          B-36(ToC) → B-37(链接) → B-38(分享) → B-39(专注) → B-40(AI)
```

**最关键的依赖链：** B-03 编辑器 → B-05 持久化 → B-06 CRUD → B-13 搜索 → B-21 WebSocket → B-23 Y.js → B-24 实时同步。这条链上任意一步有 Bug，后续协作功能全崩。

---

## 验证标准（两个项目通用）

| 检查项       | 说明                                           |
| ------------ | ---------------------------------------------- |
| 编译检查     | `cargo build` / `trunk build` 零错误零警告 |
| 路由完整性   | 所有页面路由可达，404 兜底正常                 |
| 数据持久化   | 刷新后数据不丢失（IndexedDB / SQLite）         |
| 错误边界     | Server Function 失败时 UI 显示错误提示而非白屏 |
| 表单验证     | 必填字段为空时禁止提交并显示验证消息           |
| 状态机合法性 | 订单/文档状态只能按合法路径转换                |
| 未授权访问   | 未登录状态重定向到登录页而非显示空白           |
| 响应式布局   | 移动端侧边栏折叠、表格水平滚动                 |
| 构建产物体积 | CSR < 5MB，SSR < 20MB（Docker 镜像）           |
| 递进兼容     | Step N 的修改不破坏 Step N-1 的已有功能        |

---

## 关键决策

1. **前 6 章纯 CSR + Trunk**：零服务器依赖，`trunk serve` 开箱即用。第 7 章独立为 `cargo-leptos` SSR。
2. **每个练习独立 crate**：100 Exercises 风格，可独立编译运行，无命名冲突。依赖仅在需要时递增添加。
3. **Leptos 0.9.x + nightly**：

   - 函数调用语法 `foo()` / `set_foo(42)`，更简洁
   - 最新 `leptos_router` API（`path!()`、`ParentRoute`、`use_params_map`）
   - `rust-toolchain.toml` 锁定 nightly
4. **极细粒度 + 多解法**：同一个知识点拆成多题。例如计数器有 `.set()`、`.update()`、`.write()`、函数调用四种写法各一题。
5. **TODO 中文注释引导**：描述意图而非实现，逐步减少提示量。
6. **每题包含答案折叠**：`<details>` 标签折叠参考答案 + 知识点说明。

---

## 实施步骤

### 第 1 步：创建项目骨架（~1h）

- `Cargo.toml` workspace 配置
- `rust-toolchain.toml`
- `00_preface/` 练习导航首页
- `scripts/new-exercise.ps1` 脚手架脚本

### 第 2 步：逐章编写（按流水线顺序）

1. 第 1 章（20 题）→ 验证每个 `trunk build`
2. 第 2 章（75 题）→ 逐步验证
3. 第 3 章（60 题）
4. 第 4 章（45 题）
5. 第 5 章（50 题）
6. 第 6 章（40 题）
7. 第 7 章（55 题）→ 用 `cargo leptos build`
8. 第 8 章（40 题）
9. Projects（10 个）

### 第 3 步：验证

- 每章完成后 `trunk build` 确保无编译错误
- 抽样 `trunk serve` 浏览器预览
- 检查 `<details>` 标签正确性
- 确保所有 `// TODO:` 有对应答案

---

## 环境准备

```bash
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install cargo-leptos
```

---

## 工作量估计

| 阶段           | 内容               | 题量           | 估计时间        |
| -------------- | ------------------ | -------------- | --------------- |
| 骨架           | workspace + 脚手架 | -              | ~1 h            |
| 第 1 章        | 基础与环境         | 20             | ~2 h            |
| 第 2 章        | 响应式系统         | 75             | ~9 h            |
| 第 3 章        | 组件进阶           | 60             | ~7 h            |
| 第 4 章        | 异步与资源         | 45             | ~5.5 h          |
| 第 5 章        | 路由               | 50             | ~5.5 h          |
| 第 6 章        | 表单/样式/DX       | 40             | ~4.5 h          |
| 第 7 章        | SSR 全栈           | 55             | ~8.5 h          |
| 第 8 章        | 高级模式           | 40             | ~5.5 h          |
| 项目 A         | ShopOS 电商后台    | 40             | ~16 h           |
| 项目 B         | NoteFlow 知识库    | 40             | ~16 h           |
| 验证           | 编译+预览          | 全部           | ~5 h            |
| **总计** |                    | **~465** | **~88 h** |
