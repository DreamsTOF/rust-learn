# 练习 19: 动态标签名

## 为什么要学这个

你在 `view!` 中写的标签名是静态的——`<h1>` 就是 `h1`，`<p>` 就是 `p`，编译期就确定死了。

但现实中你经常需要"同样的内容，不同的标签"：

- **目录组件** — 根据层级的 depth 值，渲染 `h1` / `h2` / `h3`
- **标题组件** — 用户通过下拉框选择标题级别
- **动态表格** — 列头应该是 `th` 还是 `td`，取决于上下文

在 Vue 或 React 中，你可以写 `<component :is="tagName">` 或 `<Tag as={dynamicTag}>`。但在 Leptos 的 `view!` 里，你不能写：

```rust
// ❌ 这在 view! 中不支持
view! {
    <{dynamic_tag}>"内容"</{dynamic_tag}>
}
```

**核心矛盾：** `view!` 是编译期宏，标签名必须在编译期确定。但"用什么标签"这个决定需要在运行期做出。你需要在 `view!` 之外找到动态创建 HTML 元素的能力。

---

## 从问题出发

### 为什么 `view!` 不能支持动态标签？

因为 `view!` 是一个**声明式宏**（declarative macro），它在编译期解析你写的 HTML 模板。宏看到 `<h1>` 就知道生成调用 `leptos::html::h1()` 的代码；看到 `<p>` 就知道生成调用 `leptos::html::p()` 的代码。

要让宏支持 `<TAG>` 这样的动态语法，需要：

1. 在宏内部处理可变标识符
2. 在编译期无法确定变量 `TAG` 的值
3. 生成能够运行期调度的代码

Rust 的宏系统做不到这一点——它不是 JavaScript 的运行时求值，而是编译期的源码变换。

解决方案不在宏层面，在**函数层面**。

### 构建器 API 的入口

Leptos 在 `leptos::html` 模块提供了每个 HTML 标签对应的**构建器函数**：

```
leptos::html::h1()    → 返回 HtmlElement<Heading> 类型
leptos::html::h2()    → 返回 HtmlElement<Heading> 类型
leptos::html::h3()    → 返回 HtmlElement<Heading> 类型
leptos::html::p()     → 返回 HtmlElement<Paragraph> 类型
leptos::html::div()   → 返回 HtmlElement<Div> 类型
...
```

这些函数直接返回构建器，你可以通过 `.child()`、`.attr()`、`.on()` 等方法链式调用。

关键是：**它们只是函数**。你可以在运行期动态选择调用哪个：

```rust
// 运行期决定用哪个标签
let tag = match level() {
    1 => leptos::html::h1(),
    2 => leptos::html::h2(),
    3 => leptos::html::h3(),
    _ => leptos::html::h1(),
};
```

---

## `into_any()` — 类型擦除

### 问题：不同类型无法统一

上面的代码有个类型问题——`leptos::html::h1()` 返回 `HtmlElement<Heading>`，`leptos::html::h2()` 也返回 `HtmlElement<Heading>`——但 Heading 泛型参数的值不同（对应不同的 HTML 规范类型）。

Rust 的强类型系统不会让你把一个 `match` 的不同分支返回不同类型：

```rust
let tag = match level() {
    1 => leptos::html::h1(),  // HtmlElement<H1>
    2 => leptos::html::h2(),  // HtmlElement<H2>
    // 编译错误：类型不匹配！
};
```

### 解决方案：`.into_any()`

`.into_any()` 是 `HtmlElement` 上的一个方法，它将**类型精确的 HTML 元素**转换为**类型擦除的 HTML 元素**——`HtmlElement<AnyElement>`。

```rust
let tag = match level.get() {
    1 => leptos::html::h1().child("标题").into_any(),
    2 => leptos::html::h2().child("标题").into_any(),
    3 => leptos::html::h3().child("标题").into_any(),
    _ => leptos::html::h1().child("标题").into_any(),
};
```

所有分支现在都返回 `HtmlElement<AnyElement>`，类型统一了。

### `AnyElement` 是什么？

`AnyElement` 是一个特殊的类型标签，表示"我不知道/不关心这个元素的具体类型"。它类似 Rust 标准库中的 `dyn Any`——把具体类型隐藏在了 trait object 后面。

```
HtmlElement<H1>  ──── .into_any() ────┐
HtmlElement<H2>  ──── .into_any() ────┤
HtmlElement<H3>  ──── .into_any() ────┼── HtmlElement<AnyElement>
                                      │
            类型信息被擦除，统一为一个类型。
```

> **设计权衡：** 类型擦除让你失去了编译期的类型检查——编译器不再能确保你在 `HtmlElement<AnyElement>` 上能调用哪些标签专有的方法。但这是动态标签的必然代价。Leptos 让 `into_any()` 成为一个显式方法调用，就是要让你清楚地知道："我从静态类型降级到动态类型了。"

### 在 `view!` 中使用

`.into_any()` 返回的类型实现了 `IntoView`，因此可以直接放在 `view!` 的 `{}` 中：

```rust
view! {
    {match level.get() {
        1 => leptos::html::h1().child(/* ... */).into_any(),
        2 => leptos::html::h2().child(/* ... */).into_any(),
        3 => leptos::html::h3().child(/* ... */).into_any(),
        _ => leptos::html::h1().child(/* ... */).into_any(),
    }}
    <p>"当前级别: " {level.get()}</p>
}
```

这里的核心模式是：`view!` 中嵌入 `{}` 来执行 Rust 代码，其中通过 `match` 动态选择标签构建器，再用 `.into_any()` 统一返回值类型。

---

## 静态 vs 动态

| 特性 | `view!` 静态标签 | 构建器 API + `into_any()` 动态标签 |
|------|-----------------|-----------------------------------|
| 标签名 | 编译期确定 | 运行期决定 |
| 类型检查 | 编译期完整检查 | 运行时可能出错（但通过 `.into_any()` 显式降级） |
| 写法 | 声明式、直观 | 命令式、灵活 |
| 性能 | 零开销——直接对应 DOM 操作 | 轻微动态分发开销 |
| 适用场景 | 绝大多数 UI | 需要动态标签的场景（标题级别、可配置组件） |

---

## 一个原则

```
view! 处理 95% 的 UI，构建器 API + into_any() 处理剩下那 5%。
```

`view!` 的静态标签不是"限制"——它是经过设计的选择。让标签名在编译期确定，意味着所有类型检查在编译期完成，没有任何运行期开销或意外。

当你碰到那 5% 需要动态标签的场景时，Leptos 给了你两个工具：

1. **构建器函数**（`leptos::html::h1()`）——让你在运行期选择创建哪个标签
2. **`.into_any()`**——让你把不同类型的元素统一为类型擦除的视图

这两者的组合，让你能在严格类型安全的 Rust 语言中，优雅地处理"直到运行期才知道标签名"的场景。

这种"宏优先 + 构建器 API 兜底"的设计策略贯穿 Leptos 始终——`view!` 是快车道，构建器 API 是灵活通道，你根据需要选择。
