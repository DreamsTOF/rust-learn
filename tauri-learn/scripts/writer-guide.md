# Tauri 练习编写指南（Agent 必读）

本文档定义 02_commands 块练习的编写规范。**开工前完整阅读**，并阅读以下模式参照文件：

- 答案版参照: `01_getting_started/e04_first_command_answer/src-tauri/src/lib.rs`、`src/main.ts`、`index.html`
- 练习版参照: `01_getting_started/e04_first_command/src-tauri/src/lib.rs`、`src/main.ts`
- 通用样式: `01_getting_started/e01_hello_world_answer/src/styles.css`（复制到每个项目作为基础，可追加类）
- 多窗口参照: `01_getting_started/e06_window_config_answer/src-tauri/src/lib.rs`
- React 模板参照（仅 e41 需要）: `04_super_project/p01_project_init/`（package.json、vite.config.ts、tsconfig.json、index.html、src/main.tsx、src/App.tsx）

## 1. 流程（务必遵守）

1. **先写全部 N 题答案版**（lib.rs + main.ts + index.html + README.md + 需要的配置），每写完一题立即 `cargo check` 验证
2. 全部答案版通过后，**再回头基于答案挖空写练习版**（练习版也逐一 `cargo check`）

## 2. 背景与约束

- 项目目录: `02_commands/eNN_name`（练习版）和 `02_commands/eNN_name_answer`（答案版），骨架已由脚手架生成
- 每个项目已有: package.json、tsconfig.json、vite.config.ts、index.html、src/styles.css、src-tauri/（Cargo.toml、tauri.conf.json、capabilities/default.json、icons/、src/lib.rs 骨架、src/main.rs、build.rs）、README.md
- **不要改动**: tauri.conf.json 的 identifier/端口/devUrl/productName、icons/、src-tauri/src/main.rs、vite.config.ts 的端口
- 需要新依赖时改 `src-tauri/Cargo.toml`，用 `{ workspace = true }` 引用（tokio、tauri-plugin-* 均已注册在 workspace.dependencies）
- 前端 npm 依赖改 package.json（版本风格: @tauri-apps/api 用 ^2.11.1，插件包用 ^2）
- capabilities 权限加进 `src-tauri/capabilities/default.json` 的 permissions 数组

## 3. 文件头注释格式（lib.rs / main.ts 首部）

```
// ============================================================
// 练习 E19: 后台任务
// 目标: <一句话>
// 知识点: <知识点>
// ============================================================
```

练习版在知识点行后加一行: `// TODO: 按照注释提示补全代码`

## 4. 答案版要求

- 完整可运行；`cargo check` 零错误零警告；TypeScript strict 零错误（无未使用变量）
- Rust 命令用 `#[tauri::command]`，注册进 `tauri::generate_handler![]`
- 前端统一 `import { invoke } from '@tauri-apps/api/core'`（或插件包 API），错误用 try/catch 展示
- 前端 DOM 结构放 index.html，用现有 styles.css 的类（.card .field .row .status .checklist .kv 等），需要时可在项目自己的 styles.css 追加类（答案/练习两版一致）

## 5. 练习版挖空规则（关键）

### 5.1 Rust

- TODO 处保持**可编译**：空 `vec![]`、占位值 `String::new()` / `None` / `0` / `false`、下划线参数 `_app`
- 挖掉 `#[tauri::command]` 属性（TODO 注释提示添加）；注册行注释掉（TODO 提示 `.invoke_handler(tauri::generate_handler![...])`）
- 文件顶部（注释块之后）加：

```
// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]
```

- 新增依赖（Cargo.toml）、插件 init（`.plugin(...)` 行）、capabilities、前端 npm 依赖：**答案/练习两版保留一致**（练习版也要能编译）

### 5.2 TypeScript

- `import { invoke }` 注释掉，上一行加 `// TODO: 完成填空后取消注释（invoke 用于调用后端命令）`
- invoke 调用改为类型化占位（如 `const x: T[] = [];` 或 `const x: T = { ...占位字段 };`）并留 TODO 注释说明完整写法；**渲染逻辑保留对变量的引用**（否则 noUnusedLocals 报错）
- 被挖空的中间变量（如 `const name = input.value.trim()`）不要留在代码里，写进 TODO 注释
- 插件 import（@tauri-apps/plugin-*）保留在两版中，练习版挖的是调用逻辑

### 5.3 TODO 格式

```
// === 步骤 N: 步骤名 ————————————————————————————
// TODO: 具体描述（要做什么）
// 提示: 关键代码（答案的核心一行）
```

每题 3-6 个 TODO 点，覆盖：命令定义、注册、invoke 调用、渲染/传参等关键位置。

### 5.4 底线

练习版挖空后必须也能 `cargo check` 通过（dead_code 已被 allow，其余零警告）。

## 6. 验证命令

在 `c:\code\testruetlearn\tauri-learn` 根目录运行:

```
cargo check -p eNN-name-answer -p eNN-name
```

（-p 名称格式: `eNN-name` / `eNN-name-answer`，name 含下划线；可一次 check 多个）

前端不验证（依赖由主 agent 统一安装），但写代码时确保 TS 语法正确、无未使用变量。

## 7. 已知 API 陷阱（已踩过，勿再犯）

- `PackageInfo` **没有** identifier 字段！identifier 在 `app.config().identifier`（String，直接 `.clone()`）
- `config.build.frontend_dist` 是 `Option<FrontendDist>`，用 `.as_ref().map(|d| d.to_string()).unwrap_or_default()`
- `app.config()` / `app.package_info()` 是 AppHandle 固有方法，**不需要** `use tauri::Manager`；`app.get_webview_window()` / `app.webview_windows()` / `app.windows()` 需要 `use tauri::Manager`
- 前端 invoke 参数名 camelCase，与 Rust snake_case 参数自动对应
- 事件: Rust 侧 emit 用 `use tauri::Emitter`（`app.emit` / `app.emit_to` / `window.emit`）；Rust 侧 listen 用 `use tauri::Listener`（`app.listen`）；前端 listen/emit 用 `@tauri-apps/api/event`
- 窗口: `WebviewWindowBuilder::new(&app, 'label', WebviewUrl::App('index.html'.into()))`；`app.get_webview_window('main')` 返回 Option
- 练习版 TS 中 interface 必须被占位变量引用，否则 noUnusedLocals 报错

## 8. 交付清单

每题 2 个目录（练习/答案）各含：lib.rs（+ 需要的模块文件）、main.ts、index.html、styles.css、README.md（简短：题目名、知识点、运行方式；练习版注明"对照答案: eNN_xxx_answer/"）、Cargo.toml / capabilities / package.json（如需）。

完成后汇报：每题答案版/练习版 cargo check 结果、与设计描述的差异说明。