# 练习 01: 环境准备与项目创建

## 为什么要学这个

第一个练习不做花哨的事，但这一章要回答三个问题，它们决定了你对 Tauri 的基本理解正确与否：

1. **一个桌面应用框架的入口点意味着什么？** — 从 Rust 的 `main()` 到窗口出现，中间发生了什么？
2. **"命令"到底是一个什么东西？** — 为什么它不是一个普通的 Rust 函数？前端怎么能调用到它？
3. **前端怎么调后端？** — `invoke()` 的本质是什么？为什么调用必须写成异步的？

如果把这三件事理解透了，后面所有练习（命令参数、事件、状态、窗口）就只是这三件事的自然延伸。

---

## 从问题出发

练习 01 要做的事：**检查本机的开发环境，把"前置条件是否满足"以清单形式展示在窗口里**。

在命令行里，人工检查环境要做很多件事：`rustc --version` 看 Rust、`node --version` 看 Node.js、查 WebView2 是否内置、试 `cargo tauri --version` 看 CLI…… 但在桌面应用里，事情变得复杂了——因为**网页前端做不到这些**。

**核心矛盾：** 前端跑在 WebView 里，本质上是浏览器环境。浏览器里的 JavaScript 运行在沙箱中，**没有执行系统进程的能力**——它不能启动 `rustc`，不能读取它的输出。而"检查环境"恰恰必须由操作系统进程来执行。

所以架构必然是分工的：

```
前端 (WebView)                   Rust 核心进程               检查对象
┌──────────────────┐    IPC    ┌──────────────────┐    ┌──────────────┐
│ 发请求            │ ────────► │ 检查项清单        │ ──► │ rustc / node │
│ invoke("...")    │ ◄──────── │ #[tauri::command] │ ◄── │ WebView2/CLI │
│ 拿到结果并展示     │           │ 返回结果          │     │              │
└──────────────────┘           └──────────────────┘     └──────────────┘
```

Tauri 要解决的问题很简单：**让"前端请求"和"后端执行"这两件事以最少的仪式连接起来。** 它把这个分工抽象成三层：

```
main.rs / run()         → 入口：装配并启动应用
#[tauri::command]       → 命令：定义可被前端调用的单元
invoke()                → 调用：前端跨进程发起请求
```

这三层就是本节的三个知识点。下面逐一展开。

---

## 1. `main.rs` 与 `run()` — 为什么 Tauri 应用不是普通 Rust 程序

### 最直接的问题

打开练习项目，`main.rs` 只有 6 行：

```rust
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    e01_hello_world_lib::run()
}
```

一个"Hello World"级别的 Rust 程序，`main` 里居然没有任何业务逻辑，只调用了另一个 crate 里的 `run()`。为什么？

### Tauri 应用是双进程的

普通 Rust 程序是一个进程，`main` 是它唯一的入口。但 Tauri 应用由**两个部分**组成：

```
┌──────────────────────────────────────────────┐
│ Rust 核心进程                                │
│  ├── main.rs → run()    ← 程序入口           │
│  ├── 窗口管理（创建原生窗口）                 │
│  ├── 命令注册表（#[tauri::command]）          │
│  └── 事件循环（等待前端请求、窗口事件）        │
│                                              │
│  ┌────────────────────────────────────────┐  │
│  │ WebView（内嵌的浏览器）                  │  │
│  │  ├── index.html                       │  │
│  │  └── src/main.ts（前端逻辑）             │  │
│  └────────────────────────────────────────┘  │
└──────────────────────────────────────────────┘
```

窗口由 Rust 进程创建，窗口内部嵌入了一个 WebView 来渲染前端页面。前端负责界面，Rust 负责逻辑——两者通过 IPC（进程间通信）对话。

### `main` 函数执行完就结束了吗？

通常的 Rust 程序，`main` 结束进程就退出了。但 Tauri 应用不是——`run()` 内部启动了一个**事件循环**，它在等待用户操作、窗口事件、前端请求等。窗口关闭、事件循环结束，应用才退出。

> **关键理解：** Tauri 应用不是"一次性计算"，而是"驻留程序"。`main` 只是装配步骤，真正的生命周期从 `run()` 开始。

### 为什么逻辑放在 `lib.rs` 而不是 `main.rs`？

