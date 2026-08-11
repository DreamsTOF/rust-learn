# 练习 03: 运行与构建

## 为什么要学这个

练习 01、02 你一直在 `cargo tauri dev` 下开发，但这条命令背后发生了什么，没人解释。这一章要回答三个问题：

1. **`cargo tauri dev` 到底启动了几个进程？** — 一条命令，Vite、cargo、窗口是怎么被编排起来的？
2. **命令怎么"认识"自己的应用？** — 一个命令函数还能收到框架注入的对象？`AppHandle` 是什么？
3. **为什么有的配置字段是 `Option`？** — `devUrl` 和 `frontendDist` 一个"可有可无"，一个"一定有"，前端怎么处理这种差异？

把这三件事弄懂，你就掌握了 Tauri 开发循环的完整图景——它直接决定了你每天写代码的节奏。

---

## 从问题出发

练习 03 要做的事：**读取应用自身的构建配置（devUrl、frontendDist、identifier、productName），把它显示在窗口里**。

网页开发有个幸福的习惯：改一行代码，刷新一下浏览器就是新的。但桌面应用做不到——它需要"重新编译 + 重启窗口"。如果每改一行字都要等几十秒，开发体验就是灾难。

**核心矛盾：** 窗口里跑的是网页（WebView），而网页可以被"服务器"服务。于是 Tauri 想到：**开发时把前端托管给一个 dev server**，前端就能像网页开发一样热更新；Rust 端则保持"编译后加载"的传统方式。一条命令把两套流程编排起来：

```
cargo tauri dev
│
├── ① 启动前端 dev server        beforeDevCommand: "pnpm dev"
│      └── Vite 监听 1424 端口，等待就绪
├── ② 编译 Rust（debug 模式）    cargo build
│      └── 启动应用进程
└── ③ WebView 加载 devUrl        http://localhost:1424
       ├── 前端文件改动 → Vite 热更新推送（秒级）
       └── Rust 文件改动 → 重新编译并重启（较慢）
```

而"应用自己怎么知道 devUrl 是什么"——这正是本练习命令做的事：**通过 `AppHandle` 读取运行时配置**。这就是第二个知识点。

---

## 1. cargo tauri dev / build — 两种装配形态

### tauri.conf.json 里的两条命令

```json
"build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1424",
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build"
}
```

| 配置 | 本练习的值 | 什么时候被用到 |
|---|---|---|
| `beforeDevCommand` | `pnpm dev` | `tauri dev` 启动前执行：把 Vite dev server 拉起来 |
| `devUrl` | `http://localhost:1424` | dev 模式下 WebView 加载的地址 |
| `beforeBuildCommand` | `pnpm build` | `tauri build` 打包前执行：把前端构建成静态文件 |
| `frontendDist` | `../dist` | 打包时告诉 Tauri 前端产物在哪 |

`pnpm dev` 又指向 `package.json` 里的脚本：

```json
"scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
}
```

所以链条是：`cargo tauri dev` → `beforeDevCommand: pnpm dev` → `vite` → Vite dev server 启动。**tauri.conf.json 在这里扮演"编排者的配置"**——它决定 dev 和 build 两种形态下，前端分别从哪里来。

### 为什么 dev 不直接加载文件？

因为前端源码是 `.ts`，WebView 里的浏览器不认识。Vite dev server 做的事是**即时转译**：浏览器请求哪个模块，它就现场把 TS 编译成 JS 再返回。所以 `devUrl` 指向的不是"文件"，而是"一个会动态编译的服务器"。

顺带一提：第 ② 步的 `cargo build` 也是 **debug 构建**——不做优化、编译更快、且会打开 `debug_assertions`。这就是为什么 `tauri dev` 每次启动都比 `tauri build` 快得多。

> **关键理解：** `tauri dev` 不是"一个程序"，而是"一个编排器"：启动前端服务器 → 编译 Rust → 拉起窗口 → 让窗口加载服务器的页面。练习 01 的三层模型在这里有了工具链形态——入口层负责把这三件事拼起来。

### 两种模式，前端两个来源

```
开发模式 cargo tauri dev                       生产模式 cargo tauri build
┌──────────────────────────┐                ┌──────────────────────────┐
│ 1. 先执行 beforeDevCommand│                │ 1. 先执行 beforeBuildCommand│
│    pnpm dev → Vite 服务   │                │    pnpm build → dist/    │
│    改代码即时生效（HMR）   │                │ 2. dist 静态资源嵌入二进制 │
│ 2. WebView 加载 devUrl    │                │ 3. 产出可执行文件+安装包   │
└──────────────────────────┘                └──────────────────────────┘
   前端是"活的"，来自本地端口                    前端是"死的"，长在二进制里
```

