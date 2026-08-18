# Tauri v2 练习项目 — 实现计划

## 概述

在 `c:\code\testruetlearn\tauri-learn\` 下创建 Cargo workspace，内含约 **84 个可运行练习**，按四大块组织：

```
入门（8 题） → 基本命令和语法（40 题） → 简单项目（10 题） → 超级项目（26 步）
```

**设计原则：**

| 原则 | 说明 |
| ---- | ---- |
| **一题一知识面** | 每题覆盖一个知识点的**完整 API 族**（含常用变体与组合），一次练透；不做"返回字符串/数字/布尔"式的变体轰炸 |
| **项目巩固** | 简单项目综合运用 3-5 个知识点，超级项目串起全栈——知识靠项目内化，不靠重复 |
| **即学即用** | 全部练习可 `tauri dev` 一键运行，所见即所得 |
| **练习 + 答案** | 每道题提供含 TODO 的练习版和完整答案版两个独立项目 |

**目标读者：** 已有 Rust 基础、希望学习 Tauri v2 桌面应用开发的开发者。

---

## 项目结构

```
tauri-learn/
├── Cargo.toml                        # workspace
├── rust-toolchain.toml               # stable 工具链
├── package.json / pnpm-workspace.yaml
│
├── 00_preface/                       # 练习导航首页
├── templates/                        # 练习模板（minimal + vite-ts）
├── scripts/new-exercise.ps1          # 脚手架脚本
│
├── 01_getting_started/               # 第一块：入门（e01-e08）
├── 02_commands/                      # 第二块：基本命令和语法（e09-e48）
├── 03_simple_projects/               # 第三块：简单项目（e49-e54）
├── 04_super_project/                 # 第四块：超级项目（p01-p26，串行递进）
└── 00_preface/
```

每个练习是独立 Tauri 项目：

```bash
cd 01_getting_started/e01_hello_world
cargo tauri dev              # 运行
cargo tauri build --no-bundle   # 仅编译验证
```

---

## 技术选型

| 项目 | 选型 | 原因 |
| ---- | ---- | ---- |
| Tauri 版本 | **2.11.x** | 最新稳定 |
| Rust 通道 | **stable** | 无需 nightly |
| 前端 | **Vanilla TS / Vite**，超级项目用 **React** | 最小依赖起步，框架集成在高级练习中 |
| 包管理器 | **pnpm** | monorepo 友好 |
| 错误处理 | **thiserror + serde** | 推荐模式 |

### 依赖版本锁定（workspace）

```toml
[workspace.dependencies]
tauri = { version = "2.11", features = [] }
tauri-build = "2.6"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
tokio = { version = "1", features = ["full"] }
tauri-plugin-fs = "2.5"
tauri-plugin-dialog = "2.7"
tauri-plugin-shell = "2.3"
tauri-plugin-sql = { version = "2.4", features = ["sqlite"] }
tauri-plugin-store = "2.4"
tauri-plugin-notification = "2.3"
tauri-plugin-clipboard-manager = "2.3"
tauri-plugin-http = "2.5"
tauri-plugin-os = "2.3"
tauri-plugin-opener = "2.5"
tauri-plugin-global-shortcut = "2.3"
tauri-plugin-window-state = "2.3"
tauri-plugin-updater = "2.10"
tauri-plugin-single-instance = "2.3"
```

### API 风格

```rust
// src-tauri/src/lib.rs
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
```

```typescript
// src/main.ts
import { invoke } from "@tauri-apps/api/core";

const greeting = await invoke("greet", { name: "Tauri" });
```

### tauri.conf.json 模板

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "productName": "e01-hello-world",
  "version": "0.1.0",
  "identifier": "com.taurilearn.e01",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1420",
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build"
  },
  "app": {
    "windows": [{ "title": "练习 01: Hello World", "width": 800, "height": 600 }]
  }
}
```

---

## 难度规则