`main.rs` 只做一件事：调用 `e01_hello_world_lib::run()`。真正的入口逻辑在 `lib.rs`：

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // TODO: 在此注册命令
            // check_environment,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
```

把逻辑放在库 crate（`lib.rs`）而不是二进制 crate（`main.rs`），是为了**移动端**：iOS/Android 的入口不是 `main()`，而是平台自己的入口函数。`#[cfg_attr(mobile, tauri::mobile_entry_point)]` 让同一个 `run()` 在移动端被替换成平台入口。这也解释了 `Cargo.toml` 里的这行：

```toml
[lib]
name = "e01_hello_world_lib"
crate-type = ["staticlib", "cdylib", "rlib"]
```

库必须能编译成多种 crate 类型，才能在桌面（rlib）和移动端（staticlib/cdylib 供原生绑定）都能被链接。

### `Builder` 装配链路

`run()` 里的三行代码，每一行都对应一个职责：

| 代码 | 职责 |
|---|---|
| `tauri::Builder::default()` | 创建一个空的构建器，准备装配应用 |
| `.invoke_handler(...)` | 注册命令（下一节详讲） |
| `.run(tauri::generate_context!())` | 启动应用 |

其中 `generate_context!()` 是**编译期宏**：它在编译时读取 `tauri.conf.json`（窗口标题、尺寸、标识符、构建配置等），生成应用运行所需的上下文数据。所以改窗口配置不用改代码，改完配置文件重新编译即可。

`.run()` 返回 `Result`，`.expect("启动 Tauri 应用失败")` 表示：启动失败就 panic 并输出错误信息——对桌面应用来说，启动失败是致命错误，没有恢复的必要。

---

## 2. `#[tauri::command]` — 把"环境检查"变成可调用的命令

### 命令与普通函数的区别

写环境检查逻辑，用普通 Rust 函数很简单：

```rust
fn check_environment() -> Vec<EnvCheck> {
    // ...
}
```

但问题是：**这个函数怎么被前端调用？** 前端和它隔着一个进程边界，不能直接调用。前端发出的是一条"消息"（一个字符串：命令名字），后端收到消息后去查表，找到对应的函数来执行。

所以"命令"和普通函数的区别在于：

| | 普通函数 | Tauri 命令 |
|---|---|---|
| 定义位置 | Rust 代码内部 | Rust 后端 |
| 调用者 | 同进程的 Rust 代码 | 前端 JS（跨进程） |
| 调用方式 | 直接调用 | 按名字符串 `invoke` |
| 参数/返回值 | 任意 Rust 类型 | 必须可序列化 |
| 由谁管理 | 调用者 | Tauri 运行时（注册表） |

**核心区别：** 你直接调用 `check_environment()`，立即得到结果。你通过 `invoke("check_environment")` 调用，是向 Tauri 运行时发出一条消息，由它找到并执行对应的函数，再把结果送回。

### 为什么用 `#[tauri::command]` 宏？

Rust 语言本身没有"命令"这个概念。`#[tauri::command]` 是一个**属性宏**，它在编译期把普通函数"升级"成框架可以识别和调用的单元：

```rust
// 你写的
#[tauri::command]
fn check_environment() -> Vec<EnvCheck> { ... }

// 宏大致生成（简化）
// 1. 参数反序列化代码：把前端传来的 JSON 转成函数参数
// 2. 返回值序列化代码：把 Vec<EnvCheck> 转成可回传前端的 JSON
// 3. 错误转换代码：函数返回 Result 时的错误处理
// 4. 注册信息：命令的名字、签名，供运行时查找
```

> **为什么用宏而不是手写？** 因为"参数反序列化、返回值序列化、错误转换"这套样板代码跟函数签名强相关——宏可以分析函数签名自动生成，手写则每个命令都要重复一遍，而且容易出错。

### 返回值：第一次出现结构体

练习的返回值不是单个字符串，而是一个**结构体列表**：

