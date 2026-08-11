# 练习 02: 项目结构

## 为什么要学这个

练习 01 你在现成的项目里改代码——入口、配置、命令注册早就替你搭好了。但"每个文件为什么存在"还没人解释。这一章要回答三个问题：

1. **一个项目为什么有两个 `src`？** — `src/` 和 `src-tauri/` 各自装着什么？为什么必须分开？
2. **`src-tauri/` 里每个文件是干什么的？** — 哪些是你手写的、哪些是自动生成的、哪些不能碰？
3. **命令的返回值为什么能是一个字符串数组？** — `Vec<String>` 穿越进程边界，前端拿到的又是什么？

目录结构是双进程架构的"物理投影"。看懂它，你就不会再在项目里迷路。

---

## 从问题出发

练习 02 要做的事：**写一个命令，返回项目的目录结构说明，以"结构树"的形式显示在窗口里**。

**核心矛盾：** 一个 Tauri 项目里混着两个"世界"——前端世界（HTML/TS/Vite）和 Rust 世界（cargo crate）。它们语言不同、构建工具不同、目录不同，却要合体成一个应用。**项目结构就是这两个世界的地图：**

```
┌─────────────────────────────────────────────────────┐
│ 项目根目录                                           │
│                                                     │
│  src/                前端世界（Vite 的输入）         │
│  ├── index.html      HTML 入口（页面骨架）           │
│  └── src/            main.ts（逻辑）+ styles.css    │
│  package.json        前端依赖与脚本                  │
│  vite.config.ts      Vite 开发服务器配置             │
│  tsconfig.json       TypeScript 编译选项             │
│                                                     │
│  src-tauri/          Rust 世界（cargo crate）        │
│  ├── src/main.rs     平台入口（只有一行）            │
│  ├── src/lib.rs      命令、状态、Builder 配置        │
│  ├── Cargo.toml      Rust 依赖清单                  │
│  ├── tauri.conf.json 应用配置                       │
│  ├── capabilities/   权限声明（Tauri v2）            │
│  ├── icons/          应用图标                       │
│  └── build.rs        构建脚本                        │
└─────────────────────────────────────────────────────┘
```

练习版 `lib.rs` 的注释里就藏着这张地图——你的任务是把 12 行说明补进 `project_layout` 函数。但填完之前，先理解每一行为什么存在。

---

## 1. 双目录结构 — 前端世界与 Rust 世界

### 为什么叫 `src-tauri`

两个世界都需要一个"源码目录"，如果都叫 `src` 就会冲突。Tauri 的约定是：前端用 `src/`，Rust 用 `src-tauri/`。这个约定是 `tauri.conf.json` 里所有路径的基准——比如 `frontendDist: "../dist"` 就是以 `src-tauri/`（配置文件所在目录）为起点的相对路径。

### 前端世界：Vite 的输入

`index.html` 是整个前端唯一真正的 HTML 文件，`src/main.ts` 是逻辑入口，`src/styles.css` 是样式。前端世界的三个配置文件，角色各不相同：

| 文件 | 管什么 | 关键内容 |
|---|---|---|
| `package.json` | 依赖 + 脚本 | `dev`/`build`/`preview` 三个命令 |
| `vite.config.ts` | Vite 开发服务器 | 端口、HMR、监视规则 |
| `tsconfig.json` | TypeScript 编译器 | `strict: true`、`noUnusedLocals: true` 等 |

注意 `tsconfig.json` 里的 `"include": ["src"]`——TypeScript 只编译 `src/` 目录，`src-tauri/` 里的 Rust 代码它根本看不见。两个世界互不干扰。

### Rust 世界：一个完整的 cargo crate

`src-tauri/` 不是"半成品"，它本身就是一个可以独立编译的 cargo crate：有自己的 `Cargo.toml`、自己的 `src/`。它和前端唯一的"连接点"是 `tauri.conf.json` 和编译期宏 `generate_context!`（练习 01）。

> **关键理解：** 把前端和 Rust 想象成两个独立项目，只在"窗口加载页面"这个点上汇合（dev 是 devUrl，build 是 frontendDist）。这个心智模型能解释后面练习里几乎所有的"为什么"。

---

## 2. src-tauri/ 内部解剖

### main.rs — 永远只有一行调用

```rust
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    e02_project_structure_lib::run()
}
```

练习 01 讲过：入口只做装配，真正的工作在 `lib.rs`。注意最后一行调用的 `e02_project_structure_lib`——它不是魔法，而是 Cargo.toml 里 `[lib]` 段声明的名字：

```toml
[package]
name = "e02_project_structure"
version = "0.1.0"
edition = "2021"

[lib]
name = "e02_project_structure_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build.workspace = true

[dependencies]
tauri.workspace = true
serde.workspace = true
serde_json.workspace = true
```

