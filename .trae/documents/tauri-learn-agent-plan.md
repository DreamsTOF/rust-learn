# Tauri v2 练习项目 — Agent 编写规划

## 1. 概述

约 **84 个练习**（每练习含练习版 + 答案版，共 168 个项目）由 AI Agent 自动编写。本文档定义编写流水线、并发策略、验证门禁和质量标准。

**与纯 Rust 练习的差异：**
- 每个练习同时管理 Rust 后端（`src-tauri/`）和 Web 前端（`src/`）
- 需要 `tauri.conf.json`、`capabilities/`、`icons/` 等额外配置
- 前端依赖通过 pnpm 管理，使用 `cargo tauri` CLI

### 核心原则

| 原则 | 说明 |
| ---- | ---- |
| **模板驱动** | 所有项目由 `scripts/new-exercise.ps1` 从模板生成，零手工脚手架 |
| **并行最大化** | 三大块（入门/命令/简单项目）无依赖，同时并发；超级项目串行 |
| **验证前置** | 每个项目写完即 `cargo tauri build --no-bundle`，不过关不留到下阶段 |
| **幂等生成** | 同一 agent 重复运行同一道题产生相同代码 |

---

## 2. 编写流水线总览

```
┌───────────────────────────────────────────────────────────────┐
│  Phase 0: 脚手架（1 agent, ~1h）                               │
│  workspace + templates + scripts + 00_preface                  │
│  预注册所有 workspace member                                   │
└───────────────────────┬───────────────────────────────────────┘
                        ↓
┌───────────────────────────────────────────────────────────────┐
│  Phase 1: 三大块并行编写（12 agents, ~2-3h）                   │
│                                                                │
│  入门（8 题）       → 2 agents × 4 题                          │
│  命令语法（40 题）  → 8 agents × 5 题                          │
│  简单项目（10 题）  → 2 agents × 5 题                          │
│                                                                │
│  全部同时启动，互不依赖                                        │
└───────────────────────┬───────────────────────────────────────┘
                        ↓
┌───────────────────────────────────────────────────────────────┐
│  Phase 2: 超级项目（1 agent 串行, ~10-12h）                    │
│  Markdown 编辑器 p01 → p26，每步依赖前一步代码                 │
│  分 6 个阶段：骨架→核心编辑→体验→桌面→高级→发布                │
└───────────────────────┬───────────────────────────────────────┘
                        ↓
┌───────────────────────────────────────────────────────────────┐
│  Phase 3: 全量验证（并行, ~1-2h）                              │
│  所有练习编译通过 → 项目锁定                                   │
└───────────────────────────────────────────────────────────────┘
```

**墙钟总时间：~14-18h**（单 agent 串行约 80h）

---

## 3. 项目模板

### 3.1 标准项目结构

```
02_commands/e10_async_command/
├── package.json               # 前端依赖
├── tsconfig.json
├── vite.config.ts             # 固定模板
├── index.html                 # 前端入口
├── src/
│   ├── main.ts                # 前端逻辑（TODO 或答案）
│   └── styles.css
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json        # devUrl 端口 = 1420 + 题号
│   ├── build.rs               # 固定模板
│   ├── capabilities/default.json
│   ├── icons/
│   └── src/
│       ├── lib.rs             # Rust 后端（TODO 或答案）
│       └── main.rs            # 固定模板
└── README.md                  # 题目说明
```

### 3.2 固定模板文件

`vite.config.ts`：

```typescript
import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: { ignored: ["**/src-tauri/**"] },
  },
});
```

`src-tauri/src/main.rs`：

```rust
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri_learn_lib::run()
}
```

`src-tauri/build.rs`：

```rust
fn main() {
    tauri_build::build()
}
```

`src-tauri/capabilities/default.json`：

```json
{
  "identifier": "default",
  "description": "Default capability for the main window",
  "windows": ["main"],
  "permissions": ["core:default"]
}
```

### 3.3 练习 lib.rs / main.ts 模板

```rust
// ============================================================
// 练习 NNN: 题目名称
// 目标: 一句话描述知识点
// TODO: 按照注释提示补全代码
// ============================================================

use tauri::Manager;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 添加 greet 命令，接受 name: &str，返回 "你好, {name}!"

// === 步骤 2 ——————————————————————————————————————————
// TODO: 注册到 invoke_handler

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 你的代码在这里
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
```

