# 练习 05: 参数与返回值

## 为什么要学这个

练习 04 传了一个 `&str`，但真实应用的命令很少这么简单：表单、配置、业务数据，往往是**数字、布尔、数组、嵌套结构体**混着来。这一课回答三个问题：

1. **命令参数到底能有多复杂？** — `String`、`i32`、`bool`、`Vec<String>`、结构体，五种类型一次传齐，各自怎么声明？
2. **`Deserialize` 和 `Serialize` 有什么区别？** — 为什么进方向要 `Deserialize`、出方向要 `Serialize`，搞反了会怎样？
3. **snake_case 和 camelCase 是怎么对上的？** — Rust 的 `text_length` 为什么到前端变成了 `textLength`？谁在背后做转换？

回答完这三个问题，你就掌握了命令数据契约的完整形态——之后所有练习的传参，都是这五种类型的排列组合。

---

## 从问题出发

练习 05 要做的事：**一个表单，五种类型的输入（文本、数字、布尔、逗号分隔列表、两个数的加法），提交后后端返回一个结构化结果**。

```
前端 (WebView)                        Rust 核心进程
┌─────────────────────┐    IPC     ┌──────────────────────────┐
│ text: "Hello Tauri" │ ─────────► │ fn analyze(              │
│ number: 21          │  { ... }   │     text: String,        │
│ flag: true          │            │     number: i32,         │
│ items: [rust,tauri] │ ◄───────── │     flag: bool,          │
│ calc: { a: 3, b: 4 }│  Summary   │     items: Vec<String>,  │
└─────────────────────┘            │     calc: CalcInput      │
                                   │ ) -> Summary             │
                                   └──────────────────────────┘
```

**核心矛盾：** 练习 04 的 `&str` 背后是 Tauri 自动完成的序列化——一条 `name` 就能对上。但类型一多，问题就来了：**"进"和"出"两个方向的序列化是两套 trait**——前端传来的 JSON 要**反序列化**成 Rust 值（`Deserialize`），Rust 的返回值要**序列化**成 JSON（`Serialize`）。搞混方向，编译期就会报 trait 未实现。

---

## 1. 参数方向 — `Deserialize`：把 JSON 变成 Rust 值

### 五种参数类型

```rust
#[tauri::command]
fn analyze(
    text: String,
    number: i32,
    flag: bool,
    items: Vec<String>,
    calc: CalcInput,
) -> Summary {
    ...
}
```

| 参数 | Rust 类型 | 前端对应的值 | 反序列化来源 |
|---|---|---|---|
| `text` | `String` | `"Hello Tauri"` | 字符串 |
| `number` | `i32` | `21` | 数字 |
| `flag` | `bool` | `true` | 布尔 |
| `items` | `Vec<String>` | `["rust","tauri","vite"]` | 数组 |
| `calc` | `CalcInput` | `{ a: 3, b: 4 }` | 嵌套对象 |

前四种是标准库类型——**它们天生实现了 `Deserialize`**，不需要任何标注。第五种是自定义结构体，必须自己声明：

```rust
// 前端传来的结构体参数：
// Rust 侧字段是 snake_case，JS 侧自动使用 camelCase 传参
#[derive(serde::Deserialize)]
struct CalcInput {
    a: i32,
    b: i32,
}
```

`#[derive(serde::Deserialize)]` 让编译器自动生成"从 JSON 构造 `CalcInput`"的代码：找到 `a` 和 `b` 两个 key，解析成 `i32`。

> **关键理解：** `Deserialize` 是**输入方向**的契约——"我能从 JSON 变回来"。命令的参数（前端 → 后端）需要它。没有它，`invoke` 时参数解析会失败。

### 为什么是 `i32` 而不是别的

数字类型的宽度选择有讲究：`i32` 是 32 位有符号整数，JSON 里的数字（JS 的 `number`）默认按双精度浮点表示，32 位整数在安全范围内可以无损往返。练习里用 `i32` 是"够用且惯用"的选择——真实项目里根据数值范围选 `i32`/`i64`/`f64`。

---

## 2. 返回方向 — `Serialize`：把 Rust 值变成 JSON

### 返回结构体

```rust
// 返回给前端的结果结构体
#[derive(serde::Serialize)]
struct Summary {
    text_length: usize,
    doubled: i32,
    reversed_flag: bool,
    item_count: usize,
    total: i32,
}

/// 混合多种参数类型（String / i32 / bool / Vec / 结构体），
/// 返回结构化结果，演示完整序列化链路。
#[tauri::command]
fn analyze(
    text: String,
    number: i32,
    flag: bool,
    items: Vec<String>,
    calc: CalcInput,
) -> Summary {
    Summary {
        text_length: text.chars().count(),
        doubled: number * 2,
        reversed_flag: !flag,
        item_count: items.len(),
        total: calc.a + calc.b,
    }
}
```

五个字段，五种算法，每个都有讲究：

| 字段 | 计算 | 为什么这么写 |
|---|---|---|
| `text_length` | `text.chars().count()` | **字符数，不是字节数**——`"中文"` 的 `.len()` 是 6（UTF-8 每字 3 字节），`.chars().count()` 才是 2 |
| `doubled` | `number * 2` | 普通算术 |
| `reversed_flag` | `!flag` | 布尔取反 |
| `item_count` | `items.len()` | 数组长度 |
| `total` | `calc.a + calc.b` | 结构体字段访问 |

