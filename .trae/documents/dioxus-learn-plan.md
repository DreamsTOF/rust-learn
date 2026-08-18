# Dioxus 练习项目 — 实现计划

## 概述

在 `c:\code\testruetlearn\` 下创建 `dioxus-learn/` 目录，Cargo workspace 内含约 **535 项练习内容**（415 道独立题 + 120 步终极项目），覆盖从 rsx! 宏到全栈实战的 12 章内容 + 2 个综合项目。每道题都是独立可运行的 Dioxus Web 应用（`dx serve` 一键预览），**答案单独放在 `*_answer/` 文件夹中**，练习与答案完全分离。

参照 **100 Exercises to Learn Rust** 风格：每个练习是一个独立的 Cargo crate，可编译可运行，所见即所得。

---

## 项目结构

```
c:\code\testruetlearn\dioxus-learn\
├── Cargo.toml                   # workspace（管理所有练习 crate）
├── rust-toolchain.toml          # stable 工具链锁定
│
├── 00_setup/                    # 环境准备（10 题）
├── 01_basics/                   # 第 1 章（20 题）
├── 02_signals/                  # 第 2 章（75 题）
├── 03_components/               # 第 3 章（50 题）
├── 04_events_forms/             # 第 4 章（40 题）
├── 05_async/                    # 第 5 章（40 题）
├── 06_router/                   # 第 6 章（45 题）
├── 07_context_state/            # 第 7 章（35 题）
├── 08_fullstack/                # 第 8 章（40 题）
├── 09_advanced/                 # 第 9 章（40 题）
├── 10_verification/             # 验证与优化（20 题）
│
└── projects/                    # 综合实战（完整应用级练习）
    ├── hotdog_app/              # HotDog 项目（60 步）
    └── todo_dashboard/          # TodoDashboard 项目（60 步）
```

### 练习与答案分离

每道题都有两个独立 crate：

```
e001_install_rust/          ← 练习文件（含 TODO，不含答案）
e001_install_rust_answer/   ← 参考答案（完整可编译，不含 TODO）
```

练习者先尝试完成练习，遇到困难参考答案。两个 crate 都可以独立 `dx build` 和 `dx serve`。

---

## 技术选型

| 项目            | 选型                                                   | 原因                                                                     |
| --------------- | ------------------------------------------------------ | ------------------------------------------------------------------------ |
| Dioxus 版本     | **0.7.x**（最新稳定版 0.7.9）                          | 2026 年最新，支持 stable Rust，文档完善                                  |
| Rust 通道       | **stable**（1.85+）                                    | Dioxus 0.7 完全支持 stable Rust，无需 nightly                            |
| 构建工具        | **dx**（dioxus-cli）                                   | 官方 CLI，支持热重载、打包、多平台部署                                   |
| 路由            | **dioxus** `features = ["router"]`                     | 内置类型安全路由，`#[derive(Routable)]` 声明式路由                       |
| 全栈            | **dioxus** `features = ["fullstack"]`                  | 基于 Axum 的 SSR + Server Functions，支持流式 SSR                       |
| 信号系统        | **dioxus-signals**（内置）                             | 细粒度响应式模型，`Copy` 语义，自动批处理                                |
| 数据获取        | **use_resource** + **SuspenseBoundary**（内置）       | 异步数据加载 + 暂停边界                                                  |
| 全局状态        | **Signal::global()** + **use_context**（内置）        | 全局信号 + 上下文提供者                                                  |
| 细粒度集合      | **dioxus-stores**（内置）                              | Store derive 宏，逐字段/逐条目响应式更新                                 |
| UI 组件库       | 不强制（可选 TailwindCSS 或 DaisyUI）                  | 保持轻量，练习题聚焦 Dioxus 核心 API                                     |
| 工具库          | **serde**, **anyhow**, **reqwest** (wasm)              | 序列化、错误处理、HTTP 请求                                              |

### 依赖管理

```toml
[workspace]
members = [
    "00_setup/e001_install_rust",
    "00_setup/e001_install_rust_answer",
    "01_basics/e011_hello_world",
    "01_basics/e011_hello_world_answer",
]

[workspace.dependencies]
dioxus = { version = "0.7", features = ["web", "router"] }
dioxus-logger = "0.6"
serde = { version = "1", features = ["derive"] }
anyhow = "1"
reqwest = { version = "0.12", features = ["wasm"] }
gloo-net = "0.6"
wasm-bindgen = "0.2"
futures = "0.3"
gloo-timers = "0.3"

dioxus-fullstack = { version = "0.7", features = ["axum"] }
axum = "0.8"
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
tokio = { version = "1", features = ["full"] }
tower-sessions = "0.13"
serde_json = "1"
```

### Dioxus 0.7 API 风格

```rust
use dioxus::prelude::*;

fn App() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    }
}

fn main() {
    dioxus::launch(App);
}
```

### 关键 API 对照（Leptos -> Dioxus）

| 概念          | Leptos 0.9                     | Dioxus 0.7                              |
| ------------- | ------------------------------ | --------------------------------------- |
| 宏            | `view! { }`                    | `rsx! { }`                              |
| 启动          | `mount_to_body(App)`           | `dioxus::launch(App)`                   |
| 信号创建      | `let (r, w) = signal(0)`       | `let mut s = use_signal(|| 0)`          |
| 信号读取      | `count()` / `count.get()`      | `count()` / `count.read()`              |
| 信号写入      | `set_count(42)`                | `count.set(42)` / `count += 1`          |
| 派生信号      | `Memo::new()`                  | `use_memo(move || count() * 2)`         |
| 副作用        | `Effect::new()`                | `use_effect(move || { ... })`           |
| 组件返回      | `impl IntoView`                | `Element` (Result<VNode>)               |
| 路由          | `<Routes>` tags                | `Router::<Route> {}` + `#[derive(Routable)]` |
| 服务端函数    | `#[server]`                    | `#[get("/api/...")]`                    |
| 异步资源      | `Resource::new()`              | `use_resource(move || async { })`       |
| 暂停边界      | `<Suspense/>`                  | `SuspenseBoundary { fallback: ..., }`   |
| 错误处理      | 通过 Result 返回               | `?` + `ErrorBoundary`                   |

### 构建与工具链

```toml
[toolchain]
channel = "stable"
targets = ["wasm32-unknown-unknown"]
```

```bash
cargo install dioxus-cli
cd 01_basics/e011_hello_world && dx serve --port 8080
dx build --release
```

---

## 难度分布规则

每 5 题一组，难度递增：

| 偏移 | 难度   | 引导程度               | 代码完成度     |
| ---- | ------ | ---------------------- | -------------- |
| +0   | ⭐     | 每行都有详细 TODO      | 只需填空       |
| +1   | ⭐     | 每行都有详细 TODO      | 只需填空       |
| +2   | ⭐⭐   | 关键位置有 TODO        | 补全 50%       |
| +3   | ⭐⭐   | 少量提示               | 补全 50%       |
| +4   | ⭐⭐⭐ | 仅描述目标             | 几乎全部自己写 |

---

## 内容大纲（12 章，约 415 道独立题 + 120 步项目）

### 第 0 章：环境准备（10 题，e001-e010）

**目标：** 搭建 Dioxus 开发环境，安装工具链，运行第一个 Dioxus 应用。