- `[lib] name` — 库 crate 的名字。改了它，`main.rs` 里的调用也要跟着改
- `crate-type` — 练习 01 讲过：为了移动端，库要能编成多种 crate 类型
- `.workspace = true` — 整个 `tauri-learn` 是一个 **Cargo workspace**，所有练习的依赖版本由根目录 `Cargo.toml` 统一管理。练习 crate 不写死版本，只声明"用 workspace 里定义的版本"，这样几十个练习不会出现依赖版本打架

### lib.rs — 你的主要工作区

```rust
/// 返回本项目结构说明（每行一个节点，前端按行渲染）
#[tauri::command]
fn project_layout() -> Vec<String> {
    vec![
        "src/                  # 前端（Vite + TS + HTML）".into(),
        "  index.html          # 页面骨架，浏览器加载入口".into(),
        "  src/main.ts         # 前端逻辑：UI 与 invoke 调用".into(),
        "  src/styles.css      # 全局样式".into(),
        "src-tauri/            # 后端（独立 Rust crate）".into(),
        "  src/main.rs         # 平台入口：只有一行，调用 lib 的 run()".into(),
        "  src/lib.rs          # 核心：命令、状态、Builder 配置都在这".into(),
        "  tauri.conf.json     # 应用配置：identifier / 窗口 / 构建命令".into(),
        "  capabilities/       # 权限声明（core:default 最小权限）".into(),
        "  icons/              # 应用图标（icon.ico / icon.png）".into(),
        "  Cargo.toml          # Rust 依赖（tauri 等）".into(),
        "  build.rs            # 构建脚本，调用 tauri_build::build()".into(),
    ]
}
```

注意这里的几个细节：

- **`.into()`** — 把 `&str` 字面量转换成 `String`（从 `&str` 到 `String` 的 `From` 转换）
- **`Vec<String>`** — 返回的是数组而非拼接好的单个字符串。每行是一个独立元素，**结构与展示分离**：后端只管"说什么"，前端决定"怎么排版"（练习版前端用 `lines.join("\n")` 拼成多行文本）
- **行内注释** — 每个元素自带说明文字，这一行就同时承载了"路径"和"用途"两个信息

### tauri.conf.json — 应用配置的"唯一真源"

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "productName": "e02-project_structure",
  "version": "0.1.0",
  "identifier": "com.taurilearn.e02",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1422",
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build"
  },
  "app": {
    "windows": [
      {
        "title": "练习 E02: 项目结构",
        "width": 800,
        "height": 600
      }
    ]
  }
}
```

| 字段 | 本练习的值 | 作用 |
|---|---|---|
| `productName` | `e02-project_structure` | 应用的显示名，打包后出现在安装包/窗口标题 |
| `identifier` | `com.taurilearn.e02` | 应用唯一标识（反向域名格式），安装/更新时识别应用身份 |
| `build.devUrl` | `http://localhost:1422` | 开发时 WebView 加载的地址（练习 03 详讲） |
| `build.frontendDist` | `../dist` | 打包时前端产物目录（练习 03 详讲） |
| `app.windows[0]` | 800×600，标题"练习 E02: 项目结构" | 主窗口的初始尺寸与标题 |

注意 `productName` 与 Cargo.toml 的 `name`（`e02_project_structure`）**不是同一个东西**：前者是打包后的应用名（用连字符），后者是 Rust crate 名（用下划线）。两个世界，两套名字。

### build.rs、capabilities/、icons/ — 三件"配角"

```rust
// build.rs — 构建脚本
fn main() {
    tauri_build::build()
}
```

Rust 的 `build.rs` 在编译 crate 之前先执行。`tauri_build::build()` 做的是：校验 `tauri.conf.json`、处理图标、生成 `generate_context!` 需要的上下文代码。**你几乎永远不会改它**，但它解释了"为什么配置文件写错会在编译期就报错"。

`capabilities/default.json`：

```json
{
  "identifier": "default",
  "description": "Default capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:default"
  ]
}
```

Tauri v2 用 capabilities 声明"哪个窗口拥有哪些权限"。**自己用 `#[tauri::command]` 定义的命令不需要额外权限**——练习 01 能直接调用，就是因为这条规则（插件提供的 API 才需要权限，后续练习会展开）。

`icons/` 里是应用图标（`icon.ico`、`icon.png` 及各尺寸变体），打包时会被嵌入可执行文件（练习 08 详讲）。