### `chars().count()` — 中文长度的坑

```rust
"Hello Tauri".chars().count()  // 11
"中文文本".chars().count()     // 4
"中文文本".len()               // 12（UTF-8 字节数！）
```

Rust 的 `String::len()` 返回**字节数**——UTF-8 编码下每个中文占 3 字节。要"数有几个字"，必须用 `.chars().count()`。这个坑在表单校验、字数统计里反复出现，练习 07 的时间戳/消息长度也会用到同样的写法。

> **练习的坑：** 练习版把五个计算全挖成了占位值（`0` / `false`）。照抄提示时最容易写错的就是 `text_length`——写成 `text.len()` 编译能过、运行能跑，但中文文本的计数是错的。**编译通过 ≠ 行为正确**，这是第一个"语义级"的坑。

---

## 3. snake_case ↔ camelCase — 谁在背后做转换

### 现象

后端字段叫 `text_length`，前端接口里却是 `textLength`：

```typescript
// 与后端 Summary 对应的 TS 接口（camelCase ↔ snake_case 自动转换）
interface Summary {
  textLength: number;
  doubled: number;
  reversedFlag: boolean;
  itemCount: number;
  total: number;
}
```

**Tauri 在 IPC 边界自动做蛇形 ↔ 驼峰转换**：Rust 的 `text_length` 序列化给前端时变成 `textLength`；前端传 `textLength` 反序列化回 Rust 时还原成 `text_length`。这是 Tauri 对 JS 生态（camelCase 惯例）与 Rust 生态（snake_case 惯例）的自动调和。

### 前端调用：五种类型的传法

```typescript
const summary = await invoke<Summary>("analyze", {
  text: (fd.get("text") as string) || "",
  number: Number(fd.get("number")) || 0,
  flag: fd.get("flag") === "on",
  items: ((fd.get("items") as string) || "")
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean),
  calc: {
    a: Number(fd.get("a")) || 0,
    b: Number(fd.get("b")) || 0,
  },
});
```

| 参数 | 前端转换 | 说明 |
|---|---|---|
| `text` | `fd.get("text") as string` | 直接取字符串 |
| `number` | `Number(...) \|\| 0` | 字符串 → 数字，空值兜底 0 |
| `flag` | `fd.get("flag") === "on"` | checkbox 选中时 FormData 里有 `"on"` |
| `items` | `.split(",").map(trim).filter(Boolean)` | 逗号分隔 → 数组，去空白去空项 |
| `calc` | `{ a: Number(...), b: Number(...) }` | 嵌套对象，对应 `CalcInput` |

`fd` 是 `new FormData(form)`——浏览器原生的表单数据提取 API，`name` 属性就是 key。**前端负责"表单 → 对象"的转换，Tauri 负责"对象 → JSON → Rust 值"的转换**，两层分工清晰。

### 展示：JSON 原样输出

```typescript
outputEl!.textContent = JSON.stringify(summary, null, 2);
```

`JSON.stringify(summary, null, 2)` 把返回的结构体格式化成缩进 2 格的 JSON 文本——`<pre id="output">` 里直接展示。这是调试命令返回值的通用手法：**先原样看 JSON，再谈渲染**。

---

## 知识点连起来看

```
┌──────────────────────────────────────────────┐
│ 前端 (WebView)                               │
│  invoke<Summary>("analyze", { text, number,  │
│      flag, items, calc })                    │ ← 传参层：对象即参数
│      │                                       │
│      │  camelCase ←→ snake_case 自动转换      │
│      ▼                                       │
│ Rust 核心进程                                │
│  #[derive(Deserialize)] CalcInput            │ ← 输入契约：JSON → 值
│  #[derive(Serialize)]  Summary               │ ← 输出契约：值 → JSON
│  fn analyze(...) -> Summary                  │ ← 业务层：纯计算
└──────────────────────────────────────────────┘
```

| 层 | 本课回答的问题 | 关键概念 |
|---|---|---|
| 输入契约 | JSON 怎么变成 Rust 值 | `Deserialize`、类型映射 |
| 业务层 | 数据怎么处理 | `chars().count()`、结构体运算 |
| 输出契约 | Rust 值怎么变成 JSON | `Serialize`、字段名即 key |
| 命名层 | 两种命名风格怎么调和 | snake_case ↔ camelCase 自动转换 |

**一通百通的核心：** 命令的数据契约是**双向**的——进去要 `Deserialize`，出来要 `Serialize`，自定义类型必须显式声明（derive），标准库类型天生具备。参数可以是任意组合的嵌套结构，前端传对象、后端收参数、返回再变对象，**序列化是唯一的通道**。这个模型覆盖了后面所有练习（事件载荷、状态、插件 API）的数据传递。

**递进关系：** 练习 06 将把战场从"数据"转移到"窗口"——`AppHandle` 注入 + `WebviewWindowBuilder` 动态创建窗口，命令第一次产生"副作用"（真的创建一个窗口），而不是只返回数据。