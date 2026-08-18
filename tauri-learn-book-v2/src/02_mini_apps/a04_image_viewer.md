# 练习 A04: 图片查看器

## 为什么要学这个

前几课都是"点按钮"。真实的桌面应用还依赖一些"桌面特有的交互"：**把文件拖进窗口**、**直接看本地文件**、**操作窗口本身**。这一章要回答三个问题：

1. **怎么把拖进来的文件交给应用？** —— 拖放事件（`onDragDropEvent`）拿到的到底是什么？
2. **webview 怎么显示本地图片？** —— 为什么不能直接写 `<img src="C:\...">`？asset 协议是什么？
3. **怎么直接操控窗口？** —— 缩放、置顶、全屏，前端一行代码能搞定？

学完你会：做出一个"拖进去就能看"的图片查看器——这是纯网页永远做不到的**桌面感**。

---

## 从问题出发

练习 A04 要做的事：**把图片（或整个图片文件夹）拖进窗口，就能一张张翻看，还能放大、缩小、置顶、全屏。**

**核心矛盾：** WebView 本质上是个浏览器。浏览器的两个"原生限制"挡住了路：

1. **拿不到文件路径**——浏览器拖文件进来只给文件内容副本，不给路径；而桌面应用需要路径才能继续操作（本例是显示、列目录）。
2. **加载不了本地绝对路径**——`<img src="C:/Users/.../a.png">` 在网页里会被当成 URL，浏览器出于安全禁止访问本机任意路径。

Tauri 的解法正好对应这两点：

```text
拖文件 → onDragDropEvent 拿到"真实路径"（桌面专有 API）
显示本地图 → asset 协议：把本地路径包装成合法的 app 内 URL
操控窗口 → getCurrentWindow() 提供 setSize / setAlwaysOnTop / setFullscreen
```

```text
前端 (React)                            Rust / 系统
┌────────────────────────────┐          ┌──────────────────────────┐
│ onDragDropEvent 拖放        │ ─路径──► │ (文件夹) list_images 命令 │
│ convertFileSrc 本地图片URL  │ ◄──────  │ asset 协议映射本地文件    │
│ getCurrentWindow 窗口操作   │ ──────►  │ 缩放/置顶/全屏/居中       │
└────────────────────────────┘          └──────────────────────────┘
```

---

<a id="sec-a04-drag"></a>
## 1. 拖放 — 拿到文件的真实路径

浏览器能拖文件，但只给内容不给路径。Tauri 的前端 API `onDragDropEvent` 直接把**真实路径**给你：

```typescript
import { getCurrentWebview } from "@tauri-apps/api/webview";

getCurrentWebview().onDragDropEvent((event) => {
  if (event.payload.type !== "drop") return;  // over/leave 阶段忽略
  const paths = event.payload.paths;          // 拖入的所有路径（文件或文件夹）
});
```

- 拖放过程有三个阶段：`over`（拖进窗口）、`drop`（松手放下）、`leave`（拖出去）。我们只关心 `drop`
- `event.payload.paths`：**文件系统里的绝对路径**（文件夹也可以是拖入项）
- 拿到路径后做什么都行——本课：图片直接收下，文件夹交给后端 `list_images` 列目录

> **为什么这很重要：** 这是"浏览器做不到、桌面才能做"的分水岭。有了路径，才能继续走命令、读文件、列目录。

<a id="sec-a04-asset"></a>
## 2. 静态资源 — 用 asset 协议显示本地图片

### 为什么 `C:\...` 的路径不能直接当 src

WebView 加载资源时，`src` 会被解析成 URL（`http://`、`asset://`……）。把 `C:/Users/...` 塞进去，要么被当成本机 URL 被安全策略拦下，要么根本解析不了。

### asset 协议：本地路径 → 应用内 URL

三步：

```json
// ① tauri.conf.json 开启 asset 协议并允许读取（练习版已配好）
"security": {
  "assetProtocol": { "enable": true, "scope": ["**"] }
}
```

```toml
// ② 开启 asset 协议对应的 cargo feature（练习版已配好）
[dependencies]
tauri = { workspace = true, features = ["protocol-asset"] }
```

```typescript
// ③ 前端用 convertFileSrc 把路径转成 URL
import { convertFileSrc } from "@tauri-apps/api/core";

const url = convertFileSrc("C:/Users/.../a.png"); // → asset://localhost/<编码后的路径>
<img src={url} />
```

> 少了 `protocol-asset` feature，`tauri build` 的构建脚本会直接报错："dependency features do not match the allowlist"。这是 asset 协议最典型的配置坑。

- `convertFileSrc` 负责：路径 → URL、特殊字符编码
- 加载时 Tauri 检查路径是否在 `assetProtocol.scope` 白名单里——本课用 `["**"]` 全部放行（教学），真实项目要收窄
- **CSP 注意**：页面加载 `asset:` 资源，需要 `img-src` 允许 asset 协议（本课 csp 是 null，不受限）

> **关键理解：** asset 协议是"安全的本地文件服务器"——它让 WebView 用标准的 `<img>` 加载本地文件，但路径必须过 scope 白名单。生产环境用 `asset:` 而不是把图片读成 base64，性能和内存都好得多。

<a id="sec-a04-window"></a>
## 3. 窗口操作 — 缩放、置顶、全屏

前端一行代码就能操控窗口：

```typescript
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";

const win = getCurrentWindow();
await win.setSize(new LogicalSize(1280, 800));   // 设置大小
await win.setAlwaysOnTop(true);                  // 置顶
await win.setFullscreen(true);                   // 全屏
await win.center();                              // 居中
```