| #  | 题目              | 难度 | 核心知识点                                        |
| -- | ----------------- | ---- | ------------------------------------------------- |
| 01 | 安装 Rust         | ⭐   | `rustup`、`wasm32-unknown-unknown` target          |
| 02 | 安装 Dioxus CLI   | ⭐   | `cargo install dioxus-cli`、`dx doctor`             |
| 03 | 创建第一个项目    | ⭐   | `dx new`、理解目录结构                              |
| 04 | 运行开发服务器    | ⭐   | `dx serve`、热重载体验                             |
| 05 | 编辑器配置        | ⭐   | VSCode 插件、rust-analyzer、Dioxus 扩展             |
| 06 | rust-toolchain    | ⭐⭐ | 工具链锁定、团队一致性                              |
| 07 | 第一个组件        | ⭐⭐ | `fn App() -> Element`、`rsx!`                       |
| 08 | 构建生产版本      | ⭐⭐ | `dx build --release`                                |
| 09 | 浏览器开发者工具  | ⭐⭐ | WASM 调试、console.log                              |
| 10 | 项目模板化        | ⭐⭐ | 使用模板创建项目                                    |

### 第 1 章：基础（20 题，e011-e030）

**目标：** 理解 `rsx!` 宏和组件基本概念。

| #  | 题目                | 难度 | 核心知识点                                               |
| -- | ------------------- | ---- | -------------------------------------------------------- |
| 11 | Hello World         | ⭐   | `dioxus::launch`, `rsx!` 宏, `fn App() -> Element`       |
| 12 | 文本节点            | ⭐   | 字符串文本 `"..."`、`Display` 格式化 `{expr}`             |
| 13 | HTML 元素与属性     | ⭐   | `class`, `id`, `style`, `<a>`, `<img>`                   |
| 14 | 元素嵌套            | ⭐   | `<div>`, `<section>`, 层级结构                           |
| 15 | 组件定义与调用      | ⭐⭐ | `#[component]`, `fn Name() -> Element`                   |
| 16 | 组件嵌套            | ⭐⭐ | `<App/>` 嵌套 `<Header/>` `<Main/>`                       |
| 17 | Fragment 语法       | ⭐⭐ | `Fragment { }` 组件、多根节点                              |
| 18 | 注释写法            | ⭐   | `rsx!` 内的注释                                           |
| 19 | Rust 表达式嵌入     | ⭐⭐ | `{ }` 块、变量插值                                        |
| 20 | 块级表达式          | ⭐⭐ | `{ let x = 1; x + 2 }`                                   |
| 21 | 条件 if 在 rsx 中   | ⭐⭐⭐ | `if cond { rsx!{...} } else { rsx!{...} }`               |
| 22 | 匹配 match 在 rsx   | ⭐⭐⭐ | `match x { 1 => rsx!{...}, _ => rsx!{...} }`             |
| 23 | 列表渲染：迭代器    | ⭐⭐ | `{(0..10).map(\|i\| rsx!{ ... })}`                        |
| 24 | 列表渲染：for 循环  | ⭐⭐⭐ | `for item in items.iter() { ... }`                        |
| 25 | key 属性            | ⭐⭐ | `key: "{item.id}"`                                        |
| 26 | SVG 元素            | ⭐⭐ | `<svg>`, `<circle>`, `<rect>`                             |
| 27 | dangerous_inner_html| ⭐⭐ | XSS 防范                                                  |
| 28 | 内联与 style 属性   | ⭐⭐ | `style: "..."`, 属性化 style                              |
| 29 | 动态标签名          | ⭐⭐ | 根据变量值动态渲染标签                                    |
| 30 | 属性展开 spread     | ⭐⭐⭐ | `..attributes` 语法                                       |

### 第 2 章：响应式系统（75 题，e031-e105）

**目标：** 深入理解 Dioxus 的细粒度响应式模型，掌握所有信号 API。

**2.1 信号创建与读写（17 题）**

| #  | 题目                 | 难度 | 核心知识点                                            |
| -- | -------------------- | ---- | ----------------------------------------------------- |
| 31 | use_signal 创建      | ⭐   | `let mut s = use_signal(\|\| val)`                    |
| 32 | .read() 引用读取     | ⭐   | `count.read()` 返回 `Ref<T>`                          |
| 33 | 函数调用语法读取     | ⭐   | `count()` 等价于 `.cloned()`                          |
| 34 | {count} Display 读取 | ⭐   | `rsx! { "{count}" }` 自动格式化                       |
| 35 | .set() 设置值        | ⭐   | `count.set(42)`                                       |
| 36 | *signal.write()      | ⭐⭐ | `*count.write() = 42`                                 |
| 37 | 运算符重载           | ⭐⭐ | `count += 1`, `count -= 1`, `count *= 2`              |
| 38 | .toggle() 布尔切换   | ⭐⭐ | `flag.toggle()`                                       |
| 39 | .iter() 集合迭代     | ⭐⭐ | `for item in items.iter()`                            |
| 40 | 多信号创建           | ⭐   | 多个 `use_signal`                                     |
| 41 | 信号类型推断         | ⭐⭐ | 泛型参数、类型推导                                     |
| 42 | ReadSignal vs Signal | ⭐⭐ | 只读/读写权限分离                                     |
| 43 | 懒初始化             | ⭐⭐ | `use_signal(\|\| expensive_init())`                    |
| 44 | 信号 Copy 语义       | ⭐⭐ | 信号是 `Copy` 的                                       |
| 45 | peek() 非响应式读取  | ⭐⭐⭐ | `count.peek()` 不建立订阅                              |
| 46 | 跨 async 边界        | ⭐⭐ | 异步任务中安全使用信号                                 |
| 47 | 自动批处理           | ⭐⭐⭐ | 多次 `.set()` 合并为一次重渲染                         |

**2.2 派生信号与 Memo（16 题）**

| #  | 题目              | 难度 | 核心知识点                                |
| -- | ----------------- | ---- | ----------------------------------------- |
| 48 | 简单 move 闭包    | ⭐   | `let double = move \|\| count() * 2`       |
| 49 | 多信号派生        | ⭐   | `let sum = move \|\| a() + b()`            |
| 50 | use_memo 基础     | ⭐⭐ | `use_memo(move \|\| count() * 2)`          |
| 51 | Memo vs 原始闭包  | ⭐⭐ | Memo 缓存、按需重算                        |
| 52 | Memo 链式         | ⭐⭐ | `a -> memo1 -> memo2`                     |
| 53 | Memo 条件派生     | ⭐⭐⭐ | 条件触发重算                              |
| 54 | 惰性派生          | ⭐⭐ | 闭包捕获不立即计算                         |
| 55 | 派生中调用函数    | ⭐⭐ | `move \|\| format!("{}", count())`         |
| 56 | 信号数组派生      | ⭐⭐⭐ | `(0..10).map(\|i\| move \|\| base() + i)` |
| 57 | 响应式 Eq 判断    | ⭐⭐⭐ | `move \|\| a() == b()`                    |
| 58 | 派生信号作 prop   | ⭐⭐ | `value={move \|\| count() * 2}`            |
| 59 | Memo read() 方法  | ⭐⭐ | `.read()` 获取引用                          |
| 60 | Memo cloned()     | ⭐   | `.cloned()` 获取克隆值                      |
| 61 | 条件传播          | ⭐⭐⭐ | 仅依赖变化时通知                           |
| 62 | 依赖追踪精确性    | ⭐⭐⭐ | 仅跟踪读取过的信号                         |
| 63 | Memo 与 Signal 互转 | ⭐⭐ | `ReadSignal` 统一接收 Memo 和 Signal        |

