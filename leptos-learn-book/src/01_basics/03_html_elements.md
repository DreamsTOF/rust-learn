# 练习 03: HTML 元素与属性

## 为什么要学这个

文本节点只是第一步。真实的网页由各种 HTML 元素组成——标题、段落、链接、图片、表单……每个元素都有它的语义和属性。

这一节回答四个问题，它们将帮你理解 `view!` 如何跟标准 HTML 世界对接：

1. **`view!` 中的 HTML 属性和 HTML 标准属性完全一致吗？** — 是的，但有一个重要的前提
2. **为什么 class 和 id 可以直接写，而 style 是一个字符串？** — 三种属性的不同本质
3. **自闭合标签（`<br/>`、`<img/>`）在 view! 中怎么写？** — 跟 HTML5 一样，但必须闭合
4. **`<a>` 的 href 和 `<img>` 的 src —— 链接的本质是什么？** — 从资源的视角理解属性

这些问题看似琐碎，但它们背后有一个统一的答案：**`view!` 的属性语法，就是标准 HTML 语法的直接映射。** 你已有的 HTML 知识完全可用，只需注意几个宏特有的约束。

---

## 从问题出发

HTML 元素由三部分组成：

```
标签名     属性                 内容
┌──┐   ┌──────────┐       ┌──────────────┐
<h1 id="title" class="heading">Hello, World!</h1>
                              └──────────────┘
                              文本节点或子元素
```

在传统 Web 开发中，你直接在 HTML 文件中写这些。在 Leptos 中，你通过 `view!` 宏来写——但语法跟 HTML 几乎一样：

```rust
view! {
    <h1 id="title" class="heading">"Hello, World!"</h1>
}
```

**这意味着你已有的 HTML 知识可以直接迁移到 Leptos。** 但有一个关键差异需要注意——这也是本节要深入的内容。

---

## 1. 属性语法 — 为什么跟 HTML 一样，但又不完全一样？

### 一样的部分

`view!` 中的属性语法和 HTML 属性的写法几乎一致：

```rust
// HTML: <h1 id="title" class="heading">
// view!:
<h1 id="title" class="heading">"标题"</h1>
```

属性名和属性值都用双引号包裹，名字和 HTML 标签的属性名完全一致——**没有驼峰命名转换**。

这对于 React 开发者来说是一个重要的区别：

| 属性 | React (JSX) | HTML | Leptos (view!) |
|------|-------------|------|----------------|
| CSS 类名 | `className` | `class` | `class` |
| 点击事件 | `onClick` | `onclick` | `on:click` |
| 标签属性 | `htmlFor` | `for` | `r#for` |
| 内联样式 | `{{}}` 对象 | `style="..."` 字符串 | `style="..."` 字符串 |

**Leptos 的原则：** 跟随 HTML 标准，而不是 React 的习惯。因为你最终生成的是 DOM，不是 React 的虚拟 DOM。直接用 HTML 的属性名更自然，也减少了学习成本。

### 不一样的细节：属性值可以是 Rust 表达式

虽然 `id="title"` 看起来像静态 HTML，但属性值也可以来自 Rust 表达式：

```rust
let my_id = "title";

view! {
    <h1 id={my_id}>"标题"</h1>
}
```

当属性值用 `{ }` 包裹时，`view!` 会把它当作 Rust 表达式求值。这跟文本节点中的 `{ }` 是同一个机制。

> **关键理解：** `id="title"` 是语法糖，等价于 `id={String::from("title")}`。所有属性最终都被编译为类型安全的 Rust 函数调用。

---

## 2. class / id / style — 三种属性的不同本质

### class 和 id：字符串属性

`class` 和 `id` 是 HTML 中最基础的标识属性：

```rust
<h1 id="title" class="heading">"标题"</h1>
```

- `id` — 唯一标识，页面中每个 id 只能用一次
- `class` — CSS 类名，可以重复使用

它们在 `view!` 中的处理最直接——就是简单字符串。

### style：内联样式的两种方式

`style` 属性在 `view!` 中同样支持，而且是字符串形式：

```rust
<p style="color: blue; font-size: 18px;">"这是一个带样式的段落"</p>
```

**为什么 style 是一个字符串，而不是一个对象？** 这是 `view!` 跟 React 的一个重要区别。

在 DOM 标准中，`style` 属性本来就是一个字符串：

```javascript
// DOM API
element.setAttribute('style', 'color: blue; font-size: 18px;');
// 或者
element.style.color = 'blue';
element.style.fontSize = '18px'; // 驼峰命名
```

React 选择使用对象语法（`{{ color: 'blue', fontSize: 18 }}`），因为它在 JSX 中更自然地嵌入 JavaScript。但 `view!` 选择跟随 HTML 标准——属性值就是字符串，跟你在 HTML 文件中写的一样。

| 写法 | 示例 | 适用场景 |
|------|------|---------|
| 字符串 style | `style="color: blue"` | 简单静态样式 |
| 字符串 style（多属性） | `style="color: blue; font-size: 18px"` | 常规内联样式 |
| Rust 表达式 | `style={format!("color: {}", color)}` | 动态样式 |

> **设计取舍：** 字符串 style 更简单、更接近 HTML 原生写法。如果将来需要更细粒度的样式控制，Leptos 也提供了 `style` 属性的对象形式支持（在 Leptos 0.7+ 的某些版本中）。

### style 属性和 CSS 的关系

一个重要的认识：**`style` 属性不等于 CSS。** 它是"内联样式"（inline style），优先级最高但最不利于维护：