| 块 | 难度 | 引导程度 |
| -- | ---- | -------- |
| 入门 | ⭐ | 逐行 TODO，只需填空 |
| 基本命令和语法 | ⭐ / ⭐⭐ | 关键位置 TODO，补全约 50% |
| 简单项目 | ⭐⭐ / ⭐⭐⭐ | 仅描述功能目标与用到知识点 |
| 超级项目 | ⭐⭐⭐ | 每步描述功能，代码几乎全写 |

每道题提供 **练习版（含 TODO）** 和 **答案版（完整代码）** 两个项目。

---

## 内容大纲（四大块，84 个练习）

### 第一块：入门（8 题）

**目标：** 把环境搭好，跑通第一个应用，理解项目骨架。

| # | 题目 | 核心知识点 |
| - | ---- | ---------- |
| 01 | 环境准备与项目创建 | `create-tauri-app`，前置条件检查 |
| 02 | 项目结构 | `src/` vs `src-tauri/`，`lib.rs` vs `main.rs` |
| 03 | 运行与构建 | `tauri dev` / `tauri build`，`devUrl` / `frontendDist` |
| 04 | 第一个命令 | `#[tauri::command]` + `generate_handler!` + `invoke()` |
| 05 | 参数与返回值 | 字符串/数字/结构体/Vec，serde 序列化 |
| 06 | 窗口配置 | `title`/`width`/`height`/`center`，多窗口配置 |
| 07 | 调试 | Web Inspector，`println!` 与日志 |
| 08 | 打包与图标 | `tauri build` 产物，`cargo tauri icon`，identifier 规范 |

### 第二块：基本命令和语法（40 题）

**目标：** 覆盖日常开发 90% 会用到的 API。**一题一知识面**——每题练透一个知识点的完整 API 族（常用变体与组合一次练完），不留最小切片。

#### 2.1 命令系统（7 题）

| # | 题目 | 横向覆盖的 API 族 |
| - | ---- | ----------------- |
| 09 | 异步命令 | `async fn` 命令、`tokio::time::sleep`、`tokio::fs`、`tokio::time::timeout` 超时 |
| 10 | 依赖注入 | 注入 `AppHandle`、`WebviewWindow`、`State<T>`，多依赖组合注入 |
| 11 | 错误处理 | `Result<T, String>`、`thiserror` 错误枚举、`#[from]` 错误链、`map_err` 转换 |
| 12 | Channel 流式传输 | `Channel::new`、`send` 推送、进度更新、前端 `onmessage` 消费 |
| 13 | 命令模块化 | `commands/` 目录拆分、子模块定义命令、跨模块 `generate_handler!` 注册 |
| 14 | 可变状态 | `manage()`、`State<T>` 读取、`Mutex` / `RwLock`、多个 State 并存 |
| 15 | 前后端类型同步 | TS 接口定义、`invoke<T>()` 泛型、`serde rename`（camelCase）、`tauri-specta` 可选 |

#### 2.2 状态与生命周期（5 题）

| # | 题目 | 横向覆盖的 API 族 |
| - | ---- | ----------------- |
| 16 | setup 钩子 | `Builder::setup()`、异步初始化、`run_on_main_thread`、窗口创建后回调 |
| 17 | 退出拦截 | `ExitRequested` / `CloseRequested`、`prevent_exit`、确认对话框后放行退出 |
| 18 | 路径 API | `app_data_dir` / `app_config_dir` / `app_log_dir` / `resource_dir` / `temp_dir`、路径拼接 |
| 19 | 后台任务 | `async_runtime::spawn`、`spawn_blocking`、任务完成结果回传（事件/Channel） |
| 20 | 单实例 | `single-instance` 插件、重复启动唤起已有窗口、携带启动参数 |

#### 2.3 事件（3 题）

| # | 题目 | 横向覆盖的 API 族 |
| - | ---- | ----------------- |
| 21 | 前端事件 | `listen` / `emit` / `unlisten` / `once`、复杂 payload 的类型化 |
| 22 | 窗口级事件 | `emitTo` 定向发送、指定窗口监听、全局 vs 窗口级事件差异 |
| 23 | 后端监听 | `app_handle.listen`、事件过滤、后端转发到其他窗口 |