**2.3 Effect 与生命周期（21 题）**

| #  | 题目             | 难度 | 核心知识点                            |
| -- | ---------------- | ---- | ------------------------------------- |
| 64 | use_effect 基础  | ⭐   | `use_effect(move \|\| { ... })`        |
| 65 | Effect 响应信号  | ⭐   | 信号改变 -> Effect 重新执行           |
| 66 | Effect 依赖追踪  | ⭐⭐ | 只追踪内部读取的信号                   |
| 67 | 不追踪外部变更   | ⭐⭐ | Effect 外改变不触发                    |
| 68 | 多个 Effect      | ⭐⭐ | 互不影响                              |
| 69 | 条件分支 Effect  | ⭐⭐ | `if count() > 0 { info!(...) }`       |
| 70 | use_drop 清理    | ⭐⭐⭐ | 组件销毁时释放资源                     |
| 71 | Effect 中异步    | ⭐⭐⭐ | `spawn(async move { ... })`           |
| 72 | 避免死循环       | ⭐⭐ | 不写被自己读取的信号                   |
| 73 | Batched 更新     | ⭐⭐⭐ | await 前后自动批处理                   |
| 74 | untrack 取消追踪 | ⭐⭐⭐ | 读值不建立依赖                         |
| 75 | dioxus-logger    | ⭐⭐ | `dioxus_logger::init()` + `info!()`    |
| 76 | use_hook 原语    | ⭐⭐⭐ | 不触发更新的存储                       |
| 77 | use_signal vs use_hook | ⭐⭐⭐ | 选型判断                              |
| 78 | 组件重渲染条件   | ⭐⭐ | 信号改变触发订阅组件                   |
| 79 | 零成本响应       | ⭐⭐⭐ | 未读取的信号不触发重渲染               |
| 80 | needs_update     | ⭐⭐⭐ | 手动触发组件重渲染                     |
| 81 | use_effect 执行时机 | ⭐⭐ | 渲染后执行，可读 DOM                   |
| 82 | Effect 读取 Memo | ⭐⭐ | 建立依赖链                             |
| 83 | Effect 访问 DOM  | ⭐⭐⭐ | `document::eval()`                     |
| 84 | 条件性 Effect    | ⭐⭐ | 通过 if 控制执行                        |

**2.4 全局信号 GlobalSignal（6 题）**

| #  | 题目          | 难度 | 核心知识点                                        |
| -- | ------------- | ---- | ------------------------------------------------- |
| 85 | 创建全局信号  | ⭐⭐ | `static COUNT: GlobalSignal<i32> = Signal::global(\|\| 0);` |
| 86 | 读写          | ⭐   | `COUNT += 1`, `COUNT.set(42)`                      |
| 87 | 多组件共享    | ⭐⭐ | 任意组件直接读写                                   |
| 88 | 与局部信号混合 | ⭐⭐ | 全局 + 局部协作                                    |
| 89 | 类型约束      | ⭐⭐ | 必须 `Send + Sync`                                 |
| 90 | SSR 多实例隔离 | ⭐⭐⭐ | 多租户独立                                        |

**2.5 响应式工具箱（15 题）**

| #  | 题目        | 难度 | 核心知识点                      |
| -- | ----------- | ---- | ------------------------------- |
| 91 | use_ref 基础 | ⭐⭐ | 非响应式跨渲染保持值              |
| 92 | use_ref vs use_signal | ⭐⭐ | 不触发重渲染                    |
| 93 | 多信号同步更新 | ⭐⭐ | 同一事件处理器中更新多个信号      |
| 94 | 信号作 prop  | ⭐⭐ | 自动转为 ReadSignal              |
| 95 | ReadSignal 作为 Prop | ⭐⭐ | 组件泛型参数                    |
| 96 | Vec 添加元素  | ⭐   | `.write().push(new_item)`        |
| 97 | Vec 删除元素  | ⭐⭐ | `.write().retain(...)`           |
| 98 | HashMap      | ⭐⭐ | `use_signal(\|\| HashMap::new())` |
| 99 | split 拆分   | ⭐⭐⭐ | `ReadSignal + WriteSignal`       |
| 100| map 映射     | ⭐⭐⭐ | `signal.map(\|x\| x * 2)`         |
| 101| filter 过滤  | ⭐⭐⭐ | 选择性触发                        |
| 102| async 交互   | ⭐⭐ | `spawn` 中更新信号                 |
| 103| 闭包捕获     | ⭐⭐ | 信号是 `Copy` 的                   |
| 104| 惰性评估     | ⭐⭐⭐ | 派生信号只在被读取时计算            |
| 105| 性能思维模型 | ⭐⭐⭐ | 何时重渲染，如何最小化             |

### 第 3 章：组件与属性（50 题，e106-e155）

**目标：** 掌握 Dioxus 组件系统，属性传递，children，和组合模式。

**3.1 组件基础（15 题）**

| #   | 题目          | 难度 | 核心知识点                              |
| --- | ------------- | ---- | --------------------------------------- |
| 106 | 函数组件定义  | ⭐   | `fn Name() -> Element`                 |
| 107 | #[component]  | ⭐   | 宏元数据、文档注释                       |
| 108 | 内联属性      | ⭐   | `fn Greeting(name: String)`             |
| 109 | 多个属性      | ⭐   | `fn Card(title: String, body: String)` |
| 110 | Option 可选   | ⭐⭐ | `fn Avatar(src: Option<String>)`        |
| 111 | #[props(default)] | ⭐⭐ | 默认值                               |
| 112 | #[props(into)] | ⭐⭐ | 类型转换                               |
| 113 | 结构化 Props  | ⭐⭐⭐ | `#[derive(Props)]`                    |
| 114 | PartialEq 定制| ⭐⭐⭐ | 自定义比较                              |
| 115 | Clone 要求    | ⭐⭐ | 为什么 Props 需要 Clone                 |
| 116 | 属性展开 props| ⭐⭐ | `..props` 语法                          |
| 117 | 文档注释      | ⭐⭐ | IDE 中显示 API 文档                      |
| 118 | 组件 vs 函数  | ⭐⭐ | `<Card/>` vs `Card()`                  |
| 119 | 组件生命周期  | ⭐⭐ | 挂载 -> 重渲染 -> 卸载                 |
| 120 | 纯函数区分    | ⭐⭐ | 何时用组件，何时用渲染函数              |

**3.2 Children 与组合（15 题）**

| #   | 题目           | 难度 | 核心知识点                       |
| --- | -------------- | ---- | -------------------------------- |
| 121 | children: Element | ⭐ | 组件接收子元素                   |
| 122 | 传递子元素     | ⭐   | `<Wrapper>{ rsx!{...} }`         |
| 123 | 多层嵌套       | ⭐⭐ | 组件树传递 children               |
| 124 | Wrapper 模式   | ⭐⭐ | Card, Modal, Panel                |
| 125 | 多插槽         | ⭐⭐⭐ | 命名插槽                          |
| 126 | Children 类型  | ⭐⭐ | Element 类型                      |
| 127 | Layout 组件    | ⭐⭐⭐ | 页面布局组件                      |
| 128 | Fragment       | ⭐⭐ | `fn Fragment(children: Element)`  |
| 129 | 条件性 children | ⭐⭐ | `if show { rsx!{...} }`          |
| 130 | 迭代 children  | ⭐⭐ | `for child in children`          |
| 131 | 动态生成       | ⭐⭐ | 函数返回 Element                  |
| 132 | Context Provider | ⭐⭐ | `use_context_provider` + children |
| 133 | 高阶组件       | ⭐⭐⭐ | 组件接收配置返回 Element          |
| 134 | Render Prop    | ⭐⭐⭐ | prop 是闭包                        |
| 135 | spread override | ⭐⭐ | `Card { ..props, title: "..." }` |

