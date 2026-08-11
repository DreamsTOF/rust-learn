# 练习 04: 第一个命令

## 为什么要学这个

前三课你都在"读取"后端——环境清单、结构说明、构建配置，命令要么无参要么只注入 AppHandle。这一课开始**双向通信**：前端把数据发给后端，后端处理后返回。三个问题：

1. **命令怎么接收参数？** — 前端的输入框怎么变成 Rust 函数里的变量？
2. **全链路长什么样？** — `#[tauri::command]` → `generate_handler!` → `invoke()`，这条链每一环各干什么？
3. **调用失败了怎么办？** — 后端报错、命令没注册，前端怎么优雅处理而不是白屏？

回答完这三个问题，你就走通了 Tauri 开发里最高频的操作——**带参调用命令**。之后所有练习都是它的变体。

---

## 从问题出发

练习 04 要做的事：**输入框里填一个名字，点击按钮，后端返回一句问候语**。

这是 Tauri 官方脚手架里 `greet` 命令的经典形态——"Hello World"的命令版。它把练习 01 的三层模型补上了最后一块：**参数**。

```
前端 (WebView)                   Rust 核心进程
┌──────────────────┐    IPC    ┌──────────────────────┐
│ input #name       │ ────────► │ #[tauri::command]    │
│ "Tauri"           │  { name } │ fn greet(name: &str) │
│ invoke("greet")   │ ◄──────── │ -> String            │
│ 拿到问候语并展示    │  "你好, Tauri! ..." │            │
└──────────────────┘           └──────────────────────┘
```

**核心矛盾：** 前端的世界里名字是"字符串变量"，Rust 的世界里名字是"`&str` 参数"——两个世界之间隔着一个进程边界，数据必须序列化成 JSON 才能穿越。**参数怎么传、怎么回来，就是本课的全部内容。**

---

## 1. 命令参数 — 从输入框到 `&str`

### 答案版的后端

```rust
/// 接收 name 参数并返回问候语。
/// &str 参数会被前端按值传入，命令返回 String 直接序列化给前端。
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {name}! 这是你的第一个 Tauri 命令 🎉")
}
```

三个细节值得拆开：

**1. 参数类型是 `&str`（借用）而不是 `String`（拥有）。**

命令的参数由 Tauri 反序列化生成。对"只需要读、不需要拥有"的参数，`&str` 是更轻的选择——反序列化后借用即可，避免不必要的拷贝。这是 Tauri 命令的惯用写法（`String` 参数同样合法，练习 05 会用）。

**2. `format!` 是格式化宏。**

`format!("你好, {name}! ...")` 里 `{name}` 是**内联变量插值**（Rust 1.58+ 的捕获格式）——等价于老写法 `format!("你好, {}! ...", name)`。它返回新的 `String`，不修改原参数。

**3. 参数名就是"协议"。**

`name` 这个参数名，前端调用时必须用同名 key 传值——**命令签名就是前后端的协议**。

### 练习版挖掉了什么

```rust
// TODO: 添加 #[tauri::command] 属性
// TODO: 补全函数：接收 name: &str，返回 String 问候语
// 提示: format!("你好, {name}! ...")
fn greet() -> String {
    // TODO: 使用 format! 拼接待返回的问候语（可以带上 emoji 🎉）
    String::new()
}
```

练习版把三样东西挖掉了：`#[tauri::command]` 属性、`name: &str` 参数、`format!` 函数体。**注意顺序**：如果你先填了函数体但忘了加属性，前端 `invoke("greet")` 会得到"命令未找到"；加了属性但忘了注册（下一节），结果一样。三个 TODO 缺一不可。

### 注册：别忘了挂在 Builder 上

```rust
.invoke_handler(tauri::generate_handler![greet])
```

练习 01 讲过：命令必须注册到运行时，前端才调得到。`generate_handler![greet]` 把 `greet` 加入命令注册表。**这是最容易漏的一步**——函数写得再好，没注册就是"命令未找到"。

---

## 2. 前端调用 — `invoke` 的第二参数

### 传参：键值对对象

```typescript
const name = nameInput!.value.trim() || "Tauri";
const message = await invoke<string>("greet", { name });
```

`invoke` 的第二个参数是一个**对象**，它的 key 必须与 Rust 参数名一致：

```typescript
invoke<string>("greet", { name })
//                        ^^^^^^
//                        key = Rust 参数名
```

Tauri 会把 `{ name: "Tauri" }` 序列化成 `{"name":"Tauri"}`，后端按 `name` 反序列化进 `greet` 的参数。**key 拼错（比如 `{ n: "Tauri" }`），后端会报参数缺失**——这是前后端联调最常见的错误之一。

> **关键理解：** 命令参数没有"位置"概念，只有"名字"概念。Rust 侧 `fn greet(name: &str)`、前端 `{ name }`——两边靠名字对上。练习 05 会看到这个规则在 camelCase/snake_case 下的完整形态。