> **关键理解：** 同一个 `index.html`，两种完全不同的来源——开发时从 `devUrl`（本地端口）加载，生产时从 `frontendDist`（嵌入可执行文件的静态资源）加载。**dev 与 build 的本质区别，就是"前端从哪里来"。**

### vite.config.ts — dev server 的细节

```typescript
import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  server: {
    port: 1424,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
```

| 选项 | 值 | 作用 |
|---|---|---|
| `port` | `1424` | Vite 监听端口，**与 `devUrl` 一致**（脚手架保证，改了要两边一起改） |
| `strictPort` | `true` | 端口被占用就**报错退出**，而不是静默换端口——避免换端口后与 `devUrl` 失配 |
| `host` | `host \|\| false` | 默认只监听本机；设了 `TAURI_DEV_HOST` 环境变量（移动端真机调试）时监听所有网卡 |
| `hmr` | 仅 `TAURI_DEV_HOST` 时配置 | 移动端场景下显式指定 WebSocket 地址；桌面端用默认值即可 |
| `watch.ignored` | `["**/src-tauri/**"]` | 明确告诉 Vite：**不要监视 Rust 目录** |

`watch.ignored` 是"两个监视器"的分界线：Vite 的 watch 只管 `src/`，tauri CLI 的 watch 只管 `src-tauri/`。前端改动走 HMR（秒级），Rust 改动重新编译重启（较慢）。**HMR 是开发期专属的能力**——生产模式没有 dev server，页面是打包好的静态文件，也就没有 HMR。

> **练习的坑：** 遇到"改了没反应"先对号入座：前端不更新 → 看 vite 终端有没有输出、HMR 连接是否正常；Rust 不重编译 → 看 tauri 终端有没有重新 `cargo build` 的日志；窗口白屏 → 检查 `devUrl` 与 `vite.config.ts` 的 `server.port` 是否一致（脚手架里它们已经一致，手动改端口时最容易弄丢这层关系）。

---

## 2. AppHandle 注入 — 命令如何"认识"自己的应用

### 命令的参数，不只是数据

练习 04 你会学到命令可以接收 `String`、`i32` 等普通参数。但本练习的参数很特殊：

```rust
#[tauri::command]
fn build_info(app: tauri::AppHandle) -> BuildInfo {
    let config = app.config();
    let package = app.package_info();

    BuildInfo {
        dev_url: config.build.dev_url.as_ref().map(|u| u.to_string()),
        frontend_dist: config
            .build
            .frontend_dist
            .as_ref()
            .map(|d| d.to_string())
            .unwrap_or_default(),
        identifier: config.identifier.clone(),
        product_name: package.name.to_string(),
    }
}
```

`app: tauri::AppHandle` 不是前端传进来的数据——它是**框架自动注入的对象**。只要命令签名里声明了这个类型，Tauri 运行时就会把"当前应用实例的句柄"递进来。这就是依赖注入：**你不需要自己创建 AppHandle，只需要声明"我需要它"**。

### `config()` 与 `package_info()` 能拿到什么

| 方法 | 返回 | 本练习用到的字段 |
|---|---|---|
| `app.config()` | 应用配置（对应 tauri.conf.json） | `build.dev_url`、`build.frontend_dist`、`identifier` |
| `app.package_info()` | 应用清单（对应 Cargo.toml / package.json） | `name`（产品名） |

> **注意（练习的坑）：** `app.config()` / `app.package_info()` 是 AppHandle 的**固有方法**，直接用，不需要 `use tauri::Manager`；而 `app.get_webview_window()` 等窗口操作方法来自 `Manager` trait，需要引入（练习 06 会用到）。分不清时，先看方法的归属。

### 为什么 `dev_url` 是 `Option`？

`devUrl` 在配置里是可选的——一个纯生产应用完全可以不配 devUrl。所以它的类型是 `Option<String>`：

```rust
dev_url: config.build.dev_url.as_ref().map(|u| u.to_string()),
```

- `as_ref()` — `Option<String>` → `Option<&String>`，不移动所有权（因为 `config` 后面还要用）
- `.map(|u| u.to_string())` — 有值就转换，没有就保持 `None`