#### 2.4 窗口、菜单与托盘（6 题）

| # | 题目 | 横向覆盖的 API 族 |
| - | ---- | ----------------- |
| 24 | 创建与操作窗口 | `WebviewWindowBuilder`、label 管理、show/hide、`set_size` / `set_position` / center |
| 25 | 窗口事件 | `on_window_event`、Resized / Moved / CloseRequested / 焦点变化 |
| 26 | 无边框窗口 | `decorations: false`、`data-tauri-drag-region`、自定义标题栏 + 最小化/关闭按钮 |
| 27 | 应用菜单 | `Menu` / `Submenu` / `MenuItem` / `CheckMenuItem` / separator、快捷键、`set_enabled` |
| 28 | 系统托盘 | `SystemTray` + 图标、托盘菜单、托盘事件、关闭时隐藏到托盘、恢复 |
| 29 | 窗口状态持久化 | `window-state` 插件、位置/大小/最大化自动保存与恢复 |

#### 2.5 核心插件（10 题）

| # | 题目 | 横向覆盖的 API 族 |
| - | ---- | ----------------- |
| 30 | 文件系统 | `readTextFile` / `writeTextFile` / `readFile`（二进制）、`readDir`、`stat`、copy/rename/remove、`exists`、scope 限制 |
| 31 | 对话框 | `open`（文件/目录/多选/过滤器）、`save`、`ask` / `message`、取消结果处理 |
| 32 | Shell | `Command.create` + 参数、stdout/stderr 读取、异步等待、超时、scope 白名单 |
| 33 | SQL | `Database::load`、execute/select、绑定参数、事务提交回滚 |
| 34 | Store | `set` / `get` / `has` / `delete`、`save` / `load`、`watch` 监听变化 |
| 35 | 通知 | 权限检查/请求、`sendNotification`、通知点击事件 |
| 36 | 剪贴板 | `readText` / `writeText`、图片读写、清空 |
| 37 | HTTP | `fetch` GET/POST、Headers 设置、超时、非 2xx 响应处理 |
| 38 | OS 与 Opener | 平台/版本/架构查询、`open` URL / 文件、`reveal` 在资源管理器中显示 |
| 39 | 全局快捷键 | `register` / `unregister`、快捷键事件回调、状态查询 |

#### 2.6 前端集成（5 题）

| # | 题目 | 横向覆盖的 API 族 |
| - | ---- | ----------------- |
| 40 | Vite 与 HMR | `vite.config.ts` Tauri 集成、`TAURI_DEV_HOST`、HMR 端口、strictPort |
| 41 | React 集成 | `useState` + `invoke`、事件监听 hook 封装、组件卸载时 `unlisten` |
| 42 | 主题切换 | CSS 变量、`prefers-color-scheme` 系统主题检测、手动切换 + Store 记忆 |
| 43 | 静态资源 | 图片/字体加载、resource 目录或 `asset` 协议、打包后路径处理 |
| 44 | 内容安全策略 | `security.csp`、script-src / style-src / connect-src 限制 |

#### 2.7 安全与发布（4 题）

| # | 题目 | 横向覆盖的 API 族 |
| - | ---- | ----------------- |
| 45 | 权限系统 | capabilities 配置、自定义 permission 文件、allow/deny、平台限定 |
| 46 | 自定义错误传播 | 前端统一 `.catch` 捕获、错误码区分展示、错误提示组件（承接 11） |
| 47 | 打包发布 | NSIS/MSI/DMG/AppImage 多平台产物、图标、体积优化 |
| 48 | 自动更新 | `updater` 插件、更新源配置、版本检查、下载与安装流程 |

### 第三块：简单项目（10 题）

**目标：** 每个项目综合 3-5 个知识点，做出完整可用的小应用。领域互不重复，覆盖命令语法块的各类 API 组合。

