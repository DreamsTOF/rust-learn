# 练习 13: 索引与方法调用

## 为什么要学这个

上一个练习我们学会了在 `view!` 中嵌入 Rust 控制流。但控制流的对象是什么？**数据**。

在真实的应用中，数据多以集合形式出现——数组、列表、映射。当你有了一个数据集合后，需求接踵而至：

- "这里有几项？" → 调用 `.len()`
- "第一项是什么？" → 索引 `[0]`
- "每一项分别显示出来" → 循环遍历

这个练习回答三个问题：

1. **`{ }` 里到底能放哪些 Rust 表达式？** — 有没有什么限制？
2. **`items.len()` 和 `items[0]` 为什么能在 `view!` 中直接使用？** — 这跟普通 Rust 代码有什么不同？
3. **非响应式数据 vs 响应式信号，在 `view!` 中的表现有何区别？** — 什么时候更新？

---

## 从问题出发

### 集合操作是 UI 的基础

假设你有一个语言列表：

```rust
let items = vec!["Rust", "Leptos", "WASM"];
```

在界面上，你几乎总是需要做三件事：

```
┌───────────────────────────────────────┐
│  编程语言列表                          │
│                                       │
│  共有 3 门语言          ← 调用 .len()  │
│  第一门语言: Rust       ← 索引 [0]     │
│  第二门语言: Leptos     ← 索引 [1]     │
│  第三门语言: WASM       ← 索引 [2]     │
└───────────────────────────────────────┘
```

在 Leptos 的 `view!` 中，这些操作跟你写普通 Rust 代码的方式完全一样：

```rust
view! {
    <p>"共有 " { items.len() } " 门语言"</p>
    <p>"第一门语言: " { items[0] }</p>
    <p>"第二门语言: " { items[1] }</p>
}
```

**关键洞见：** `view!` 的大括号 `{ }` 没有"特殊规则"。里面的代码就是普通的 Rust 代码。`.len()` 是 Rust 标准库的 `Vec::len`，`[0]` 是 Rust 的 `Index` trait 的语法糖。Leptos 没有重新发明这些东西——它只是让你能直接使用它们。

### 这跟 JavaScript 模板有什么不同？

在 JavaScript 的 JSX 或模板字符串中：

```jsx
<p>共有 {items.length} 门语言</p>
<p>第一门语言: {items[0]}</p>
```

JavaScript 的模板语法和 Leptos 的 `view!` 在表面上极其相似。但底层差异巨大：

| | JavaScript (JSX) | Rust (view!) |
|---|---|---|
| `items.length` | 运行时访问属性 | 编译时类型检查，调用方法 |
| `items[0]` | 运行时索引 | 编译时检查 `Index` trait 实现 |
| 类型安全 | ❌ — `items.length` 拼错成 `items.lengh` 不报错 | ✅ — 拼错方法名编译直接失败 |
| 调试体验 | 运行时 TypeError | 编译错误，带准确的行号和提示 |

**Rust 的优势：** 在 JSX 中，如果你写 `items.len()`（误用了 Rust 风格），运行时才会报错。在 Rust 中，你写 `items.lengh` 编译器直接告诉你方法名不存在。这在大型代码库中是一个巨大的生产力提升。

---

## 为什么 `view!` 中可以直接调用方法？

### 魔法来自编译器，不是框架

`view!` 宏的基本工作方式是：**把看起来像 HTML 的模板，在编译期转换成普通的 Rust 函数调用**。

当你写：

```rust
view! {
    <p>"共有 " { items.len() } " 门语言"</p>
}
```

宏展开后的代码（简化）大致是这样的：

```
// 编译期展开
leptos::html::p()
    .child("共有 ")
    .child(items.len())    // ← 这行就是普通的 Rust 表达式
    .child(" 门语言")
    .into_any()
```

核心要点：**`items.len()` 的出现位置和普通 Rust 代码中的位置没有区别**。编译器对它的处理方式完全一致——类型检查、方法解析、所有权检查，一切照常。

> **设计原理：** Leptos 不引入自己的"表达式语言"。它直接用 Rust 的表达式。这意味着你不需要学习一套新的 DSL 语法来操作数据。你已有的 Rust 知识——`Vec` 的方法、`String` 的操作、迭代器——全部可以直接用在 `view!` 中。

### `Index` trait：`[0]` 背后的魔法

你写 `items[0]` 时，Rust 编译器实际上把它转换成：

```rust
*items.index(0)
```

这由标准库的 `Index` trait 支持：