```typescript
// ============================================================
// 练习 NNN: 题目名称
// 目标: 一句话描述知识点
// TODO: 按照注释提示补全代码
// ============================================================

import { invoke } from "@tauri-apps/api/core";

// === 步骤 1 ——————————————————————————————————————————
// TODO: 调用 greet 命令并显示结果
// 提示: invoke("greet", { name: "Tauri" })
```

### 3.4 难度引导

| 块 | TODO 密度 | 代码完成度 |
| -- | --------- | ---------- |
| 入门 | 逐行 TODO | 填空即可 |
| 命令语法 | 关键位置 TODO | 补全约 50% |
| 简单项目 | 仅功能目标 | 骨架 + 自写 |
| 超级项目 | 每步功能描述 | 几乎全写 |

---

## 4. 并发策略

每个练习项目之间**完全独立**（无共享代码），并发粒度可下探到 batch 级。

### Batch 分配矩阵

| 块 | 题量 | Agent 数 | 划分 |
| :-: | :--: | :------: | ---- |
| 入门 | 8 | 2 | e01-04 / e05-08 |
| 命令语法 | 40 | 8 | 每 agent 5 题：e09-13 / e14-18 / e19-23 / e24-28 / e29-33 / e34-38 / e39-43 / e44-48 |
| 简单项目 | 10 | 2 | e49-53 / e54-58 |
| 超级项目 | 26 步 | 1 | p01→p26 串行（强依赖链，分 6 个阶段） |

**Phase 1 合计：12 agents 同时运行**，全部互不依赖。

```
时间线 →
入门:   [A1a e01-04][A1b e05-08]
命令:   [A2a e09-13][A2b e14-18]...[A2h e44-48]   ← 同时启动
项目:   [A3a e49-53][A3b e54-58]
        └────── Phase 1 全并发 ~2-3h ──────→
```

### 总并发量

| Phase | 并行 agents | 墙钟时间 |
| :---: | :---------: | :------: |
| 0 | 1 | ~1h |
| 1 | 12 | ~2-3h |
| 2 | 1（串行） | ~10-12h |
| 3 | 4（分块并行验证） | ~1-2h |

---

## 5. Agent 编写指令规范

### 5.1 统一 Agent Prompt 模板

```
## 任务: 编写「块名」Batch eX-eY

### 上下文
- 工作区: c:\code\testruetlearn\tauri-learn\
- 负责题号: eX 到 eY（共 N 题）
- 每道题创建 2 个项目：练习版（TODO）+ 答案版（完整代码）
- 依赖 crate / 前端依赖 / Tauri 版本 2.11.x / stable

### 编写流程（对每道题循环）:
1. 创建练习项目:
   `powershell scripts/new-exercise.ps1 -Chapter "02_commands" -Number 10 -Name "async_command"`
2. 按难度模板写 lib.rs 和 main.ts（仅 TODO，不含答案）
3. 安装依赖: `pnpm install`
4. 编译验证: `cargo tauri build --no-bundle`
5. 创建答案项目（同上，名称加 `_answer`），写完整代码
6. 编译验证答案项目
7. 编译报错则修复重试，最多 3 次；仍失败记入 build-errors.log 并跳过

### 质量要求:
- 练习/答案都须通过 `cargo tauri build --no-bundle`
- 练习只含 TODO，答案不含 TODO
- 前端 TypeScript strict 模式
```

### 5.2 脚手架脚本

`scripts/new-exercise.ps1` 接收 `-Chapter -Number -Name` 参数，生成完整项目：
package.json、vite.config.ts、tsconfig.json、index.html、src/main.ts、src/styles.css、src-tauri/Cargo.toml、tauri.conf.json（devUrl 端口 = 1420 + 题号）、build.rs、capabilities/default.json、icons/、src/lib.rs、src/main.rs，并注册 workspace member、生成 README.md。

> 端口规则：`1420 + 题号`，答案项目同名端口 + 标题加"（答案）"后缀。

---

## 6. 验证门禁

### 每道题（agent 自检）

```
G1: cargo tauri build --no-bundle 通过   ← 零错误零警告
G2: 答案项目存在且可编译
G3: 端口无冲突
     ↓ 不通过 → 修复重试（最多 3 次）
```

### 块级验证（Phase 3，验证 agent 执行）

```
G4: 编号连续无跳号
G5: 块内所有项目编译通过
G6: 每项目都有 lib.rs + main.ts + tauri.conf.json + README.md
```

### 全量验证