| # | 项目 | 用到的知识点 |
| - | ---- | ------------ |
| 49 | 待办清单 | 命令 + 可变状态 + Store 持久化 |
| 50 | 密码生成器 | 异步命令 + 剪贴板 + 错误处理 |
| 51 | 文件笔记 | FS + Dialog + 事件刷新 |
| 52 | 系统监视器 | OS 插件 + 后台任务 + 托盘 |
| 53 | 番茄计时器 | 事件 + 通知 + 托盘 + 窗口控制 |
| 54 | 图片查看器 | FS + 拖放 + 窗口操作 + 静态资源 |
| 55 | 记账本 | SQL CRUD + 命令 + 状态 + 汇总统计 |
| 56 | 汇率查询 | HTTP + 异步 + Store 缓存 + 错误处理 |
| 57 | 批量重命名 | FS + Dialog + Channel 进度 + 后台任务 |
| 58 | RSS 阅读器 | HTTP + Opener + Store + 事件 |

### 第四块：超级项目（26 步）

**目标：** Markdown 编辑器，串行递进，每步依赖前一步代码，最终交付一个**可安装分发的完整产品**。

**与简单项目的差异：**

| | 简单项目 | 超级项目 |
| :-: | :-: | :-: |
| 规模 | 单页小应用，约 200-400 行 | 多模块产品，5000+ 行，可打包分发 |
| 结构 | 单文件入口 + 少量命令 | 分层架构：数据模型 / 存储 / 命令 / UI 组件 |
| 深度 | 会用 API | 产品级体验：未保存提示、自动保存、撤销重做、状态恢复、错误处理、发布流程 |
| 输出 | 可运行的练习 | 可安装的桌面应用 |

#### 阶段 A：骨架（P1-P3）

| 步骤 | 名称 | 核心功能 | 复用的知识点 |
| :--: | ---- | -------- | ------------ |
| P1 | 项目初始化 | Tauri + React 脚手架，应用菜单，窗口配置 | 40 Vite/HMR、41 React、27 应用菜单 |
| P2 | 数据模型与存储层 | Rust 端定义 Document 结构，命令模块化拆分，Store 持久化偏好 | 13 模块化、14 可变状态、34 Store、15 类型同步 |
| P3 | 编辑器核心 | 集成 CodeMirror/Monaco 编辑器，语法高亮 | 41 React、43 静态资源、10 依赖注入 |

#### 阶段 B：核心编辑（P4-P8）

| 步骤 | 名称 | 核心功能 | 复用的知识点 |
| :--: | ---- | -------- | ------------ |
| P4 | 文件管理 | Dialog 打开/保存/另存为，FS 读写，最近文件列表 | 30 FS、31 Dialog、11 错误处理、46 错误传播 |
| P5 | 实时预览 | Markdown 渲染 + Split 视图，前端状态管理 | 41 React、21 前端事件 |
| P6 | 工具栏 | 加粗/斜体/标题/列表格式命令 | 09 异步命令、15 类型同步 |
| P7 | 多文件标签页 | 标签切换/关闭，未保存提示，退出拦截确认 | 14 状态、21 事件、17 退出拦截、25 窗口事件 |
| P8 | 撤销/重做与字数统计 | 编辑历史栈，实时字数/标题数状态栏 | 14 状态、21 事件、09 异步命令 |

#### 阶段 C：体验完善（P9-P14）

| 步骤 | 名称 | 核心功能 | 复用的知识点 |
| :--: | ---- | -------- | ------------ |
| P9 | 搜索替换 | 全文搜索/替换，菜单快捷键 | 09 异步命令、27 快捷键 |
| P10 | 自动保存 | 定时自动保存，后端保存完成事件通知前端 | 19 后台任务、34 Store、23 后端监听 |
| P11 | 主题系统 | 明暗主题切换 + 偏好记忆 | 42 主题切换、34 Store |
| P12 | 拖放支持 | 拖拽文件/图片打开或插入 | 09 异步命令、30 FS |
| P13 | 图片管理 | 插入图片，本地图片嵌入文档 | 30 FS、31 Dialog、43 静态资源 |
| P14 | 目录导航 | 文档大纲（标题解析），点击跳转 | 前端能力、41 React |