```rust
pub trait Index<Idx> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}
```

`Vec<T>` 实现了 `Index<usize>`，所以你可以用 `items[0]`、`items[1]` 等。如果索引越界，会在运行时 panic——这一点跟普通 Rust 中一样。

> **安全提示：** `view!` 中的索引跟 Rust 中的索引一样有越界风险。如果你不能保证索引总在范围内（例如动态列表），考虑用 `.get(index)` 返回 `Option<&T>` 配合 `match` 处理越界情况。

---

## 非响应式 vs 响应式的关键区别

这个练习中的 `items` 是一个普通的 `Vec<&str>`：

```rust
let items = vec!["Rust", "Leptos", "WASM"];
```

它**不是**信号（Signal），也不是响应式的。这意味着：

```
┌──────────────────┐
│  items = ["Rust",│
│   "Leptos",      │
│   "WASM"]        │
│                  │
│  这是一个普通变量   │
│  创建后就固定了     │
│  不会变           │
└──────────────────┘
       ↓
view! 中引用 { items.len() }
       ↓
首次渲染执行一次，之后永不更新
       ↓
即使你后面修改了 items，界面也不会变
```

如果想让界面随数据变化而更新，你需要把 `items` 放进信号：

```rust
let (items, set_items) = signal(vec!["Rust", "Leptos", "WASM"]);

view! {
    <p>"共有 " { items().len() } " 门语言"</p>
}
```

注意区别：

| 写法 | 类型 | 行为 |
|---|---|---|
| `items.len()` | 普通 `usize` | 渲染时执行一次，永不更新 |
| `items().len()` | 信号包裹的 `usize` | 每次 `items` 变化时重新执行并更新 DOM |
| `items.get().len()` | 一次性的 `usize` | 读取当前值，不跟踪后续变化 |

> **核心原则：** `<p>{ items.len() }</p>` 和 `<p>{ items().len() }</p>` 在语法上只差一对括号，但在语义上差了一个"响应式订阅"。前者是一次性计算，后者建立了数据到 UI 的自动更新通道。

---

## 不仅仅是 `len()` 和 `[ ]`

学会了在 `view!` 中调用 `.len()` 和 `[ ]`，你就可以在 `view!` 中调用任何你喜欢的 Rust 方法了。例如：

```rust
let items = vec!["Rust", "Leptos", "WASM"];

view! {
    <p>{ items.first().unwrap_or(&"无") }</p>           // Option 方法
    <p>{ items.contains(&"Rust") }</p>                   // Vec 方法
    <p>{ items.join(", ") }</p>                          // 切片方法（需 String）
}
```

它们都工作，因为 `view!` 的 `{ }` 不限制你写什么表达式——只要表达式实现了 `IntoView`（或可隐式转换的类型如 `String`、`&str`、`i32`、`bool`）。

```
适用于 view! 中 { } 的类型
────────────────────────
✅ &str, String           ← 文本
✅ i32, i64, u32, ...     ← 数字
✅ bool                   ← 显示 "true" / "false"
✅ HtmlElement<...>       ← 元素
✅ Vec<impl IntoView>     ← 列表
✅ Option<impl IntoView>  ← 可选内容
✅ Result<impl IntoView, impl IntoView>  ← 成功/错误
```

---

## 一通百通

这个练习的篇幅很短，但揭示了一个深远的概念：

```
view! 不是"模板语言"，它是"Rust 代码的声明式外观"
```

普通模板语言需要你学习它们的表达式语法、过滤器、辅助函数。Leptos 的 `view!` 则说："你知道 Rust 就够了。在 `{ }` 里，用你熟悉的方式操作数据。"

| 知识点 | 核心理解 |
|--------|---------|
| `{ items.len() }` | 在 view! 中调用 Vec 的方法 |
| `{ items[0] }` | 使用 Index trait 访问元素 |
| 非响应式数据 | 渲染一次后不更新 |
| `.len()` 与 `items().len()` | 普通值 vs 信号包裹的值 |
| `view!` 不限制表达式 | 任何实现 IntoView 的类型均可 |

到目前为止，你在 `view!` 的 `{ }` 中已经学过三种用法：

```
{ num }                    ← 练习 01: 直接显示信号值
{ match x { ... } }        ← 练习 12: 模式匹配
{ items.len() }            ← 练习 13: 方法调用和索引
```

它们的共同原理是相同的：**`{ }` 内是普通 Rust 代码**。只要掌握了这个原理，你就能把整个 Rust 标准库和第三方 crate 的能力引入 UI 层。