**3.3 组件进阶（20 题）**

| #   | 题目         | 难度 | 核心知识点                        |
| --- | ------------ | ---- | --------------------------------- |
| 136 | 局部状态     | ⭐   | `use_signal` 在组件内              |
| 137 | 回调 prop    | ⭐⭐ | `onclick: EventHandler`            |
| 138 | EventHandler | ⭐⭐ | `on_save: EventHandler<String>`    |
| 139 | 双向绑定     | ⭐⭐ | value + onchange                   |
| 140 | 受控/非受控  | ⭐⭐ | 信号控制 vs DOM 原生               |
| 141 | 传递信号     | ⭐⭐ | `<Child count={count}/>`           |
| 142 | 传递闭包     | ⭐⭐ | `<Button on_click={move || ...}/>` |
| 143 | 泛型组件     | ⭐⭐⭐ | `fn List<T: Display>(items: Vec<T>)` |
| 144 | 可重用性设计 | ⭐⭐⭐ | 如何设计 Props                    |
| 145 | 重渲染优化   | ⭐⭐ | 避免不必要重渲染                  |
| 146 | 缓存机制     | ⭐⭐⭐ | 自动属性 memoization               |
| 147 | 组件内 context | ⭐⭐ | `use_context::<MyState>()`        |
| 148 | 多 hooks     | ⭐⭐ | signal + memo + effect             |
| 149 | 异步初始化   | ⭐⭐ | `use_resource` 在组件中            |
| 150 | 条件性挂载   | ⭐⭐ | `if show { Comp {} }`             |
| 151 | key 保留状态 | ⭐⭐⭐ | `key` 保持跨渲染状态               |
| 152 | 组合 vs 继承 | ⭐⭐ | 组合优于继承                       |
| 153 | 原子组件     | ⭐⭐ | Button, Input, Badge               |
| 154 | 复合组件     | ⭐⭐⭐ | `<Select><Option/></Select>`       |
| 155 | 样式封装     | ⭐⭐ | 内联样式                           |

### 第 4 章：表单与事件（40 题，e156-e195）

**目标：** 掌握事件处理、表单输入、验证等交互式 UI 开发。

**4.1 事件处理基础（15 题）**

| #   | 题目         | 难度 | 核心知识点                      |
| --- | ------------ | ---- | ------------------------------- |
| 156 | onclick      | ⭐   | `button { onclick: move || ... }` |
| 157 | oninput      | ⭐   | `input { oninput: move \|e\| ... }` |
| 158 | onchange     | ⭐   | 输入变更事件                    |
| 159 | onsubmit     | ⭐⭐ | 表单提交                        |
| 160 | Event 对象   | ⭐⭐ | `e.value()`, `e.checked()`      |
| 161 | onkeydown    | ⭐⭐ | `e.key()`, `e.code()`           |
| 162 | onmouseover  | ⭐⭐ | 鼠标坐标                        |
| 163 | onfocus/onblur | ⭐⭐ | 焦点事件                        |
| 164 | onscroll     | ⭐⭐ | 滚动事件                        |
| 165 | onresize     | ⭐⭐ | Dioxus IntersectionObserver      |
| 166 | onvisible    | ⭐⭐ | 进入/离开视口                    |
| 167 | 事件冒泡     | ⭐⭐⭐ | `e.stop_propagation()`           |
| 168 | prevent_default | ⭐⭐ | `e.prevent_default()`           |
| 169 | 返回错误     | ⭐⭐ | `onclick: move || -> Result<()>` |
| 170 | 自定义事件   | ⭐⭐⭐ | 带 `"on"` 前缀                   |

**4.2 表单输入（15 题）**

| #   | 题目       | 难度 | 核心知识点                  |
| --- | ---------- | ---- | --------------------------- |
| 171 | 文本绑定   | ⭐   | `value: "{name}"` + oninput |
| 172 | 数字输入   | ⭐   | `input { type: "number" }`  |
| 173 | Checkbox   | ⭐   | `checked` + `oninput`       |
| 174 | Radio      | ⭐⭐ | 一组 radio 共享状态          |
| 175 | Select     | ⭐⭐ | 下拉选择框                   |
| 176 | Textarea   | ⭐⭐ | 多行文本                     |
| 177 | 文件上传   | ⭐⭐ | `e.files()`                  |
| 178 | 必填验证   | ⭐⭐ | 检查非空                     |
| 179 | 邮箱验证   | ⭐⭐ | 格式验证                     |
| 180 | 密码强度   | ⭐⭐⭐ | 组合验证                     |
| 181 | 表单提交   | ⭐⭐ | 序列化字段                    |
| 182 | 表单重置   | ⭐⭐ | 一键清空                     |
| 183 | 加载状态   | ⭐⭐ | 禁用按钮 + spinner            |
| 184 | 多步骤表单 | ⭐⭐⭐ | 步骤状态机                    |
| 185 | 自定义控件 | ⭐⭐⭐ | FormInput 组件                |

**4.3 特殊属性和模式（10 题）**

| #   | 题目         | 难度 | 核心知识点                   |
| --- | ------------ | ---- | ---------------------------- |
| 186 | 条件属性     | ⭐⭐ | `type: if number() { ... }`  |
| 187 | 多 class     | ⭐⭐ | `class: "base", class: if active { "active" }` |
| 188 | 内联样式     | ⭐⭐ | `width: "100px"` 等           |
| 189 | 拖拽事件     | ⭐⭐⭐ | dragstart/dragover/drop       |
| 190 | 剪贴板事件   | ⭐⭐⭐ | copy/cut/paste                |
| 191 | 触摸事件     | ⭐⭐⭐ | touchstart/touchmove/touchend |
| 192 | 动画事件     | ⭐⭐ | animationend/animationstart   |
| 193 | dangerous_inner_html 高级 | ⭐⭐ | Markdown 渲染          |
| 194 | data 属性    | ⭐⭐ | `"data-*": "value"`          |
| 195 | 内联 JS      | ⭐⭐⭐ | `"onclick": "alert('hi')"`   |

### 第 5 章：异步与数据获取（40 题，e196-e235）

**目标：** 掌握 Dioxus 的异步编程模型，资源管理，Suspense。

**5.1 use_resource 与数据加载（15 题）**

