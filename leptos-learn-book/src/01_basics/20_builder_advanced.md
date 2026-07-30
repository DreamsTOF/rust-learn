# 练习 20: 构建器模式高级

## 为什么要学这个

前一个练习（e19 动态标签名）引入了构建器函数的入口——`leptos::html::h1()`、`.into_any()`。但你只用它做了"创建一个标签"这件事。

构建器 API 的能力远不止于此。它是一条完整的**替代路径**——你完全可以用纯 Rust 函数调用链写出整个组件，完全不碰 `view!` 宏。

```rust
// view! 版本
view! {
    <div>
        <h2>"计数器"</h2>
        <p>"值: 0"</p>
        <button>"增加"</button>
    </div>
}

// 构建器 API 版本（等价）
div()
    .child(h2().child("计数器"))
    .child(p().child("值: 0"))
    .child(button().child("增加"))
```

**核心问题：** 既然有了 `view!`，为什么还需要这条替代路径？

答案藏在之前没法解决的问题里：

- 动态标签名（e19）
- 需要在循环/条件表达式中程序化生成子元素
- 需要将 DOM 元素的构建分散到多个函数中
- 需要绕过宏的语法限制（比如动态属性名）

`view!` 是声明式的——你说"要什么"，宏帮你生成。构建器 API 是命令式的——你说"怎么做"，每一步都在你控制中。

---

## 从问题出发

### 构建器 API 完整使用

Leptos 的构建器 API 是一组在 `leptos::html` 模块中的函数和链式方法：

```rust
use leptos::html::{button, div, h2, p};
use leptos::{ev, prelude::*};

#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);

    div()
        .child(h2().child("计数器"))
        .child(p().child(format!("值: {}", count.get())))
        .child(
            button()
                .child("增加")
                .on(ev::click, move |_| {
                    set_count.set(count.get() + 1);
                }),
        )
        .child(
            button()
                .child("重置")
                .attr("style", "margin-left: 8px;")
                .on(ev::click, move |_| {
                    set_count.set(0);
                }),
        )
}
```

逐层分析：

| 调用 | 作用 |
|------|------|
| `div()` | 创建 `<div>` 构建器 |
| `.child(...)` | 添加子元素，返回自身（链式调用） |
| `.on(ev::click, handler)` | 绑定点击事件处理器 |
| `.attr("style", "...")` | 设置 HTML 属性 |
| `button().child("...")` | 创建按钮并设置文本 |

没有 `.build()` 调用。整个链直接作为组件函数的返回值，因为最终的构建器类型实现了 `IntoView`。

### 为什么没有 `.build()`？

这是跟很多其他语言（如 Java）的构建器模式最大的区别。传统构建器通常有一个 `.build()` 方法来返回最终产物。

Leptos 的构建器不需要 `.build()`，因为：

1. 每个构建器方法返回 `self`——最后一个方法调用的结果就是最终的构建器
2. 构建器本身已经持有了所有配置信息（子元素、属性、事件）
3. `HtmlElement<...>` 类型实现了 `IntoView`，可以直接作为组件返回值

```
传统构建器:  A().b().c().build()  →  产物类型
Leptos 构建器: A().b().c()        →  产物类型（构建器本身就是产物）
```

这跟 Rust 中 `String::new()` + `.push_str()` 的链式调用是同一个模式——不需要额外的"构建完成"步骤。

---

## 构建器 API 能做什么而 `view!` 不能？

### 1. 动态标签名

```rust
// view! 做不到——标签名在编译期必须是字面量
// 构建器 API 可以
let header = if level() == 1 {
    leptos::html::h1().child("标题").into_any()
} else {
    leptos::html::h2().child("标题").into_any()
};
```

### 2. 程序化生成子元素

```rust
// view! 中对 for 循环的支持相对受限
// 构建器 API 可以自由组合
let mut container = div();
for item in items.iter() {
    container = container.child(p().child(item.name));
}
container
```

### 3. 条件性添加属性或事件

```rust
// view! 中属性要么有要么无
// 构建器 API 可以按条件选择
let mut btn = button().child("提交");
if requires_confirm {
    btn = btn.on(ev::click, confirm_handler);
} else {
    btn = btn.on(ev::click, direct_handler);
}
btn
```

### 4. 跨函数共享构建逻辑

```rust
fn build_header(text: &str) -> impl IntoView {
    h2().child(text).attr("class", "section-header")
}

// 在多个组件中复用
div()
    .child(build_header("用户信息"))
    .child(build_header("设置"))
```

---

## `view!` 宏 vs 构建器 API

| 维度 | `view!` 宏 | 构建器 API |
|------|-----------|-----------|
| 可读性 | ★★★★★ HTML 式，直观 | ★★★☆☆ 嵌套函数，较难一眼看懂树结构 |
| 类型安全 | ★★★★★ 编译期完整检查 | ★★★★☆ 类型安全但需要显式 `.into_any()` |
| 动态能力 | ★★☆☆☆ 仅支持有限的动态性 | ★★★★★ 完全程序化控制 |
| 编写效率 | ★★★★★ 接近普通 HTML | ★★★☆☆ 需要更多样板代码 |
| 条件逻辑 | 内嵌 `{}` + `if` | 链式调用 + Rust 标准流程控制 |
| 学习曲线 | 低（HTML 知识可迁移） | 中（需要理解构建器模式） |

### 如何选择？

```
┌──────────────────────────────────────────────┐
│  你能用 view! 写吗？  ──是──→  用 view!      │
│        │                                     │
│        否                                    │
│        ↓                                     │
│  需要动态标签 / 程序化子元素 /               │
│  条件性属性 / 跨函数共享构建逻辑？            │
│        │                                     │
│        是                                    │
│        ↓                                     │
│  用构建器 API                                │
└──────────────────────────────────────────────┘
```

> **简单原则：** 默认用 `view!`。当 `view!` 的限制让你不得不跳出宏时，才用构建器 API。`view!` 不是"初学者的玩具"，而是"首选方案"；构建器 API 不是"高阶的象征"，而是"灵活的后备"。

---

## 三个层次连通

```
练习 14（构建器入门） → 练习 19（动态标签名） → 练习 20（构建器高级）

  ↓                     ↓                       ↓
  认识构建器            用构建器解决            完整构建器组件
  `.child()`            动态标签问题            事件 + 样式 + 属性
  基本的链式调用        `.into_any()`           纯构建器模式
```

这三个练习构成了一条完整的进阶路径：

1. **练习 14** 让你知道构建器 API 的存在
2. **练习 19** 让你在动态标签场景中第一次真正使用构建器
3. **练习 20** 让你完全脱离 `view!`，用构建器 API 写出完整组件

---

## 一通百通

回顾从练习 01 到练习 20，你对 Leptos 的"视图层"已经掌握了三种表达方式：

| 方式 | 本质 | 使用场景 |
|------|------|---------|
| `view!` 宏 | HTML 模板 → 编译器展开 | 90% 的常规 UI |
| 构建器 API | 函数调用链 | 动态标签、程序化结构 |
| `inner_html` | 原始 HTML 注入 | 渲染预格式化 HTML |

这三种方式不是"低级替代高级"的关系——它们是**同一抽象层次的不同工具**，分别适用于不同的场景。

- `view!` 是锤子——90% 的钉子用它敲
- 构建器 API 是螺丝刀——需要精细控制时用它
- `inner_html` 是电钻——有风险但必要的时候用它

知道什么时候用什么工具，比把所有工具用熟更重要。