| 文件 | 手写还是自动 | 你的任务 |
|---|---|---|
| `src/main.rs` | 手写 | 基本不动（入口只调用 run()） |
| `src/lib.rs` | 手写 | 主要工作区（命令都在这里） |
| `Cargo.toml` | 手写 | 加依赖时改 |
| `tauri.conf.json` | 手写 | 窗口/构建/标识配置 |
| `build.rs` | 手写 | 基本不动 |
| `capabilities/default.json` | 手写 | 需要权限时改 |
| `icons/` | 半自动 | 用 `cargo tauri icon` 生成（练习 08） |
| `gen/schemas/` | 自动 | 不要碰（工具链生成，改了也会被覆盖） |

> **设计思路：** 把"手写的"和"自动生成的"分开，是工程上的基本纪律。自动生成物改了也会被覆盖，纯属浪费时间；而手写文件里，`lib.rs` 和 `tauri.conf.json` 才是你 90% 的时间花的地方。

---

## 3. 数组返回值 — 后端说"什么"，前端管"怎么排"

### 为什么返回 `Vec<String>` 而不是拼接好的长字符串

想象两种设计：

```rust
// 方案 A：后端拼好一整段文本
fn project_layout() -> String { "src/\n  index.html\n...".into() }

// 方案 B：后端返回行数组（本练习）
fn project_layout() -> Vec<String> { vec!["src/", "  index.html", "..."] }
```

两种方案在页面上看起来一模一样。区别在于**数据边界**：方案 A 把"排版"的决定权也交给了后端——如果将来前端想给每一行加颜色、加缩进、点击跳转，就得重新解析字符串；方案 B 里每行是独立元素，前端想怎么处理都行。

> **关键理解：** 命令的返回值定义的是"数据的形状"，不是"页面的样子"。`Vec<T>` 意味着"一组同构的东西"，前端拿到数组后可以 `map`、`join`、逐个处理。这个思路在练习 05（多参数与结构体）会进一步展开。

### 前端：一行代码渲染结构树

```typescript
import { invoke } from "@tauri-apps/api/core";

const layoutEl = document.querySelector<HTMLPreElement>("#layout");

async function render() {
  // 后端返回结构说明行数组，前端按行渲染
  const lines = await invoke<string[]>("project_layout");
  layoutEl!.textContent = lines.join("\n");
}

render().catch((e) => {
  layoutEl!.textContent = `调用失败: ${e}`;
});
```

- `invoke<string[]>("project_layout")` — 泛型 `string[]` 对应 Rust 的 `Vec<String>`（练习 01 讲过的类型映射：`Vec<T>` → `T[]`）
- `lines.join("\n")` — 数组 → 多行文本。**这里就是"结构与展示分离"的合流点**：后端说"什么"，前端用 `join` 决定"怎么排"
- `layoutEl!.textContent` — `<pre id="layout">` 保留空格和换行，是展示多行文本的标准标签
- `render().catch(...)` — 命令失败时兜底展示错误（练习 01 的固定模式）

> **练习流程：** 练习版前端只有两个 TODO（取消注释 `invoke` + 补渲染），后端两个 TODO（加 `#[tauri::command]` 属性 + 补结构说明行 + 注册）。注意练习版 `project_layout` 返回的是空 `vec![]`——如果你只填了后端忘了前端，页面会显示空白；反之只改前端不改后端，会得到"命令未找到"。

---

## 知识点连起来看

```
┌──────────────────────────────────────────────┐
│ 前端世界            Rust 世界                 │
│ src/                src-tauri/               │
│  ├── index.html      ├── src/main.rs (入口)   │
│  ├── src/main.ts     ├── src/lib.rs (命令)    │
│  ├── src/styles.css  ├── Cargo.toml (依赖)    │
│ package.json         ├── tauri.conf.json (配置)│
│ vite.config.ts       ├── capabilities/ (权限)  │
│ tsconfig.json        ├── icons/ (图标)        │
│                     └── build.rs (构建脚本)   │
└──────────────────────────────────────────────┘
```

| 层 | 本课回答的问题 | 关键概念 |
|---|---|---|
| 目录结构 | 两个世界的边界在哪 | src/ vs src-tauri/、双 crate |
| 文件职责 | 每个文件干什么 | 手写 vs 自动生成、workspace |
| 权限模型 | 窗口能做什么 | capabilities、core:default |
| 数据边界 | 后端返回什么形状 | `Vec<String>`、结构与展示分离 |

**一通百通的核心：** 目录结构是"双进程架构"的物理投影——前端世界负责界面，Rust 世界负责能力，`tauri.conf.json` 是唯一的连接点。学会区分"手写 / 自动生成 / 约定路径"三类文件后，任何 Tauri 项目你都能在两分钟内找到入口、命令和配置。

**递进关系：** 练习 03 将解释"开发时窗口里的页面从哪来"——`devUrl` 与 `frontendDist` 如何决定两种加载方式，以及 `AppHandle` 注入如何让命令读取运行时配置。