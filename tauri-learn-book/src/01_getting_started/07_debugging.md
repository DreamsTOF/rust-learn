# 练习 07: 调试

## 为什么要学这个

前六课你都在写"正确的代码"。但真实开发里，一半时间花在**找问题**上：前端不更新、后端报错、数据对不上。这一课回答三个问题：

1. **前端怎么调试？** — 页面里的 JS 报错、变量值、DOM 状态，在哪看？
2. **后端怎么调试？** — Rust 代码在终端里怎么输出日志？`println!` 和 `eprintln!` 有什么区别？
3. **前后端怎么对照？** — 一次调用，前端打一条日志、后端打一条日志，怎么把两条日志"对起来"？

调试不是"出问题才学"的技能——**先建立两条日志通道的直觉，遇到问题才知道去哪看**。

---

## 从问题出发

练习 07 要做的事：**输入一条消息，点击按钮，后端打印日志、计算时间戳，把三行结果返回前端展示；同时前后端各自在控制台输出日志**。

```
前端 (WebView)                        Rust 核心进程
┌─────────────────────┐    IPC     ┌──────────────────────────┐
│ console.log("发送:") │ ─────────► │ println!("收到前端消息")   │
│ 输入框 → invoke      │  {message} │ eprintln!("stderr 示例")  │
│ console.log("返回:") │ ◄───────── │ 计算时间戳                │
│ DevTools 里看        │  3 行结果  │ 运行终端里看             │
└─────────────────────┘            └──────────────────────────┘
```

**核心矛盾：** 应用有两个"大脑"——WebView 里的 JS 和 Rust 核心进程。它们的日志去往**不同的地方**：前端日志进 DevTools 的 Console，后端日志进运行 `cargo tauri dev` 的终端。**调试的第一步，是知道每条日志该去哪看。**

---

## 1. 前端调试 — DevTools（Web Inspector）

### 怎么打开

dev 模式下（`cargo tauri dev`），窗口里的 WebView 是**可检查**的：

- Windows/Linux：`F12`，或右键页面 → **检查**
- macOS：`Cmd + Option + I`（或右键 → 检查元素）

打开后就是一套标准的浏览器开发者工具：Elements（DOM）、Console（日志）、Sources（源码）、Network（网络）。**你熟悉 Chrome DevTools 的话，这里零成本迁移。**

> **关键理解：** DevTools 只在 **dev（debug）构建**下可用——release 打包的应用不启用 devtools（体积与安全考虑）。所以"生产环境出问题没法看 Console"是常态，要靠后端日志（第 2 节）和错误兜底（练习 04 的 catch）。

### console.log — 前端日志的基本形态

```typescript
// 前端日志：DevTools Console 可见（dev 模式按 F12 或右键 → 检查）
console.log("[frontend] 页面加载完成，等待触发调试命令");

btn!.addEventListener("click", async () => {
  const message = input!.value.trim() || "hello tauri";
  console.log("[frontend] 发送给后端:", message);

  try {
    const lines = await invoke<string[]>("run_debug_trace", { message });
    console.log("[frontend] 后端返回:", lines);
    outputEl!.textContent = lines.join("\n");
  } catch (e) {
    console.error("[frontend] 调用失败:", e);
    outputEl!.textContent = `调用失败: ${e}`;
  }
});
```

三个细节：

- **`console.log` 可以传多个值** — `console.log("[frontend] 发送给后端:", message)` 把标签和值分开传，Console 里可折叠展开对象
- **`console.error` 与 `console.log` 分开用** — 错误日志走 `console.error`（Console 里红色显示、可筛选），普通信息走 `console.log`。**用前缀 `[frontend]` 统一标识来源**，日志一多就能过滤
- **日志放在"动作发生点"** — 发送前、返回后、失败时各打一条，就能还原一次调用的完整时间线

---

## 2. 后端调试 — println! 与运行终端

### 后端日志去哪里看

**运行 `cargo tauri dev` 的终端**——不是页面，不是 DevTools。Rust 的日志走标准输出/标准错误，而 `tauri dev` 把子进程的输出转发到自己的终端。所以：

```
cargo tauri dev   ← 在这个终端里看 println! 输出
```

> **练习的坑：** 新手最常犯的错是"在后端写了 println!，去 DevTools 找输出"——找不到，因为两个输出流目的地不同。**前端日志进 DevTools，后端日志进终端**，这是 Tauri 双进程架构的调试铁律。

### println! 与 eprintln!

```rust
#[tauri::command]
fn run_debug_trace(message: String) -> Vec<String> {
    println!("[debug] 收到前端消息: {message}");
    eprintln!("[debug] stderr 示例：错误日志走 eprintln!，与 stdout 区分");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis().to_string())
        .unwrap_or_else(|_| "unknown".into());

    vec![
        format!("收到消息: {message}"),
        format!("消息长度: {} 字符", message.chars().count()),
        format!("后端时间戳: {timestamp} ms"),
    ]
}
```

| 宏 | 输出流 | 用途 |
|---|---|---|
| `println!` | stdout（标准输出） | 正常信息、进度、结果 |
| `eprintln!` | stderr（标准错误） | 错误、警告、需要与正常输出区分的内容 |