| #   | 题目              | 难度 | 核心知识点                                |
| --- | ----------------- | ---- | ----------------------------------------- |
| 196 | use_resource 基础 | ⭐   | `use_resource(move \|\| async { ... })`    |
| 197 | Resource 读取     | ⭐   | `response.read()` 获取 `Result<T>`         |
| 198 | 响应式依赖        | ⭐⭐ | closure 中读信号，依赖变化重执行            |
| 199 | 加载状态          | ⭐⭐ | `match &*response.read() { Ok => ..., Err => ... }` |
| 200 | suspend() 操作符  | ⭐⭐ | `.suspend()?` 暂停组件直到数据就绪         |
| 201 | SuspenseBoundary  | ⭐⭐ | `SuspenseBoundary { fallback: \|_\| ..., ... }` |
| 202 | 多资源等待        | ⭐⭐ | 多个 use_resource 同时暂停                 |
| 203 | 嵌套 Suspense     | ⭐⭐⭐ | 父子优先级                               |
| 204 | 手动刷新          | ⭐⭐ | `resource.restart()`                      |
| 205 | 依赖链            | ⭐⭐ | 资源 A 完成 -> 资源 B 开始                |
| 206 | 错误处理          | ⭐⭐ | `resource.read().as_ref().unwrap()`        |
| 207 | 超时              | ⭐⭐⭐ | `select!` 或 timeout                       |
| 208 | 轮询              | ⭐⭐⭐ | 定时刷新资源                              |
| 209 | 取消              | ⭐⭐ | 组件卸载自动取消                          |
| 210 | 乐观 UI           | ⭐⭐⭐ | 先改 UI 再发请求，失败回滚                |

**5.2 异步基础（15 题）**

| #   | 题目          | 难度 | 核心知识点                          |
| --- | ------------- | ---- | ----------------------------------- |
| 211 | spawn         | ⭐⭐ | `spawn(async move { ... })`         |
| 212 | spawn 中更新信号 | ⭐⭐ | 异步完成后更新 UI                  |
| 213 | spawn_local   | ⭐⭐ | 线程安全 vs 局部                    |
| 214 | async 事件处理器 | ⭐⭐ | `onclick: move \|_\| async move { ... }` |
| 215 | join! 并发    | ⭐⭐⭐ | `futures::join!`                    |
| 216 | select! 竞态  | ⭐⭐⭐ | 最快响应优先                        |
| 217 | use_callback  | ⭐⭐⭐ | 避免闭包重建                        |
| 218 | 信号在 async  | ⭐⭐ | `Copy` + `Send`                     |
| 219 | 临时借用      | ⭐⭐ | 先克隆再进 async                    |
| 220 | 延迟加载      | ⭐⭐⭐ | 可见时才开始加载                    |
| 221 | gloo-net HTTP | ⭐⭐ | `gloo_net::http::Request::get()`   |
| 222 | reqwest WASM  | ⭐⭐ | `reqwest::Client` + wasm            |
| 223 | JSON 序列化   | ⭐⭐ | `serde_json`, `.json::<T>()`        |
| 224 | WebSocket     | ⭐⭐⭐ | `gloo_net::websocket`               |
| 225 | use_websocket | ⭐⭐⭐ | 全栈内置 hook                       |

**5.3 错误处理（10 题）**

| #   | 题目              | 难度 | 核心知识点                      |
| --- | ----------------- | ---- | ------------------------------- |
| 226 | RenderError       | ⭐⭐ | `Element = Result<VNode, RenderError>` |
| 227 | ? 操作符          | ⭐⭐ | 组件中使用 `?`                  |
| 228 | ErrorBoundary     | ⭐⭐ | `ErrorBoundary { handle_error: \|e\| ..., ... }` |
| 229 | 嵌套 ErrorBoundary| ⭐⭐ | 不同层级捕获不同错误            |
| 230 | 事件中抛错误      | ⭐⭐ | `onclick: move \|_\| -> Result<()>` |
| 231 | anyhow 集成       | ⭐⭐ | `.context("...")?`              |
| 232 | Downcast 错误     | ⭐⭐⭐ | `error.downcast_ref::<MyError>()` |
| 233 | 局部错误状态      | ⭐⭐⭐ | 信号中存错误                    |
| 234 | 错误恢复与重试    | ⭐⭐⭐ | ErrorBoundary + 重试按钮        |
| 235 | 全局错误策略      | ⭐⭐⭐ | 顶层 ErrorBoundary + 日志       |

### 第 6 章：路由（45 题，e236-e280）

**目标：** 掌握 Dioxus Router 的类型安全路由系统。

**6.1 路由基础（15 题）**

| #   | 题目            | 难度 | 核心知识点                            |
| --- | --------------- | ---- | ------------------------------------- |
| 236 | 安装 Router     | ⭐   | `features = ["router"]`              |
| 237 | Routable 枚举   | ⭐   | `#[derive(Routable)]`, `#[route("/")]`|
| 238 | 渲染路由        | ⭐   | `Router::<Route> {}`                  |
| 239 | Link 导航       | ⭐   | `<Link { to: Route::About }>`         |
| 240 | 路径参数 :id    | ⭐⭐ | `#[route("/user/:id")]`              |
| 241 | 查询参数 ?key   | ⭐⭐ | `#[route("/search?q")]`              |
| 242 | 通配符 *tail    | ⭐⭐ | `#[route("/files/*tail")]`           |
| 243 | 嵌套路由        | ⭐⭐ | `#[nest("/admin")]`                  |
| 244 | 布局路由        | ⭐⭐ | `#[layout(Layout)]`                  |
| 245 | 重定向 Redirect | ⭐⭐ | `Redirect { to: Route::Home }`       |
| 246 | push 导航       | ⭐⭐ | `use_navigator()().push(Route::Home)` |
| 247 | replace 替换    | ⭐⭐ | 不留下历史记录                        |
| 248 | go_back 返回    | ⭐⭐ | `use_navigator()().go_back()`         |
| 249 | 404 路由        | ⭐⭐ | `#[route("/:..route")]`              |
| 250 | active_class    | ⭐⭐ | `Link { active_class: "active", ... }`|

**6.2 路由进阶（15 题）**

| #   | 题目         | 难度 | 核心知识点                        |
| --- | ------------ | ---- | --------------------------------- |
| 251 | 路由守卫     | ⭐⭐⭐ | 检查权限重定向                    |
| 252 | 懒加载       | ⭐⭐⭐ | WASM bundle 拆分                  |
| 253 | 嵌套布局     | ⭐⭐ | `#[nest]` + `#[layout]`           |
| 254 | 多参数       | ⭐⭐ | `#[route("/:org/:repo")]`         |
| 255 | 可选参数     | ⭐⭐ | `Option<u32>`                     |
| 256 | 查询参数序列化 | ⭐⭐ | 复杂参数                          |
| 257 | SSR 路由     | ⭐⭐ | 服务端识别路由                    |
| 258 | use_route    | ⭐⭐ | 获取当前路由信息                  |
| 259 | 跨路由共享   | ⭐⭐ | Context 跨路由                     |
| 260 | 带数据导航   | ⭐⭐ | 通过信号传递数据                   |
| 261 | 过渡动画     | ⭐⭐⭐ | CSS 动画                           |
| 262 | Tab 导航     | ⭐⭐ | 二级导航                           |
| 263 | 面包屑       | ⭐⭐⭐ | 自动生成面包屑                     |
| 264 | 离开确认     | ⭐⭐⭐ | 表单未保存时确认                   |
| 265 | 无路由模式   | ⭐⭐ | 纯组件切换                         |

**6.3 路由与组合（15 题）**