- `getCurrentWindow()`：代表"当前所在的这个窗口"
- `LogicalSize` vs `PhysicalSize`：逻辑尺寸（和 DPI 无关的抽象单位）vs 物理像素。前端一般用逻辑尺寸
- 本课"放大/缩小"：先 `win.innerSize()` 拿当前尺寸，乘上比例再 `setSize`

### 窗口操作要权限

这些前端 API 背后是对应的后端命令，而它们**不属于 `core:default`**——`core:default` 只包含读取窗口信息的权限。所以 capabilities 要显式加（练习版已配好）：

```json
"core:window:allow-set-size",
"core:window:allow-set-always-on-top",
"core:window:allow-set-fullscreen",
"core:window:allow-center",
```

> **对比：** 自己写的 `#[tauri::command]` 不需要权限；**前端调 Tauri 内置能力（窗口、插件）需要权限**。这是 A03 通知插件和本课窗口操作的共同点。

<a id="sec-a04-list"></a>
## 4. 列目录 — 拖文件夹进来

拖入文件夹时，路径不是图片——本课把它当作目录，交给后端列出其中的图片（沿用 A02 的 std::fs，但这次是 `read_dir`）：

```rust
use std::path::Path;

fn is_image(path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase());
    matches!(ext.as_deref(), Some("png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp"))
}

#[tauri::command]
fn list_images(dir: String) -> Result<Vec<String>, String> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(&dir).map_err(|e| format!("打开目录失败：{e}"))? {
        let entry = entry.map_err(|e| format!("读取目录项失败：{e}"))?;
        let path = entry.path();
        if path.is_file() && is_image(&path) {
            paths.push(path.to_string_lossy().into_owned());
        }
    }
    paths.sort(); // 文件名排序，翻页顺序稳定
    Ok(paths)
}
```

- `std::fs::read_dir`：遍历目录（`A02` 用 `open` 读写文件，这里换成 `read_dir` 列目录——同一套 std::fs）
- `path.extension()` → `&OsStr` → `to_str()` → `to_ascii_lowercase()`：处理大小写（`.PNG` 也要认）
- 前端约定：拖入的非图片路径就当目录试一下

---

## 练习指引

**作业范围：** 动 2 个文件，共 11 处 TODO。两个配置文件（`tauri.conf.json` 的 `assetProtocol`、capabilities 的窗口权限）已配好，**不用动**。

| 文件 | 步骤 | 内容 |
|------|------|------|
| `src-tauri/src/lib.rs` | 1 | `is_image`：扩展名判断 |
| `src-tauri/src/lib.rs` | 2 | `list_images`：遍历目录收图片 |
| `src-tauri/src/lib.rs` | 3 | 排序后返回 |
| `src-tauri/src/lib.rs` | 4 | 登记命令 |
| `src/App.tsx` | 1 | 导入 `invoke` |
| `src/App.tsx` | 2 | 拖放：非图片路径当目录，调 `list_images` |
| `src/App.tsx` | 3-4 | 置顶 / 全屏按钮调窗口 API |

**怎么验证：**

```bash
cd 02_mini_apps/a04_image_viewer
cargo tauri dev
```

拖几张图片进窗口 → 主区域显示第一张，底部出现缩略图条，点"上一张/下一张"翻页。拖一个**文件夹**进去 → 自动列出里面所有图片。点"放大/缩小"窗口变大变小，"置顶"后窗口始终在最前，"全屏"占满屏幕，"居中"回到屏幕中间。

**故意踩坑看效果：** 把 `tauri.conf.json` 的 `assetProtocol` 关掉 → 图片显示为空白（验证 asset 协议的必要性）；把 `convertFileSrc` 换成直接 `src={path}` → 图片加载失败（验证路径必须转 URL）。

---

## 知识点连起来看

```text
onDragDropEvent                  ← 桌面交互：拖放 → 真实路径
        │
图片路径 → convertFileSrc        ← 静态资源：asset 协议映射本地文件
        │
文件夹路径 → invoke list_images  ← 命令 + std::fs：列目录过滤图片
        │
getCurrentWindow().setSize / 置顶 / 全屏 ← 窗口操作（需权限）
```

| 层 | 解决的问题 | 关键概念 |
|----|-----------|---------|
| 拖放 | 怎么拿外部文件路径 | `onDragDropEvent`、`paths` |
| 静态资源 | 本地图片怎么显示 | `assetProtocol`、`convertFileSrc`、scope |
| 命令 | 文件夹怎么展开 | `read_dir`、`is_image`、排序 |
| 窗口 | 怎么操控窗口 | `getCurrentWindow`、`LogicalSize`、权限 |

**一通百通的核心：** 这一课补上了"**桌面感**"。前面学的是"应用内部逻辑"（状态、文件、后台任务），这课是"应用与桌面系统交互"（拖放、本地资源、窗口）。这两半合起来，才是一个完整的桌面应用。

**递进关系：** 练习 A05（记账本）将引入"数据多了怎么办"——SQL 数据库。届时你会看到：命令、状态、错误处理这些套路原封不动，只是存储引擎从 `Vec` 换成了数据库。

---

> **本课配置小结（练习版已配好，无需改动）：**
> - `tauri.conf.json`：`app.security.assetProtocol = { enable: true, scope: ["**"] }`
> - `src-tauri/Cargo.toml`：`tauri = { workspace = true, features = ["protocol-asset"] }`
> - `capabilities/default.json`：`core:window:allow-set-size` / `allow-set-always-on-top` / `allow-set-fullscreen` / `allow-center` 等
