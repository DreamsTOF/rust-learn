# 练习 06: 窗口配置

## 为什么要学这个

前五课你的应用始终只有"一个窗口"——`tauri.conf.json` 里配出来的那个。真实应用不一样：设置面板、关于弹窗、帮助页，都是**额外窗口**。这一课回答三个问题：

1. **主窗口和动态窗口有什么区别？** — `tauri.conf.json` 里配置的窗口，和代码里创建的窗口，两条路各自怎么走？
2. **命令怎么"操作"应用？** — 练习 05 的命令只返回数据，这一课的命令要真的创建一个窗口——`Result<(), String>` 是什么？
3. **两个窗口共用一份前端代码，怎么区分？** — 同一个 `index.html`，如何知道"我是主窗口还是关于窗口"？

回答完这三个问题，你就理解了 Tauri 窗口系统的两条创建路径，以及命令"产生副作用"的写法。

---

## 从问题出发

练习 06 要做的事：**主窗口里点一个按钮，弹出一个"关于"子窗口（标题、尺寸、居中、禁止缩放都是动态配置的）**。

窗口从哪来？Tauri 里有两条路：

```
路径 A：声明式（配置文件）
  tauri.conf.json → app.windows[0]
  → 应用启动时自动创建，改配置重新编译生效

路径 B：命令式（运行时代码）
  WebviewWindowBuilder::new(...).build()
  → 运行时动态创建，改代码重新编译生效
```

**核心矛盾：** 主窗口是"出生时就有的"——启动配置里写好了；关于窗口是"用户点按钮才有的"——必须在运行时由命令创建。所以本练习同时用了两种机制：主窗口走配置（练习 02 看过 `app.windows` 段），关于窗口走 `WebviewWindowBuilder`。

---

## 1. 两条创建路径 — 配置窗口 vs 动态窗口

### 路径 A：tauri.conf.json 里的主窗口

```json
"app": {
  "windows": [
    {
      "title": "练习 E06: 窗口配置",
      "width": 800,
      "height": 600
    }
  ]
}
```

练习 01 讲过：`generate_context!()` 在**编译期**读取这份配置，应用启动时按它创建主窗口。改标题、尺寸，不用动代码，重新编译即可。

> **关键理解：** `app.windows` 是数组——可以声明多个窗口。但数组里的窗口都是"应用启动时全量创建"的，无法表达"用户点了按钮才弹窗"这种动态需求。动态需求必须走路径 B。

### 路径 B：WebviewWindowBuilder 动态建窗

```rust
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

WebviewWindowBuilder::new(&app, "about", WebviewUrl::App("index.html".into()))
    .title("关于本应用")
    .inner_size(420.0, 300.0)
    .center()
    .resizable(false)
    .build()
    .map_err(|e| e.to_string())?;
```

逐段拆解：

| 代码 | 作用 |
|---|---|
| `WebviewWindowBuilder::new(&app, "about", ...)` | 创建构建器：`app` 是应用句柄，`"about"` 是**窗口 label**（唯一标识），第三个参数是**加载什么内容** |
| `WebviewUrl::App("index.html".into())` | 加载打包进应用的前端页面（相对路径）；另一选项 `WebviewUrl::External(url)` 加载外部网站 |
| `.title(...)` / `.inner_size(420.0, 300.0)` | 标题与尺寸（`inner_size` 是内容区尺寸，不含边框） |
| `.center()` / `.resizable(false)` | 居中显示、禁止缩放 |
| `.build()` | 真正创建窗口，返回 `Result<WebviewWindow, Error>` |
| `.map_err(|e| e.to_string())?` | 错误转换 + 提前返回（下一节详讲） |

**构建器模式**（练习 03 的 `Command` 也是这个模式）：每一步返回构建器本身，可以链式写下去，直到 `.build()` 才真正执行。**配置与执行分离**——先声明"窗口长什么样"，最后一次性创建。

> **练习的坑：** 练习版把 `WebviewUrl` / `WebviewWindowBuilder` 的 `use` 和整个构建链都挖掉了，注释里有完整提示。照抄时最容易漏的是 `use tauri::{WebviewUrl, WebviewWindowBuilder};`——`Manager` 在文件顶部已有，但另外两个类型需要自己引入。

### 为什么 label 是 "about"？

`"about"` 是窗口的**唯一标识**——不是标题，是逻辑名。它的两个用途：

1. 前端用 `getCurrentWindow().label` 判断"我是谁"（第 3 节）
2. 后端用 `app.get_webview_window("about")` 查找窗口（第 2 节）

---

## 2. 命令的副作用 — 查找、聚焦、创建

### 完整命令