| #   | 题目           | 难度 | 核心知识点                      |
| --- | -------------- | ---- | ------------------------------- |
| 266 | use_routing    | ⭐⭐⭐ | 推送并等待完成                  |
| 267 | Link 自定义    | ⭐⭐ | 带 class 的 Link                |
| 268 | target 属性    | ⭐⭐ | `_blank` 新窗口                 |
| 269 | 动态路由生成   | ⭐⭐ | 从数据生成路由                  |
| 270 | 参数校验       | ⭐⭐ | 自动解析返回 404                 |
| 271 | 路由 + use_resource | ⭐⭐ | 路由变化自动重新获取            |
| 272 | 懒加载 + Suspense | ⭐⭐⭐ | 页面级 Suspense                |
| 273 | 响应式导航     | ⭐⭐⭐ | 信号驱动导航                    |
| 274 | 外部链接       | ⭐⭐ | `Link { to: "https://..." }`    |
| 275 | 路由变化监听   | ⭐⭐ | `use_effect` + `use_route`      |
| 276 | 多级布局嵌套   | ⭐⭐⭐ | 主布局 -> 面板 -> 内容          |
| 277 | 认证集成       | ⭐⭐⭐ | 登录状态控制路由                |
| 278 | 参数 + Resource | ⭐⭐ | `let id = use_route::<Route>().id` |
| 279 | SSR 路由匹配   | ⭐⭐⭐ | 服务端 404 状态码               |
| 280 | 多页面应用架构 | ⭐⭐⭐ | 路由 + 布局 + 状态 + 数据       |

### 第 7 章：全局状态与上下文（35 题，e281-e315）

**目标：** 掌握 Dioxus 的状态共享模式，Context，GlobalSignal，Store。

**7.1 上下文（15 题）**

| #   | 题目            | 难度 | 核心知识点                              |
| --- | --------------- | ---- | --------------------------------------- |
| 281 | use_context_provider | ⭐ | `use_context_provider(\|\| MyState::new())` |
| 282 | use_context      | ⭐   | `let state = use_context::<MyState>()`  |
| 283 | 类型安全        | ⭐⭐ | TypeId 索引，wrapper 类型               |
| 284 | 穿透多层组件    | ⭐⭐ | 无需 prop drilling                      |
| 285 | 多 Context 共存  | ⭐⭐ | ThemeContext, AuthContext, etc.          |
| 286 | Provider 组件    | ⭐⭐ | `ThemeProvider { children }`            |
| 287 | 动态 provide    | ⭐⭐⭐ | 事件处理器中动态提供                    |
| 288 | 动态 consume    | ⭐⭐⭐ | 事件处理器直接消费                      |
| 289 | 作用域 Context  | ⭐⭐ | 子树不同的 Context 值                   |
| 290 | 信号 Context    | ⭐⭐ | `struct AppState { count: Signal<i32> }`|
| 291 | 方法封装        | ⭐⭐ | `impl AppState { fn increment(&mut self) }` |
| 292 | 异步方法        | ⭐⭐ | `impl AppState { async fn fetch(&mut self) }` |
| 293 | 跨路由共享      | ⭐⭐ | 根组件提供，所有页面共享                |
| 294 | 存储 Element    | ⭐⭐⭐ | 动态 UI 片段                            |
| 295 | 状态架构        | ⭐⭐⭐ | 如何组织多个 Context                    |

**7.2 GlobalSignal（10 题）**

| #   | 题目         | 难度 | 核心知识点                                      |
| --- | ------------ | ---- | ----------------------------------------------- |
| 296 | Signal::global | ⭐⭐ | `static COUNT: GlobalSignal<i32> = Signal::global(\|\| 0);` |
| 297 | 读写安全     | ⭐⭐ | 必须 `Send + Sync`                              |
| 298 | 跨组件       | ⭐⭐ | 任意组件直接 `{COUNT}`                          |
| 299 | 多信号共存   | ⭐⭐ | 多个独立全局状态                                |
| 300 | 与局部信号组合 | ⭐⭐ | 全局主题色 + 局部表单                          |
| 301 | 初始化时机   | ⭐⭐ | 懒初始化                                      |
| 302 | SSR 隔离     | ⭐⭐⭐ | 多租户独立                                    |
| 303 | map 方法     | ⭐⭐⭐ | 派生全局只读信号                              |
| 304 | filter 方法  | ⭐⭐⭐ | 条件性通知                                    |
| 305 | 全局 vs 局部 | ⭐⭐ | 选择标准                                      |

**7.3 Store 与细粒度集合（10 题）**

| #   | 题目       | 难度 | 核心知识点                              |
| --- | ---------- | ---- | --------------------------------------- |
| 306 | #[derive(Store)] | ⭐⭐⭐ | `use_store(\|\| MyStore { ... })`       |
| 307 | Lens 访问  | ⭐⭐⭐ | `store.title()` 字段级响应式引用        |
| 308 | 嵌套 Store | ⭐⭐⭐ | 多层透镜                                |
| 309 | 写入更新   | ⭐⭐⭐ | `*store.title.write() = "new"`          |
| 310 | Option/Result | ⭐⭐⭐ | `store.data().unwrap()`                 |
| 311 | HashMap    | ⭐⭐⭐ | `use_store(\|\| HashMap::new())` 逐条目  |
| 312 | Vec        | ⭐⭐⭐ | `use_store(\|\| vec![])` + `.iter()`    |
| 313 | Lens 作 Props | ⭐⭐⭐ | 自动适配                                |
| 314 | ReadStore  | ⭐⭐⭐ | 只读版本                                |
| 315 | 性能优化   | ⭐⭐⭐ | 避免全量重渲染                          |

### 第 8 章：全栈与 SSR（40 题，e316-e355）

**目标：** 掌握 Dioxus Fullstack 的 SSR、Server Functions、数据库集成。

**8.1 全栈项目设置（10 题）**

| #   | 题目          | 难度 | 核心知识点                          |
| --- | ------------- | ---- | ----------------------------------- |
| 316 | 创建全栈项目  | ⭐⭐ | `dx new --platform fullstack`       |
| 317 | Cargo.toml    | ⭐⭐ | features, axum 依赖                  |
| 318 | SSR 渲染基础  | ⭐⭐ | 服务端渲染 HTML + 水合               |
| 319 | 水合 Hydration| ⭐⭐ | 客户端恢复交互性                      |
| 320 | 热重载        | ⭐⭐ | `dx serve --platform fullstack`     |
| 321 | 发布构建      | ⭐⭐ | `dx build --release`                |
| 322 | 项目结构      | ⭐⭐ | main.rs server + lib.rs app          |
| 323 | 环境变量      | ⭐⭐ | 服务端配置、数据库连接串              |
| 324 | CORS          | ⭐⭐⭐ | 开发模式自动处理                      |
| 325 | 部署 Fly.io   | ⭐⭐⭐ | fly.toml, Dockerfile                 |

**8.2 Server Functions（15 题）**

