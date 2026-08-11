# 练习 08: 打包与图标

## 为什么要学这个

这是入门模块的收尾。前面七课你都在 `cargo tauri dev` 下开发——但软件最终要交给别人用，靠的是**打包**。这一课回答三个问题：

1. **`tauri build` 到底产出什么？** — 安装包从哪来？`bundle` 配置段管什么？
2. **应用的"身份三件套"是什么？** — `identifier`、`productName`、`version` 各自在哪配置、运行时从哪读？
3. **图标从哪来？** — 一套图标文件（`.ico`、`.png`、各种尺寸）是手画的吗？一条命令能生成多少？

理解这三件事，你就走完了"写代码 → 交付"的完整链路——模块 01 的终点。

---

## 从问题出发

练习 08 要做的事：**读取应用的打包元数据（identifier、productName、version、图标清单），显示在窗口里**。

开发时你关心的东西（日志、HMR、DevTools）在交付时统统不重要了，用户看到的是：**安装包、图标、应用名**。这些信息在 `tauri.conf.json` 里统一定义，运行时通过 `AppHandle` 读取（练习 03 的机制）：

```
tauri.conf.json / Cargo.toml（配置时）
        │ 编译期读取 / 运行时暴露
        ▼
app.config().identifier       ← 应用唯一标识
app.package_info().name       ← 产品名
app.package_info().version    ← 版本号
icons/ 下的图标文件            ← 应用形象
        │
        ▼
tauri build → 安装包（用户拿到的东西）
```

**核心矛盾：** 配置是"开发时写一次"，打包是"交付时用一次"，运行时读取是"两者之间的桥"——本练习的命令就是这座桥：**把配置读出来、展示给用户**，顺便让你看清"交付物里到底有什么"。

---

## 1. 打包 — `tauri build` 与 bundle 段

### tauri.conf.json 的 bundle 段

```json
"bundle": {
  "active": true,
  "targets": "all",
  "icon": ["icons/icon.ico", "icons/icon.png", "icons/32x32.png", "icons/128x128.png", "icons/128x128@2x.png"]
}
```

| 字段 | 值 | 作用 |
|---|---|---|
| `active` | `true` | 是否生成安装包（false 则只产出可执行文件） |
| `targets` | `"all"` | 打包哪些格式（Windows 上是 MSI + NSIS） |
| `icon` | 5 个图标文件 | 打包时嵌入的图标集（安装包图标、可执行文件图标、任务栏图标） |

### `cargo tauri build` 的三段流水线

```bash
cd 01_getting_started/e08_packaging_and_icons
cargo tauri build
```

```
1. beforeBuildCommand: pnpm build
   └─ vite build ──► dist/（前端静态资源：index.html + JS/CSS）
2. cargo build --release
   └─ 编译 Rust；同时 generate_context!() 把 dist/ 嵌入二进制
3. 打包（bundle 段配置生效）
   └─ 生成可执行文件与平台安装包
```

在 Windows 上，产物位于 `src-tauri/target/release/`：

```
src-tauri/target/release/
├── e08-packaging_and_icons.exe   ← 主可执行文件（双击即可运行）
└── bundle/
    ├── msi/…                     ← MSI 安装包
    └── nsis/…                    ← NSIS 安装包
```

> **关键理解：** `tauri dev` 和 `tauri build` 差的不只是 `--release`。dev 模式的前端来自网络端口（活的），build 模式的前端来自 `frontendDist`（被嵌入二进制的静态文件）——**一个进程内嵌了一个完整网站**，这就是 Tauri 应用能单文件分发的原因（练习 03 的两种装配形态在这里兑现）。

### 可执行文件 vs 安装包

| | 可执行文件（.exe） | 安装包（.msi / .nsis） |
|---|---|---|
| 给谁用 | 自己测试、分发单文件 | 最终用户 |
| 安装 | 不需要 | 双击安装、开始菜单、卸载干净 |
| 命名来源 | `productName` | `productName` |

注意可执行文件的名字来自 `tauri.conf.json` 的 `productName`（`e08-packaging_and_icons`），不是 Cargo.toml 的包名——**打包世界的名字由配置决定**。

---

## 2. 身份三件套 — identifier、productName、version

### 答案版命令

```rust
#[tauri::command]
fn bundle_info(app: tauri::AppHandle) -> BundleInfo {
    let config = app.config();
    let package = app.package_info();

    BundleInfo {
        identifier: config.identifier.clone(),
        product_name: package.name.to_string(),
        version: package.version.to_string(),
        icon_files: vec![
            "icons/icon.ico          # Windows 可执行文件嵌入图标".into(),
            "icons/icon.png          # 通用 256×256 图标".into(),
            "icons/32x32.png         # 小尺寸图标".into(),
            "icons/128x128.png       # 中尺寸图标".into(),
            "icons/128x128@2x.png    # 高分屏图标".into(),
        ],
    }
}
```

### 三件套各是什么

| 身份 | 配置在哪 | 运行时从哪读 | 作用 |
|---|---|---|---|
| `identifier` | `tauri.conf.json`（顶层） | `app.config().identifier` | 应用唯一标识（反向域名格式），**安装/更新时识别身份** |
| `productName` | `tauri.conf.json`（顶层） | `app.package_info().name` | 显示名：安装包、可执行文件、开始菜单 |
| `version` | `tauri.conf.json` / Cargo.toml | `app.package_info().version` | 版本号，更新时比较新旧版本 |

**为什么 identifier 和 productName 的读取位置不同？** 这是 Tauri 的内部约定：

- `identifier` 属于**应用配置**（`config`）——它定义"这个应用是谁"
- `name` / `version` 属于**应用清单**（`package_info`）——它们描述"这个应用的信息"