```rust
// 环境检查项：名称 + 是否就绪 + 说明
#[derive(serde::Serialize)]
struct EnvCheck {
    name: String,
    ok: bool,
    detail: String,
}

/// 返回开发前置条件检查清单。
/// 真实项目可在此读取 rustc / node 版本做动态判断，
/// 本练习以教学为目的直接给出结论。
#[tauri::command]
fn check_environment() -> Vec<EnvCheck> {
    vec![
        EnvCheck {
            name: "Rust 工具链".into(),
            ok: true,
            detail: "cargo 1.8x+ / rustc stable".into(),
        },
        EnvCheck {
            name: "Node.js 与 pnpm".into(),
            ok: true,
            detail: "Node 18+ / pnpm 9+".into(),
        },
        EnvCheck {
            name: "WebView2 Runtime".into(),
            ok: true,
            detail: "Windows 11 自带，Windows 10 需安装".into(),
        },
        EnvCheck {
            name: "Tauri CLI".into(),
            ok: true,
            detail: "cargo tauri 2.x（或 pnpm dlx tauri）".into(),
        },
        EnvCheck {
            name: "Rust 目标链".into(),
            ok: true,
            detail: "x86_64-pc-windows-msvc".into(),
        },
    ]
}
```

这里有两个新东西：

**1. `#[derive(serde::Serialize)]` — 声明"我能被序列化"。**

命令的返回值要穿越进程边界，而进程间只能传字节，所以必须序列化。`String`、`bool`、`i32` 这些基础类型自带序列化能力；自定义结构体则要自己声明"怎么序列化"——`#[derive(Serialize)]` 让编译器自动生成序列化代码。

**2. 序列化时，结构体的字段名就是 JSON 的 key：**

```
Rust: EnvCheck { name, ok, detail }
          │ serde 序列化（字段名 → JSON key）
          ▼
IPC:  {"name":"Rust 工具链","ok":true,"detail":"cargo 1.8x+ / rustc stable"}
          │ 反序列化（JSON key → TS 属性）
          ▼
TS:   { name: "...", ok: true, detail: "..." }
```

> **关键理解：** 为什么返回结构体而不是拼一个字符串？因为数据有了结构，前端才能分别使用——`ok` 决定"✓ 还是 !"，`detail` 决定说明文字。后续练习里命令带参数、事件带载荷，都是同一套序列化规则，这里先打好地基。

### 注册：`generate_handler!` 与 `invoke_handler`

光有宏还不够——命令必须**注册**到运行时，前端才调得到：

```rust
.invoke_handler(tauri::generate_handler![
    check_environment,
])
```

`generate_handler![...]` 把列出的命令打包成一张"命令注册表"，`invoke_handler` 把这张表挂到 Builder 上。前端发来的每条调用请求，都在这张表里按名字查找。

> **练习版的坑：** 练习版 `lib.rs` 里这行是注释掉的——`// check_environment,`。如果你只填了函数体而忘了取消注释注册，前端调用会直接报错"命令未找到"。这是一个故意埋下的练习点。

---

## 3. `invoke()` — 前端如何调用后端？

### 为什么必须异步？

前端的调用长这样：

```typescript
const checks = await invoke<EnvCheck[]>("check_environment");
```

注意这个 `await`——`invoke` 返回的是一个 **Promise**，调用是异步的。为什么？

因为 IPC 的本质是**进程间消息传递**，跟网络请求是同一个模型：消息要序列化、跨进程传输、后端执行、结果再传回来。如果同步等待，前端所在的线程（WebView 的 JS 是单线程事件循环）会被阻塞——整个界面卡死，用户点任何按钮都没反应。

> **关键理解：** 前端的 `invoke` 和后端的命令执行不是"同一块内存里的函数调用"，而是"两端各自独立运行的协作"。所以前端必须用 `async/await` 等待结果，就像 `fetch` 等待 HTTP 响应一样。

### 类型对应

```typescript
const checks = await invoke<EnvCheck[]>("check_environment");
//                            ^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^
//                            TS 泛型       命令名（字符串）
```

`invoke` 的第一个参数是**命令名的字符串**——这正是后端注册表按名字查找的依据。第二个泛型参数 `<EnvCheck[]>` 声明返回值类型，对应后端的 `-> Vec<EnvCheck>`。

前端与后端的类型映射：

```
Rust:  String        JS/TS: string
Rust:  i32, f64      JS/TS: number
Rust:  bool          JS/TS: boolean
Rust:  Vec<T>        JS/TS: T[]
Rust:  结构体         JS/TS: 对象
```

两边都要可序列化，类型才能对上。这个映射关系在后面的练习里会反复出现。

### TS 接口：把 JSON 形状"翻译"成类型

```typescript
import { invoke } from "@tauri-apps/api/core";

// 与后端 EnvCheck 结构体对应的 TS 接口
interface EnvCheck {
  name: string;
  ok: boolean;
  detail: string;
}
```