| #   | 题目        | 难度 | 核心知识点                              |
| --- | ----------- | ---- | --------------------------------------- |
| 326 | #[get] 基础 | ⭐⭐ | `#[get("/api/hello")]`                  |
| 327 | #[post]     | ⭐⭐ | 创建资源                                |
| 328 | #[put]      | ⭐⭐ | 更新资源                                |
| 329 | #[delete]   | ⭐⭐ | 删除资源                                |
| 330 | 路径参数    | ⭐⭐ | `{id}`                                  |
| 331 | 查询参数    | ⭐⭐ | `?key`                                  |
| 332 | 客户端调用  | ⭐⭐ | `get_users().await?`                    |
| 333 | 错误处理    | ⭐⭐ | `Result<T, ServerFnError>`              |
| 334 | 数据库操作  | ⭐⭐⭐ | `sqlx::query!()`                       |
| 335 | 文件上传    | ⭐⭐⭐ | `FileStream`                            |
| 336 | 流式响应    | ⭐⭐⭐ | `Streaming<T>`                          |
| 337 | WebSocket   | ⭐⭐⭐ | `use_websocket` + 服务端                |
| 338 | 认证中间件  | ⭐⭐⭐ | session/token 检查                      |
| 339 | 自定义输入/输出 | ⭐⭐⭐ | `FromRequest` + `IntoRequest`           |
| 340 | 批量 Server Functions | ⭐⭐⭐ | 模块组织                  |

**8.3 SSR 与 Suspense（10 题）**

| #   | 题目           | 难度 | 核心知识点                          |
| --- | -------------- | ---- | ----------------------------------- |
| 341 | SSR 基本流程   | ⭐⭐ | LaunchBuilder + 渲染流程            |
| 342 | SSR 数据获取   | ⭐⭐ | `use_server_future`                 |
| 343 | use_server_future vs use_resource | ⭐⭐ | 序列化要求不同   |
| 344 | 流式 SSR       | ⭐⭐⭐ | 部分 HTML 先发送                    |
| 345 | SSR + Suspense | ⭐⭐ | 服务端等待暂停                      |
| 346 | SEO 优化       | ⭐⭐ | meta 标签、sitemap                  |
| 347 | SSG            | ⭐⭐⭐ | 静态站点生成                        |
| 348 | ISR            | ⭐⭐⭐ | 增量静态再生                        |
| 349 | 服务端 Context | ⭐⭐ | 服务端提供/消费 Context              |
| 350 | 水合错误排查   | ⭐⭐⭐ | 常见原因和解决                      |

**8.4 数据库集成（5 题）**

| #   | 题目       | 难度 | 核心知识点                          |
| --- | ---------- | ---- | ----------------------------------- |
| 351 | SQLite     | ⭐⭐⭐ | `sqlx::SqlitePool`                  |
| 352 | CRUD 操作  | ⭐⭐⭐ | 增删改查 Server Functions           |
| 353 | 迁移       | ⭐⭐⭐ | `sqlx migrate run`                  |
| 354 | PostgreSQL | ⭐⭐⭐ | 切换数据库                          |
| 355 | 连接池管理 | ⭐⭐⭐ | 共享状态中的连接池                  |

### 第 9 章：高级主题（40 题，e356-e395）

**目标：** 掌握自定义 Hooks、生命周期、渲染器、平台 API。

**9.1 自定义 Hooks（12 题）**

| #   | 题目         | 难度 | 核心知识点                          |
| --- | ------------ | ---- | ----------------------------------- |
| 356 | 基本模式     | ⭐⭐ | `fn use_counter() -> (Signal<i32>, impl Fn())` |
| 357 | Hook 组合    | ⭐⭐ | 多个内置 Hook 组合                  |
| 358 | use_hook 原语 | ⭐⭐⭐ | 手写自定义状态存储                  |
| 359 | use_drop 清理 | ⭐⭐ | 组件卸载时释放资源                  |
| 360 | 带参数 Hook  | ⭐⭐ | `use_local_storage(key, default)`   |
| 361 | 异步 Hook    | ⭐⭐⭐ | 管理加载状态 + 数据                 |
| 362 | 使用 Context | ⭐⭐ | `use_settings()` 封装               |
| 363 | needs_update | ⭐⭐⭐ | 手动触发重渲染                      |
| 364 | Hook 测试    | ⭐⭐⭐ | 验证 Hook 行为                      |
| 365 | 发布到 crates.io | ⭐⭐⭐ | 公开 API 设计                     |
| 366 | use_persistent | ⭐⭐⭐ | localStorage 持久化                |
| 367 | 命名规范     | ⭐⭐ | `use_` 开头，Rules of Hooks         |

**9.2 组件生命周期（8 题）**

| #   | 题目       | 难度 | 核心知识点                          |
| --- | ---------- | ---- | ----------------------------------- |
| 368 | 挂载与卸载 | ⭐⭐ | `use_hook` 初始化, `use_drop` 清理  |
| 369 | 重渲染条件 | ⭐⭐ | 信号、Props、父组件                  |
| 370 | PartialEq  | ⭐⭐ | 自动比较决定重渲染                  |
| 371 | 不要修改状态 | ⭐⭐ | 为什么不在组件体中 `.set()`         |
| 372 | use_memo 替代 | ⭐⭐ | 避免不必要的派生计算                |
| 373 | 条件渲染优化 | ⭐⭐ | 不渲染时不运行代码                  |
| 374 | VirtualNode | ⭐⭐⭐ | Dioxus 虚拟 DOM                     |
| 375 | 清理订阅    | ⭐⭐ | Effect 中使用 `use_drop`            |

**9.3 突破 Dioxus 限制（10 题）**

| #   | 题目        | 难度 | 核心知识点                          |
| --- | ----------- | ---- | ----------------------------------- |
| 376 | web-sys DOM | ⭐⭐⭐ | `document::*` API                   |
| 377 | 浏览器 API  | ⭐⭐⭐ | 通知、剪贴板                        |
| 378 | JS 插值     | ⭐⭐⭐ | `"onclick": "js code"`              |
| 379 | 自定义属性  | ⭐⭐⭐ | 非标准 HT

---

## 综合项目（120 步）

### 项目 A：HotDog —— Tinder for Dogs（60 步）

参照 Dioxus 官方教程项目，扩展为 60 步渐进式项目，细分为 8 个阶段：

| 阶段    | 步数 | 内容                                     |
| ------- | ---- | ---------------------------------------- |
| Phase 0 | 5    | 项目初始化、rsx! UI、组件拆分、样式、静态数据 |
| Phase 1 | 8    | 信号状态、喜欢/不喜欢、计数器、图片轮换、手势 |
| Phase 2 | 8    | 异步加载狗图 API、Loading、错误处理、重试    |
| Phase 3 | 8    | 路由：首页/收藏/详情/设置、导航传参           |
| Phase 4 | 8    | 全局状态：收藏跨页面共享、批量操作             |
| Phase 5 | 8    | 全栈：Server Function、SQLite、用户偏好       |
| Phase 6 | 8    | 高级：拖拽排序、CSS 动画、Suspense、性能优化  |
| Phase 7 | 7    | 测试：单元测试、组件测试、E2E 测试             |

**60 步详细分解：**