### 为什么是 `async` 事件处理器

```typescript
greetBtn!.addEventListener("click", async () => {
  const name = nameInput!.value.trim() || "Tauri";
  try {
    const message = await invoke<string>("greet", { name });
    resultEl!.textContent = message;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});
```

- `nameInput!.value.trim() || "Tauri"` — 读取输入框，去空白，空值兜底为 "Tauri"（`||` 在这里是"空就换默认值"）
- `await invoke` — 练习 01 讲过：IPC 是异步的，必须等待 Promise
- `resultEl!.className = "status ok"` — 成功/失败切换样式类，页面用颜色区分结果

### 错误处理：try/catch 是标准姿势

`invoke` 失败时会 reject Promise，`try/catch` 捕获后展示给用户。什么情况下会失败？

| 失败原因 | 例子 | 用户看到 |
|---|---|---|
| 命令未注册/名字拼错 | `invoke("greet")` vs 注册的是 `greet` | "命令未找到"错误 |
| 参数对不上 | 前端传 `{ n: ... }`，后端要 `name` | 参数解析错误 |
| 后端 panic | 命令内部 `panic!` | 调用失败错误 |
| 参数类型不匹配 | 后端要 `i32`，前端传字符串 | 反序列化错误 |

> **设计思路：** 前端永远假设 `invoke` 可能失败——网络请求有超时，IPC 有边界情况，后端代码可能 panic。**"先 try/catch，再谈业务"** 是 Tauri 前端的铁律。练习版把这个结构留好了（catch 分支完整），你只需要填 try 里的调用。

> **练习流程：** 后端两个 TODO（加属性 + 补参数和函数体）+ 注册一个 TODO；前端两个 TODO（invoke 调用 + 展示结果）。完成后运行 `cargo tauri dev`，输入名字点按钮，窗口里出现问候语——你的第一个双向命令就通了。

---

## 3. 全链路回顾 — 一条消息的旅程

把前后端拼起来，看一条消息的完整旅程：

```
① 前端：用户点击按钮
② 前端：invoke("greet", { name: "Tauri" })
        │  对象序列化成 JSON: {"name":"Tauri"}
        ▼
③ IPC 跨进程传输（消息队列）
        ▼
④ 运行时：在命令注册表里查 "greet"
        ▼
⑤ 运行时：反序列化参数 → 调用 greet("Tauri")
        ▼
⑥ 后端：format! 生成 "你好, Tauri! ..."
        │  返回值序列化成 JSON
        ▼
⑦ IPC 回传
        ▼
⑧ 前端：Promise resolve，await 拿到字符串
```

| 环节 | 谁负责 | 出错时 |
|---|---|---|
| 序列化参数 | Tauri（自动） | 参数类型不匹配 → 反序列化错误 |
| 查表找命令 | 运行时（注册表） | 名字不在表里 → "命令未找到" |
| 执行函数 | 你的 Rust 代码 | panic → 调用失败 |
| 序列化返回 | Tauri（自动） | 返回类型不可序列化 → 编译期就报错 |
| 展示结果 | 你的 TS 代码 | —（try/catch 兜底） |

注意第 ③⑤⑦ 步都是 Tauri 自动完成的——**你只需要写函数（后端）和调用（前端），中间的一切是框架的事**。这就是练习 01 说的"以最少的仪式连接起来"。

---

## 知识点连起来看

```
┌──────────────────────────────────────────────┐
│ 前端 (WebView)                               │
│  invoke<string>("greet", { name })           │ ← 调用层：传参 + 等待
│  try { ... } catch { ... }                   │ ← 容错层：失败可展示
│                                              │
│ Rust 核心进程                                │
│  #[tauri::command] fn greet(name: &str)     │ ← 命令层：参数 + 逻辑
│  generate_handler![greet]                    │ ← 注册层：挂到运行时
│                                              │
└──────────────────────────────────────────────┘
```

| 层 | 本课回答的问题 | 关键概念 |
|---|---|---|
| 命令层 | 参数怎么写 | `&str` 借用、`format!` |
| 注册层 | 命令怎么被找到 | `generate_handler!`、按名查找 |
| 调用层 | 参数怎么传 | 第二参数对象、key = 参数名 |
| 容错层 | 失败怎么办 | async/await + try/catch |

**一通百通的核心：** 带参调用是 Tauri 命令的标准形态——**前端传对象，后端收参数，两边靠名字和类型对上**。掌握它之后，练习 05 的多类型参数（数字、布尔、数组、结构体）只是这个形态的扩展，练习 07 的调试也会复用同样的调用模式。

**递进关系：** 练习 05 将把参数从单个 `&str` 扩展成五种类型（String / i32 / bool / Vec / 结构体），并深入 snake_case ↔ camelCase 的自动转换——那是"参数协议"的完整版。