```
G7: workspace 整体编译
G8: 每项目 pnpm build 通过
G9: 随机抽 3 个项目 `tauri dev` 启动抽检
```

验证通过后对应块锁定，不再修改。

---

## 7. 质量保障

- Rust：`cargo fmt`，`#[tauri::command]` 标注，serde derive，`thiserror` 错误类型，`.await` 正确处理 `Result`
- 前端：strict 模式，`invoke` 用 async/await 或 .then/.catch，事件监听返回 `unlisten`
- 配置：唯一 identifier `com.taurilearn.eNNN`，`frontendDist` → `../dist`，devUrl 端口 = `1420 + 题号`，窗口 label = `main`

### 常见问题预防

| 问题 | 预防措施 |
| ---- | -------- |
| 端口冲突 | 每题使用 `1420 + 题号` |
| 图标缺失 | 脚手架自动生成占位图标 |
| 权限不足 | 默认 capabilities 含 `core:default` |
| 异步命令借用问题 | 用 `Result<_, _>` 包装返回值，避免引用跨 await |
| 前端 invoke 路径错误 | 统一 `@tauri-apps/api/core` 的 `invoke` |

---

## 8. 故障恢复

- **编译失败**：读错误 → 识别类型（语法/类型/依赖/配置）→ 修复重试 ≤3 次 → 记录 `build-errors.log` 跳过
- **验证失败**：标记失败题目 → 通知对应 agent 修复 → 重新验证（≤2 次机会）
- **Agent 中断**：扫描目录跳过已完成题号，从未完成处继续；抽查已完成项目符合标准

---

## 9. 附录

### 附录 A：核心 API 速查

| API | 用途 | 位置 |
| --- | ---- | ---- |
| `#[tauri::command]` / `generate_handler![]` | 定义/注册命令 | lib.rs |
| `Builder::manage()` / `State<'_, T>` | 共享状态 | lib.rs / 命令参数 |
| `Builder::setup()` | 初始化钩子 | lib.rs |
| `app_handle.emit()` / `listen()` | 后端事件收发 | Rust 端 |
| `window.set_size()` / `set_position()` | 窗口操作 | Rust 端 |
| `Menu::new()` / `SystemTrayBuilder` | 菜单/托盘 | Rust 端 |
| `invoke("cmd", {args})` | 前端调用命令 | 前端 |
| `listen("event", fn)` / `emit("event", payload)` | 前端事件 | 前端 |
| `@tauri-apps/api/core` | 核心 API 包 | 前端 |

### 附录 B：插件速查

| 插件 | crate | npm 包 | 功能 | 对应练习 |
| ---- | ----- | ------ | ---- | -------- |
| FS | `tauri-plugin-fs` | `@tauri-apps/plugin-fs` | 文件系统 | 30 |
| Dialog | `tauri-plugin-dialog` | `@tauri-apps/plugin-dialog` | 对话框 | 31 |
| Shell | `tauri-plugin-shell` | `@tauri-apps/plugin-shell` | 命令执行 | 32 |
| SQL | `tauri-plugin-sql` | `@tauri-apps/plugin-sql` | SQLite | 33 |
| Store | `tauri-plugin-store` | `@tauri-apps/plugin-store` | 键值持久化 | 34 |
| Notification | `tauri-plugin-notification` | `@tauri-apps/plugin-notification` | 系统通知 | 35 |
| Clipboard | `tauri-plugin-clipboard-manager` | `@tauri-apps/plugin-clipboard-manager` | 剪贴板 | 36 |
| HTTP | `tauri-plugin-http` | `@tauri-apps/plugin-http` | HTTP 请求 | 37 |
| OS / Opener | `tauri-plugin-os` / `tauri-plugin-opener` | 同左 | 系统信息/打开 | 38 |
| Global Shortcut | `tauri-plugin-global-shortcut` | `@tauri-apps/plugin-global-shortcut` | 全局快捷键 | 39 |
| Window State | `tauri-plugin-window-state` | `@tauri-apps/plugin-window-state` | 窗口状态 | 29 |
| Updater | `tauri-plugin-updater` | `@tauri-apps/plugin-updater` | 自动更新 | 48 |
| Single Instance | `tauri-plugin-single-instance` | `@tauri-apps/plugin-single-instance` | 单实例 | 20 |

---

*本文档是 `tauri-learn-plan.md` 的配套文档，定义 Agent 如何并行生成练习。*
*上一文档：[tauri-learn-plan.md](tauri-learn-plan.md) — 练习内容规划。*