| 步骤 | 名称           | 核心知识点                                |
| :--: | -------------- | ---------------------------------------- |
| A-01 | 项目脚手架     | `dx new hotdog`                          |
| A-02 | 基本 UI 骨架   | rsx! 布局                                |
| A-03 | Button 组件    | #[component] + props                     |
| A-04 | 样式系统       | 内联 style、class                        |
| A-05 | 静态数据列表   | for 循环渲染                             |
| A-06 | 点赞信号       | use_signal + onclick                     |
| A-07 | 图片索引       | use_signal 管理索引                      |
| A-08 | 动画过渡       | CSS transition                           |
| A-09 | 滑动手势       | ondrag 事件                              |
| A-10 | 计数统计       | use_memo 派生计数                        |
| A-11 | 撤消操作       | 历史栈信号 Vec                           |
| A-12 | 匹配弹窗       | 条件渲染                                 |
| A-13 | 键盘快捷键     | onkeydown                                |
| A-14 | 引入 API       | use_resource + reqwest                   |
| A-15 | Loading 状态   | match resource.read()                    |
| A-16 | 错误处理       | ErrorBoundary + 重试                     |
| A-17 | SuspenseBoundary | fallback                                |
| A-18 | 超时与重试     | select!                                  |
| A-19 | 预加载         | 提前请求                                 |
| A-20 | 缓存           | HashMap 缓存                             |
| A-21 | 安装 Router    | features = ["router"]                    |
| A-22 | 路由定义       | #[derive(Routable)]                      |
| A-23 | 导航栏         | Link + active_class                      |
| A-24 | 详情页         | #[route("/dog/:id")]                     |
| A-25 | 路由参数+资源  | 参数驱动数据加载                         |
| A-26 | 404 页面       | 通配符路由                               |
| A-27 | 编程式导航     | navigator().push()                       |
| A-28 | 过渡动画       | 页面切换动画                             |
| A-29 | 收藏列表信号   | use_signal(Vec)                          |
| A-30 | 收藏/取消       | 切换状态                                 |
| A-31 | 收藏页面       | 路由渲染收藏列表                         |
| A-32 | 批量删除       | 多选 + 批量操作                          |
| A-33 | GlobalSignal   | Signal::global() 跨页面                  |
| A-34 | 持久化         | localStorage                             |
| A-35 | 收藏统计       | use_memo                                 |
| A-36 | 全栈项目结构   | dx new --platform fullstack              |
| A-37 | GET 端点       | #[get("/api/dogs")]                      |
| A-38 | POST 收藏      | #[post("/api/favorites")]                |
| A-39 | 客户端调用     | 前端调用 server function                 |
| A-40 | SQLite 集成    | sqlx::SqlitePool                         |
| A-41 | 迁移管理       | sqlx migrate run                         |
| A-42 | CRUD 操作      | 增删改查                                 |
| A-43 | 用户偏好       | #[get("/api/preferences")]               |
| A-44 | Session 管理   | tower-sessions                           |
| A-45 | 拖拽排序       | ondragstart/ondrop                       |
| A-46 | 列表动画       | CSS keyframes                            |
| A-47 | 图片懒加载     | onvisible                                |
| A-48 | 虚拟滚动       | 只渲染可见区域                           |
| A-49 | Memo 优化      | 隔离高频信号                             |
| A-50 | 代码分割       | WASM 按路由分块                          |
| A-51 | 日志初始化     | dioxus_logger::init()                    |
| A-52 | DevTools       | 组件树 + 信号追踪                        |
| A-53 | 单元测试       | 测试纯函数                               |
| A-54 | 组件测试       | Button/Card 渲染测试                     |
| A-55 | 事件测试       | 模拟点击喜欢                             |
| A-56 | E2E 测试       | Playwright                               |
| A-57 | 生产构建       | dx build --release                       |
| A-58 | Docker 部署    | Dockerfile                               |
| A-59 | SSR 开启       | 全栈 SSR + SEO                           |
| A-60 | 总结回顾       | 项目架构复盘                             |

### 项目 B：TodoDashboard（60 步）

与 HotDog 平行的第二个综合项目，偏向企业级管理面板。细分为 8 个阶段：

| 阶段    | 步数 | 内容                                     |
| ------- | ---- | ---------------------------------------- |
| Phase 0 | 5    | 项目初始化、布局、侧边栏、顶栏、主题切换   |
| Phase 1 | 8    | 表单系统：创建、编辑、删除、搜索、筛选、分页 |
| Phase 2 | 8    | 数据可视化：Chart、统计卡片、进度条、仪表盘 |
| Phase 3 | 8    | 路由：仪表盘/任务/用户/设置、嵌套布局       |
| Phase 4 | 8    | 全局状态：认证、通知、主题、缓存             |
| Phase 5 | 8    | 全栈：REST API、CRUD、文件导出、WebSocket    |
| Phase 6 | 8    | 高级：拖拽看板、富文本、i18n、A11y           |
| Phase 7 | 7    | 测试与部署：组件测试、E2E、CI/CD             |

**60 步详细分解：**

| 步骤 | 名称         | 核心知识点                                |
| :--: | ------------ | ---------------------------------------- |
| B-01 | 项目初始化   | `dx new dashboard`                       |
| B-02 | 三栏布局     | Header + Sidebar + Main                  |
| B-03 | 侧边栏导航   | Link + active_class                      |
| B-04 | 顶栏组件     | 搜索框 + 头像 + 通知                     |
| B-05 | 主题切换     | Context + GlobalSignal 亮/暗             |
| B-06 | 任务创建表单 | 受控组件 + 验证                          |
| B-07 | 任务列表     | for + key                                |
| B-08 | 内联编辑     | 信号切换编辑模式                         |
| B-09 | 删除确认     | 对话框                                   |
| B-10 | 搜索过滤     | use_memo 派生过滤列表                    |
| B-11 | 状态筛选     | 多条件组合筛选                           |
| B-12 | 分页         | 分页状态 + 切片                          |
| B-13 | 批量操作     | 多选 + 批量标记                         |
| B-14 | 柱状图       | SVG rsx! 绘制                            |
| B-15 | 统计卡片     | 派生信号计算                             |
| B-16 | 进度条       | 动态样式                                 |
| B-17 | 仪表盘布局   | CSS Grid                                 |
| B-18 | 折线图       | SVG polyline                             |
| B-19 | 饼图         | SVG circle stroke-dasharray              |
| B-20 | 数据联动     | 图例切换数据源                           |
| B-21 | 路由结构     | #[derive(Routable)] 多层嵌套             |
| B-22 | 嵌套布局     | #[layout(DashboardLayout)]               |
| B-23 | 面包屑       | 自动生成                                 |
| B-24 | 用户管理     | 列表 + 详情                              |
| B-25 | 设置页面     | 多 Tab 表单                              |
| B-26 | 路由守卫     | 未登录重定向                             |
| B-27 | 错误页面     | 403/404/500                              |
| B-28 | 登录页面     | 表单 + 验证                              |
| B-29 | 全局用户     | use_context_provider                     |
| B-30 | 通知中心     | 已读/未读                                |
| B-31 | 主题管理     | GlobalSignal + CSS 变量                  |
| B-32 | 数据缓存     | use_memo + localStorage                  |
| B-33 | 全局 Loading | SuspenseBoundary                         |
| B-34 | 撤销/重做    | 命令模式 + Vec 历史栈                    |
| B-35 | API 封装     | Server Functions 统一封装                |
| B-36 | 全栈初始化   | dx new --platform fullstack              |
| B-37 | CRUD API     | #[get/post/put/delete]                   |
| B-38 | 数据库       | SQLite + sqlx                            |
| B-39 | 认证 API     | 登录/注册/登出                           |
| B-40 | Session      | tower-sessions                           |
| B-41 | CSV 导出     | Streaming<String>                        |
| B-42 | WebSocket    | use_websocket + 推送                     |
| B-43 | 实时仪表盘   | WebSocket 推送统计                       |
| B-44 | 拖拽看板     | 三列 Todo/Doing/Done                    |
| B-45 | 卡片拖拽     | ondragstart/ondrop                       |
| B-46 | 状态同步     | 拖拽自动更新状态                         |
| B-47 | 富文本      