#### 阶段 D：桌面能力（P15-P18）

| 步骤 | 名称 | 核心功能 | 复用的知识点 |
| :--: | ---- | -------- | ------------ |
| P15 | 系统托盘 | 托盘图标，关闭时隐藏到托盘，恢复窗口 | 28 托盘、17 退出拦截 |
| P16 | 全局快捷键 | 注册全局快捷键唤起应用 | 39 全局快捷键 |
| P17 | 多窗口 | 新窗口打开文档，窗口间通信 | 24 创建窗口、22 emitTo、10 依赖注入 |
| P18 | 剪贴板与通知 | 复制/粘贴命令集成，操作结果系统通知 | 36 剪贴板、35 通知 |

#### 阶段 E：高级功能（P19-P22）

| 步骤 | 名称 | 核心功能 | 复用的知识点 |
| :--: | ---- | -------- | ------------ |
| P19 | 导出增强 | 导出 HTML + PDF | 30 FS、19 后台任务 |
| P20 | 云同步 | HTTP 同步到远端，token 存储，冲突处理 | 37 HTTP、34 Store、11 错误处理 |
| P21 | 拼写检查 | 字典加载 + 后端检查命令，错误标注 | 09 异步命令、30 FS、21 事件 |
| P22 | 设置面板 | 默认目录、字号、主题、同步开关等设置 | 34 Store、15 类型同步、46 错误传播 |

#### 阶段 F：发布（P23-P26）

| 步骤 | 名称 | 核心功能 | 复用的知识点 |
| :--: | ---- | -------- | ------------ |
| P23 | 错误处理与日志 | 统一错误提示组件，后端日志文件 | 11 错误处理、46 错误传播 |
| P24 | 安全加固 | CSP 收紧，权限最小化，输入校验 | 45 权限、44 CSP |
| P25 | 打包发布 | 图标、安装包、签名 | 47 打包 |
| P26 | 自动更新与验收 | updater 配置，全功能走查清单 | 48 updater、20 单实例 |

### 知识点覆盖说明

**练习教 API，项目练组合。** 40 道命令语法题中，约 35 个知识点在简单项目/超级项目中被复用内化；其余少数（如 Shell、路径 API）由练习题负责"认识"即可，不硬塞进项目制造死代码。

| 知识点去向 | 覆盖范围 |
| ---------- | -------- |
| 超级项目复用 | ~35/40，含命令、状态、事件、窗口、插件、前端、同步、发布全链路 |
| 简单项目补齐 | 10 个项目覆盖 SQL、HTTP、Channel、Opener 等场景性 API 的组合用法 |
| 练习题认识即可 | Shell、路径 API 等少量场景性 API |

---

## 总结

| 块 | 目录 | 练习数 | 定位 |
| :-: | ---- | :----: | ---- |
| 入门 | `01_getting_started/` | 8 | 跑起来，看懂骨架 |
| 基本命令和语法 | `02_commands/` | 40 | 一题一知识面，覆盖完整 API 族 |
| 简单项目 | `03_simple_projects/` | 10 | 领域各异的综合应用，覆盖 SQL/HTTP/Channel 等 API 组合 |
| 超级项目 | `04_super_project/` | 26 步 | 全栈实战，可安装分发的完整产品 |

**总量：** 84 个练习（每练习含练习版 + 答案版，共 168 个项目）。
**与旧规划的差异：** 从 350 题砍到 84 题——删掉所有变体题（返回类型、窗口属性、路径目录等各自成题），保留每个知识点最核心的一次练习，用项目代替重复；简单项目与超级项目保留足够容量，不让实战环节缩水。

下一文档：[tauri-learn-agent-plan.md](tauri-learn-agent-plan.md) — Agent 编写流水线。