```
CSS 优先级层级（从低到高）
┌────────────────────────────────────┐
│ 用户代理样式（浏览器默认）              │
│ 外部样式表（.css 文件）                │
│ <style> 块（嵌入 HTML）              │
│ style 属性（内联样式） ← 这是 view! 的  │
│ !important（尽量避免）                │
└────────────────────────────────────┘
```

在真实项目中，建议把大部分样式放在 CSS 文件中，只在需要动态控制少量样式时用 `style` 属性。

---

## 3. 自闭合标签 — `<br/>` 和 `<img/>` 的写法

HTML 中有一些元素没有内容（不能包含子元素），称为"空元素"（void elements）：

```html
<br>      <!-- 换行 -->
<img>     <!-- 图片 -->
<input>   <!-- 输入框 -->
<hr>      <!-- 水平分割线 -->
<meta>    <!-- 元数据 -->
```

在 HTML5 中，这些标签可以不写闭合斜杠：`<br>` 是合法的。但在 `view!` 宏中，**建议始终使用自闭合形式**：

```rust
<br/>      // ✅ 推荐
<img/>     // ✅ 推荐
```

**为什么？** 因为 `view!` 宏解析时需要明确知道标签的边界。自闭合形式 `<br/>` 让宏一眼就看出这是一个没有子元素的标签，不需要寻找匹配的 `</br>` 结束标签。

> **规则：** 在 `view!` 中，所有空元素都应该写成自闭合形式 `<元素名/>`。容器元素使用 `<元素名>...</元素名>` 形式。

---

## 4. `<a>` 链接和 `<img>` 图片 — 两个最常用的非文本元素

### `<a>` — 超链接

```rust
<a href="https://leptos.dev" target="_blank">"访问 Leptos 官网"</a>
```

`<a>` 元素的核心属性：

| 属性 | 作用 | 示例值 |
|------|------|--------|
| `href` | 链接目标 URL | `"https://..."` 或 `"/about"` |
| `target` | 在哪里打开 | `"_blank"`（新标签页）、`"_self"`（当前页） |
| `rel` | 关系描述 | `"noopener noreferrer"`（安全选项） |

**target="_blank" 的安全注意事项：** 在新标签页打开链接时，为了安全，通常应该同时设置 `rel="noopener noreferrer"`。这防止新页面通过 `window.opener` 访问原页面的上下文：

```rust
<a
    href="https://example.com"
    target="_blank"
    rel="noopener noreferrer"
>
    "安全的外部链接"
</a>
```

### `<img>` — 图片

```rust
<img src="https://placehold.co/200x100" alt="占位图片"/>
```

`<img>` 元素的核心属性：

| 属性 | 作用 | 是否必须 |
|------|------|---------|
| `src` | 图片来源 URL | **必须** |
| `alt` | 替代文本（图片加载失败时显示） | **必须**（无障碍要求） |
| `width` | 宽度（像素或百分比） | 可选 |
| `height` | 高度（像素或百分比） | 可选 |

**为什么 `alt` 是必须的？** 这不仅是 HTML 规范的要求，更是无障碍访问（a11y）的基本要求。视障用户依赖屏幕阅读器读取 `alt` 文本来理解图片内容。即使图片没有信息价值，也应该写 `alt=""`（空 alt）来让屏幕阅读器跳过它。

---

## 5. `view!` 与原生 DOM API 的对比

了解 `view!` 的便利性，最好的方式是对比如果不使用 `view!` 要怎么做：

| 操作 | 原生 DOM API (JS) | Leptos view! |
|------|-------------------|-------------|
| 创建元素 | `document.createElement('h1')` | `<h1>` |
| 设置 id | `element.id = 'title'` | `id="title"` |
| 设置 class | `element.className = 'heading'` | `class="heading"` |
| 设置 style | `element.style.color = 'blue'` | `style="color: blue"` |
| 设置文本 | `element.textContent = 'Hello'` | `"Hello"` |
| 创建链接 | `const a = document.createElement('a'); a.href = '...'` | `<a href="...">` |
| 创建图片 | `const img = document.createElement('img'); img.src = '...'` | `<img src="..."/>` |
| 添加事件 | `element.addEventListener('click', fn)` | `on:click=fn` |

**`view!` 的优势：** 声明式、结构清晰、编译期类型检查。原生 DOM API 是命令式的——你每一步都要手动操作，任何错误（拼错属性名、忘记 appendChild）都只能在运行时发现。

---

## 回顾总结

| 知识点 | 规则 | 示例 |
|--------|------|------|
| 属性语法 | 跟 HTML 完全一致，无驼峰转换 | `class="..."` 而不是 `className` |
| style 属性 | 字符串形式，跟 HTML 一样 | `style="color: blue"` |
| 自闭合标签 | 始终使用 `<元素名/>` | `<br/>`、`<img/>`、`<input/>` |
| `<a>` 链接 | 标准 HTML 属性 | `href`、`target`、`rel` |
| `<img>` 图片 | `src` 必须，`alt` 必须 | `src="url" alt="描述"` |
| 动态属性 | 用 `{ }` 包裹 Rust 表达式 | `id={my_var}` |

**一通百通的核心：** `view!` 中的 HTML 元素和属性，就是你早已熟悉的 HTML——没有 React 那样的抽象层（className、htmlFor、驼峰事件名），没有虚拟 DOM，没有 JSX 的怪癖。`view!` 的 HTML 语法是"透明"的，它直接映射到真实 DOM 元素和属性。你已有的 HTML 知识在这里 100% 可用，只是现在用 Rust 来表达它。