> **练习的坑（已踩过）：** `PackageInfo` 没有 identifier 字段！identifier 只在 `app.config().identifier`。如果你按惯性去 `package_info()` 里找它，编译直接报错——练习版注释专门标了这一条：**"identifier 在配置中，不在清单里"**。

### identifier 的规范与红线

`com.taurilearn.e08` 是**反向域名格式**（倒过来的域名 + 应用名）。它的两条红线：

1. **全局唯一** — 两个应用的 identifier 相同，系统会当成同一个应用（更新、数据目录都会错乱）
2. **一旦发布，不可更改** — 改了 identifier，用户机器上会被识别成"另一个应用"，之前的更新链和数据全部断掉

练习版与答案版用不同的 identifier（`com.taurilearn.e08` vs `com.taurilearn.e08a`），正是为了两个项目能共存——这也是为什么每个练习的 identifier 都不同。

---

## 3. 图标 — 一条命令生成全套

### 图标清单读的是什么

```rust
icon_files: vec![
    "icons/icon.ico          # Windows 可执行文件嵌入图标".into(),
    "icons/icon.png          # 通用 256×256 图标".into(),
    "icons/32x32.png         # 小尺寸图标".into(),
    "icons/128x128.png       # 中尺寸图标".into(),
    "icons/128x128@2x.png    # 高分屏图标".into(),
],
```

| 文件 | 用在哪 |
|---|---|
| `icon.ico` | Windows 可执行文件嵌入图标（任务栏、资源管理器） |
| `icon.png` | 通用 256×256 主图标 |
| `32x32.png` | 小尺寸（窗口、托盘） |
| `128x128.png` | 中尺寸（商店、开始菜单） |
| `128x128@2x.png` | 高分屏（2x DPI） |

### 图标不是手画的：`cargo tauri icon`

你只需要**一张 1024×1024 的源图**，然后一条命令：

```bash
cargo tauri icon ./app-icon.png
```

Tauri CLI 会自动生成全套图标并写入 `icons/` 目录：

```
输入                         输出
┌──────────────┐   cargo    ┌────────────────────────────┐
│ app-icon.png │  tauri    │ icon.ico / icon.png /      │
│ (1024×1024)  │   icon    │ 32x32 / 128x128 / @2x ...  │
└──────────────┘           └────────────────────────────┘
```

> **关键理解：** 各平台对图标的格式要求不同（Windows 要 `.ico`、macOS 要 `.icns`、Linux 要 PNG 系列），手工逐尺寸制作不现实。`cargo tauri icon` 把"源图 → 全套图标"的转换自动化了——**你只管一张图，工具管所有平台**。

### 前端：四行信息 + 图标列表

```typescript
async function render() {
  const info = await invoke<BundleInfo>("bundle_info");

  identEl!.textContent = info.identifier;
  nameEl!.textContent = info.productName;
  versionEl!.textContent = info.version;
  iconListEl!.innerHTML = info.iconFiles
    .map((f) => `<li><code>${f}</code></li>`)
    .join("");
}
```

`iconFiles` 是 `string[]`（练习 02 的 `Vec<String>` 渲染模式），每条图标说明渲染成一个 `<li><code>`。四个字段、一个列表，全部来自同一个 `invoke`——**一次调用，把应用的"身份信息"一次取齐**。

> **练习流程：** 后端两个 TODO（`_app` 改名 + 补四个字段）、注册一个 TODO；前端两个 TODO（invoke + 渲染）。完成后 `cargo tauri dev` 应显示 `com.taurilearn.e08`、`e08-packaging_and_icons`、`0.1.0` 和 5 行图标清单。

---

## 知识点连起来看

```
┌────────────────────────────────────────────────┐
│ 配置层  tauri.conf.json                        │
│         identifier / productName / bundle.icon │ ← 写一次
│                    │                           │
│ 运行时  app.config() / app.package_info()      │ ← 读出来
│                    │                           │
│ 工具层  cargo tauri build / cargo tauri icon   │ ← 用起来
│         └─ 安装包、可执行文件、全套图标         │
│                    │                           │
│ 交付层  用户拿到：安装包 + 图标 + 应用名        │
└────────────────────────────────────────────────┘
```

| 层 | 本课回答的问题 | 关键概念 |
|---|---|---|
| 配置层 | 身份在哪定义 | identifier / productName / bundle 段 |
| 运行时层 | 身份从哪读 | `config` vs `package_info` 的分工 |
| 工具层 | 产物从哪来 | `tauri build` 三段流水线、`tauri icon` |
| 交付层 | 用户拿到什么 | 安装包、可执行文件、图标 |

**一通百通的核心：** 打包是"把开发时的所有临时性剥掉，留下可交付的稳定形态"——配置定义身份（写一次），运行时读取身份（读出来），工具链生成产物（用起来）。`identifier` 是红线（不可改）、`productName` 是门面、图标是形象，**三者共同构成用户在系统里"看见的那个应用"**。

---

## 模块小结：入门四件事

回头看模块 01 的八个练习，它们恰好构成 Tauri 开发的四件事：

| 事 | 练习 | 核心收获 |
|---|---|---|
| 环境与骨架 | 01 环境准备、02 项目结构 | 双进程架构、两层目录、三层模型 |
| 跑起来 | 03 运行与构建 | dev/build 两种装配、devUrl 与 frontendDist |
| 命令与数据 | 04 第一个命令、05 参数与返回值 | 命令全链路、序列化契约、类型映射 |
| 窗口与交付 | 06 窗口配置、07 调试、08 打包图标 | 窗口两条路径、双通道日志、身份三件套 |

模块 02（命令）将在这套地基上展开更复杂的能力：异步命令、状态管理、事件系统、窗口操作——那时你会频繁回到这三层结构和两条日志通道上来。