`interface EnvCheck` 与 Rust 结构体**字段一一对应**——因为 JSON key 就是 Rust 字段名，TS 接口只是把 JSON 形状"翻译"成类型。`invoke<EnvCheck[]>` 的泛型只是 TS 侧的**类型声明**，运行时不会校验；字段名写错，显示的就是 `undefined`，这是前后端联调最常见的错位来源。

### 答案版 `main.ts` 解读

```typescript
async function render() {
  // 调用后端命令，泛型指定返回类型
  const checks = await invoke<EnvCheck[]>("check_environment");

  listEl!.innerHTML = checks
    .map(
      (c) =>
        `<li class="${c.ok ? "ok" : "warn"}">
          <span class="badge">${c.ok ? "✓" : "!"}</span>
          <strong>${c.name}</strong>
          <span class="detail">${c.detail}</span>
        </li>`
    )
    .join("");

  const ready = checks.every((c) => c.ok);
  statusEl!.textContent = ready ? "环境就绪，可以开始练习 🎉" : "存在未满足项，请先处理";
}
```

- `listEl` / `statusEl` — 分别是 `index.html` 里的 `<ul id="checklist">` 和 `<p id="status">`
- `.map(...)` — 把每个 `EnvCheck` 渲染成一行 `<li>`：`ok` 决定样式类和徽标（✓ / !）
- `checks.every((c) => c.ok)` — 全部 `ok` 才显示"环境就绪"，否则提示先处理
- `render().catch(...)` — 命令失败时兜底展示错误（比如命令没注册、后端 panic）

> **练习流程：** 后端要做两件事（给结构体/函数加标注 + 补检查项 + 取消注册注释），前端要做三件事（取消注释 invoke、完善渲染模板、完成就绪判断）。全部 TODO 完成，运行 `cargo tauri dev`，窗口里就会出现 5 项环境检查清单。

### 我是 Web 开发者，有什么要注意的？

| | `fetch` (Web) | `invoke` (Tauri) |
|---|---|---|
| 目标 | 远程 HTTP 服务器 | 本地 Rust 进程 |
| 传输 | 网络请求 | 本地进程间 IPC |
| 协议 | HTTP + JSON | 序列化消息（JSON） |
| 权限 | CORS | 命令注册 + capabilities 配置 |
| 延迟 | 高（网络往返） | 低（本地） |
| 能做的事 | 服务器能提供的 | 系统级能力（进程、文件、窗口） |

如果你写过 `await fetch(...)`，那 `await invoke(...)` 的认知模型几乎一模一样——唯一的区别是另一端从"服务器"换成了"你身边的 Rust 进程"。这条迁移捷径会贯穿整本书。

---

## 三个知识点连起来看

```
┌──────────────────────────────────────────────┐
│ 前端 (WebView)                               │
│                                              │
│  const checks = await invoke<EnvCheck[]>(    │
│      "check_environment"                     │ ← 调用层：跨进程请求
│  );                                          │
│                                              │
│ Rust 核心进程                                │
│                                              │
│  main.rs → lib.rs::run()    ← 入口层：装配启动 │
│                                              │
│  #[tauri::command]          ← 命令层：可调用单元│
│  fn check_environment() -> Vec<EnvCheck> {...} │
│                                              │
└──────────────────────────────────────────────┘
```

三个知识点是层层递进的：

| 层 | 解决的问题 | 关键概念 |
|----|-----------|---------|
| `main.rs` / `run()` | 窗口从哪来、程序何时退出 | 入口点、双进程、事件循环 |
| `#[tauri::command]` | Rust 函数如何被前端调用 | 属性宏、注册表、序列化 |
| `invoke()` | 前端如何发起调用 | IPC、Promise、async/await |

**一通百通的核心：** 这三个知识点反映的是 Tauri（以及大多数桌面应用框架）的三个基础层——**入口层、命令层、调用层**。后面要学的命令参数、事件、状态管理、窗口控制，都是在这三层的基础上叠加的：入口层决定"从哪里开始"，命令层决定"后端能力怎么暴露"，调用层决定"前端怎么使用"。

**递进关系：** 练习 02 将解剖项目结构，让你看清这三层在磁盘上各自的物理位置；练习 03 将回答"开发时窗口里的页面从哪来"——`devUrl` 与 `frontendDist`。