```rust
/// 创建「关于」子窗口：标题 / 尺寸 / 居中 / 禁止缩放。
/// 已存在同名窗口时直接聚焦，避免重复创建。
#[tauri::command]
fn open_about_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("about") {
        win.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(&app, "about", WebviewUrl::App("index.html".into()))
        .title("关于本应用")
        .inner_size(420.0, 300.0)
        .center()
        .resizable(false)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

### `get_webview_window` — 查重

```rust
if let Some(win) = app.get_webview_window("about") {
    win.set_focus().map_err(|e| e.to_string())?;
    return Ok(());
}
```

`app.get_webview_window("about")` 返回 `Option<WebviewWindow>`：**窗口存在就返回它，不存在返回 `None`**。这里用它做"查重"——用户狂点按钮，也不会创建出十个关于窗口，只会把已存在的那个聚焦到前台。

> **注意（练习的坑）：** `get_webview_window` 来自 `Manager` trait——`use tauri::Manager;` 必须引入（练习版已留好）。对比练习 03 的 `app.config()`（固有方法，不需要 trait），**窗口操作方法都需要 `Manager`**。分不清就看文档：trait 方法必须 `use` 才能调用。

### `Result<(), String>` — 有副作用的命令签名

练习 05 的命令返回数据（`-> Summary`）。本练习的命令**做的事**是创建窗口——成功时没有数据要返回，失败时却需要告诉前端"为什么没成功"。所以返回类型是：

```rust
-> Result<(), String>
//       ^^  ^^^^^^
//       成功：没有值（空元组）   失败：错误信息
```

- `Ok(())` — 成功。`()` 是空元组，"没有数据"的类型化表达
- `Err(e.to_string())` — 失败，携带可读的错误信息
- `?` — 错误传播运算符：`Err` 就提前返回，`Ok` 就取出值继续

`.map_err(|e| e.to_string())?` 是"错误转换 + 传播"的惯用组合——`.build()` 返回 `Result<_, tauri::Error>`，`map_err` 转成 `String`，`?` 提前返回。**前端拿到 `Err` 时，`invoke` 的 Promise 会 reject**，被练习 04 学过的 try/catch 捕获。

> **关键理解：** 返回值类型反映命令的性质——**纯查询返回数据，有副作用的命令返回 `Result<(), Error>`**。成功时前端不需要数据，失败时前端需要原因。这是 Tauri 命令的两种标准形态。

---

## 3. 两个窗口，一份前端代码

### `getCurrentWindow().label` — 前端怎么知道"我是谁"

关于窗口和主窗口加载的是**同一个 `index.html`、同一份 `main.ts`**。那代码怎么区分？用窗口的 label：

```typescript
import { getCurrentWindow } from "@tauri-apps/api/window";

// 根据窗口 label 渲染不同内容：主窗口 vs 关于窗口
const label = getCurrentWindow().label;
const isAbout = label === "about";

if (isAbout) {
  // 关于子窗口：展示本窗口的动态配置
  contentEl!.innerHTML = `...`;
} else {
  // 主窗口：按钮触发创建子窗口
  openBtn!.addEventListener("click", () => {
    invoke("open_about_window").catch((e) => console.error(e));
  });
}
```

`getCurrentWindow()` 返回"当前窗口"的句柄，`.label` 就是创建时给的 label。**label 在这里充当了"路由"**——同一个 bundle，根据窗口身份渲染不同内容。

### 主窗口分支：按钮触发命令

```typescript
openBtn!.addEventListener("click", () => {
  invoke("open_about_window").catch((e) => console.error(e));
});
```

注意这里**没有 `await`、没有 `try/catch` 的 async 函数包裹**——因为命令没有返回值需要处理，只需要在失败时打日志：`invoke(...).catch(...)` 就够。**"有数据要展示才 await，没数据就只管失败日志"**——调用形态跟着返回类型走。

### 关于窗口分支：展示动态配置

```typescript
contentEl!.innerHTML = `
  <h2>关于本应用</h2>
  <p>本窗口由后端命令动态创建，配置如下：</p>
  <ul>
    <li><code>title</code>: 关于本应用</li>
    <li><code>inner_size</code>: 420 × 300</li>
    <li><code>center</code>: true</li>
    <li><code>resizable</code>: false</li>
  </ul>
  <p class="sub">主窗口的 title / width / height 则在 <code>tauri.conf.json</code> 中配置。</p>
`;
```

这段 HTML 里的四个 `<code>` 值，就是后端 `WebviewWindowBuilder` 链上配置的四个属性——**"命令里配置的"和"页面里展示的"是同一份真相**。而副文本点出了两条路径的分工：主窗口走配置文件，关于窗口走运行时代码。

---

## 知识点连起来看

```
┌──────────────────────────────────────────────┐
│ 路径 A：声明式（编译期）                       │
│  tauri.conf.json app.windows[0]              │ ← 主窗口：启动即有
│      │ generate_context!() 编译期读取         │
│      ▼                                       │
│  窗口系统（WebviewWindow 注册表）              │
│      ▲                                       │
│ 路径 B：命令式（运行时）                       │
│  WebviewWindowBuilder.new(&app, "about", ...)│ ← 关于窗口：点击才有
│      .build()                                │
│                                              │
│  get_webview_window("about")  ← 按 label 查   │
│  getCurrentWindow().label     ← 前端自报身份  │
└──────────────────────────────────────────────┘
```

| 层 | 本课回答的问题 | 关键概念 |
|---|---|---|
| 配置层 | 主窗口从哪来 | `app.windows` 数组、编译期读取 |
| 命令层 | 动态窗口怎么建 | `WebviewWindowBuilder` 构建器链 |
| 副作用层 | 命令怎么报告成败 | `Result<(), String>`、`?`、`map_err` |
| 身份层 | 前端怎么区分窗口 | `getCurrentWindow().label`、Manager trait |

**一通百通的核心：** 窗口有两条创建路径——**配置文件管"出生就有的"，构建器管"运行时才要的"**；而 `label` 是窗口世界的身份证——后端靠它查窗口、前端靠它认自己。命令第一次产生了真实副作用（创建窗口），`Result<(), String>` 就是"有副作用命令"的标准签名。这套模式后面会在托盘、多窗口、菜单里反复出现。

**递进关系：** 练习 07 将从"怎么建窗口"转向"怎么找问题"——前端 DevTools 与后端日志，两条调试通道各自怎么用。