而 `frontend_dist` 用 `.unwrap_or_default()` 兜底成空字符串：**可选字段返回 `Option`，必填字段给兜底值**——这是处理配置字段的通用策略。

### `identifier` 在哪，不在哪

练习版注释里埋了个坑："identifier 在配置中，不在清单里"。的确——`PackageInfo` 没有 identifier 字段，它属于 `app.config().identifier`（反向域名风格的应用唯一标识，练习 08 还会展开）。**读配置前先想清楚：这个字段属于哪个对象。**

> **练习的坑：** 练习版把参数写成 `_app`（下划线前缀），函数体里全是占位值。不改名就用不了——`_app` 只是"我不需要它"的占位写法。把 `_app` 改成 `app` 并补全四个字段的读取，是本题的核心。

---

## 3. 前端：`Option` 字段与 null 处理

### TS 接口：`string | null`

```typescript
// 与后端 BuildInfo 对应的 TS 接口（camelCase 对应 Rust snake_case）
interface BuildInfo {
  devUrl: string | null;
  frontendDist: string;
  identifier: string;
  productName: string;
}
```

Rust 的 `Option<String>` 序列化后，在 JSON 里就是"要么字符串、要么 null"——对应 TS 的 `string | null`。**Rust 的 `Option` 与 TS 的可空类型是同一件事的两种写法。**

字段名注意：Rust 侧是 snake_case（`dev_url`），JS 侧自动使用 camelCase（`devUrl`）。Tauri 的序列化在 IPC 边界自动做这个转换（练习 05 会专门展开）。

### 渲染：`??` 空值合并

```typescript
async function render() {
  const info = await invoke<BuildInfo>("build_info");

  devUrlEl!.textContent = info.devUrl ?? "（未配置，当前为构建模式）";
  distEl!.textContent = info.frontendDist;
  identEl!.textContent = info.identifier;
  nameEl!.textContent = info.productName;
}
```

`info.devUrl ?? "（未配置，当前为构建模式）"` 是**空值合并运算符**：左边不是 `null`/`undefined` 就用左边，否则用右边。dev 模式下会显示真实地址；打包后的应用没有 devUrl，就显示"未配置"——**同一个页面，两种模式自动适配**。

> **练习流程：** 后端两个 TODO（`_app` 改名 + 补四个字段）、注册一个 TODO；前端两个 TODO（取消注释 invoke + 补四个渲染赋值）。完成后 `cargo tauri dev` 应显示 devUrl；再跑 `cargo tauri build` 并启动产物，devUrl 一栏变成"未配置，当前为构建模式"——同一份代码，两种形态，这就是本练习的直观验收。

---

## 知识点连起来看

```
┌──────────────────────────────────────────────────┐
│ 开发形态 (tauri dev)         生产形态 (tauri build)│
│                                                │
│  beforeDevCommand: pnpm dev   beforeBuildCommand│
│       │                            : pnpm build │
│       ▼                            ▼             │
│  Vite dev server            vite build → dist/   │
│  （端口 1424，即时转译）        （静态文件）       │
│       │                            │             │
│  devUrl: localhost:1424     frontendDist: ../dist│
│       │                            │             │
│  WebView 加载动态页面        WebView 加载打包产物  │
│  + HMR（WebSocket 推送）    （无 HMR）            │
└──────────────────────────────────────────────────┘
```

| 层 | 本课回答的问题 | 关键概念 |
|---|---|---|
| 编排层 | 一条命令如何驱动整套流程 | beforeDevCommand、devUrl、frontendDist |
| 注入层 | 命令怎么拿到应用信息 | AppHandle 依赖注入、config() / package_info() |
| 类型层 | 可选配置怎么跨进程 | Option ↔ string \| null、?? 空值合并 |

**一通百通的核心：** dev 和 build 是同一套代码的**两种装配形态**——dev 形态用 dev server 动态喂页面（换来 HMR），build 形态把前端打包成静态文件交给应用（换来体积与分发）。`tauri.conf.json` 的 `build` 段就是这两套装配的开关。而 `AppHandle` 注入说明命令层不是"孤岛"：**框架对象可以按需注入命令**，这为后续练习（状态管理、事件、窗口操作）打开了大门。

**递进关系：** 练习 04 将走通"命令定义 → 注册 → invoke 传参 → 错误处理"的完整链路——`AppHandle` 换成普通参数，就是你的第一个真正可交互的命令。