两者在终端里看起来差不多，但流是分开的——重定向 `> log.txt` 时只有 stdout 进文件，stderr 留在屏幕。**习惯上：`println!` 报状态，`eprintln!` 报异常**，与前端 `console.log` / `console.error` 的分工一一对应。

### 时间戳：SystemTime 的容错写法

```rust
std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)   // Result：系统时间早于纪元会 Err
    .map(|d| d.as_millis().to_string())       // 成功：毫秒 → 字符串
    .unwrap_or_else(|_| "unknown".into())     // 失败：兜底 "unknown"
```

`duration_since(UNIX_EPOCH)` 返回 `Result`——理论上的失败情形是"系统时间在 1970 之前"，现实中几乎不可能，但**API 返回 `Result` 就必须处理**。`.map(...).unwrap_or_else(...)` 是练习 10（模块收尾）会反复出现的"容错管道"的第一次亮相：**成功给值，失败给兜底，绝不 panic**。

`message.chars().count()` 是练习 05 学过的"字符数而非字节数"——消息长度按"字"算，中文消息不虚标。

---

## 3. 前后端对照 — 一次调用的双通道日志

### 把两条日志"对起来"

点击按钮，你会看到：

```
终端（后端）:                               DevTools Console（前端）:
[debug] 收到前端消息: hello tauri           [frontend] 页面加载完成...
[debug] stderr 示例：错误日志走 eprintln!    [frontend] 发送给后端: hello tauri
                                           [frontend] 后端返回: (3) [...]
```

**对照技巧：**

1. **消息内容当"关联键"** — 前后端都打印了 `hello tauri`，凭它确认"同一条消息"走到了哪一端
2. **看顺序** — 前端的"发送"日志先出现，后端的"收到"日志紧跟，最后是前端的"返回"日志——任何一步缺失，问题就出在断点处
3. **时间戳交叉验证** — 前端发送时间 vs 后端时间戳，能估算 IPC 耗时（毫秒级）

### 常见的"日志对不上"排查

| 现象 | 日志特征 | 问题在哪 |
|---|---|---|
| 前端点了没反应 | 前端"发送"日志都没有 | 事件没绑定、按钮 id 拼错 |
| 前端有"发送"，后端无"收到" | 只有 `[frontend]` 日志 | 命令未注册、命令名拼错（练习 04 的坑） |
| 后端有"收到"，前端无"返回" | 只有 `[debug]` 日志 | 后端 panic、返回类型不可序列化 |
| 前端有"调用失败" | `console.error` 红色输出 | 看错误信息本身，通常含原因 |

> **关键理解：** 日志是"故障隔离"的第一工具——**用日志先确定问题在哪一端，再深入那一端**。没有日志盲猜，等于在两个黑盒里同时找 bug。

### 练习版挖掉了什么

练习版把后端的 `println!`、`eprintln!`、时间戳计算和注册都挖成了 TODO（注释里有完整提示），前端的 `console.log` 调用和 invoke 也被注释。**注意：练习版 `main.ts` 的 catch 分支里 `console.error` 是留好的**——"错误必须留日志"是骨架的一部分，不需要你补。

> **练习流程：** 后端四个 TODO（属性、println!、eprintln!、时间戳）+ 注册；前端三个 TODO（页面加载日志、发送日志 + invoke、展示）。完成后运行 `cargo tauri dev`：终端里应出现 `[debug]` 两行，DevTools Console 里应出现 `[frontend]` 三行，页面显示三行结果。

---

## 知识点连起来看

```
┌──────────────────────────────────────────────┐
│ 前端 (WebView)         DevTools Console       │
│  console.log / error   F12 → Console          │
│       │                                      │
│       │  invoke（一条消息）                    │
│       ▼                                      │
│ Rust 核心进程          运行终端                │
│  println! / eprintln!  cargo tauri dev 输出   │
│                                              │
│ 对照方式：消息内容 + 时间顺序 + [前缀]过滤      │
└──────────────────────────────────────────────┘
```

| 层 | 本课回答的问题 | 关键概念 |
|---|---|---|
| 前端通道 | JS 日志去哪看 | DevTools、console.log / console.error |
| 后端通道 | Rust 日志去哪看 | println! / eprintln!、运行终端 |
| 流分工 | 正常与异常怎么分开 | stdout vs stderr、前缀标签 |
| 对照法 | 两条日志怎么对起来 | 消息内容、顺序、时间戳 |

**一通百通的核心：** 调试的本质是**建立可观察性**——前端一条日志、后端一条日志，一次调用的生命周期就被"点亮"了。`console.log` / `println!` 是两条最朴素的通道，但**"先定位是哪一端，再深入那一端"**这个方法论，会跟着你走完整个 Tauri 开发生涯（后面的插件、事件、状态调试都套用同一套框架）。

**递进关系：** 练习 08 是模块收尾——打包与图标。调试时看到的终端输出、DevTools，到了打包阶段会变成"产物"：bundle 里有什么、identifier 是什么、图标怎么生成。