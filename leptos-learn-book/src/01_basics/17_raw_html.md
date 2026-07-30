# 练习 17: 原始 HTML 渲染

## 为什么要学这个

前面所有练习都用 `view!` 构建 UI。你写 `<p>"Hello"</p>`，Leptos 自动把文本安全地插入 DOM——特殊字符（`<`、`>`、`&`）会被转义。

这个自动转义是安全的基础。但**如果"安全"本身变成了障碍呢？**

考虑这些场景：

- 你从 Markdown 解析器得到了一段 HTML
- 后端 CMS 返回了富文本内容（含 `<strong>`、`<ul>` 等标签）
- 你需要在页面中嵌入一段预先渲染好的 SVG

`view!` 的自动转义会把所有标签变成纯文本——`<strong>重要</strong>` 会变成屏幕上的一行原文，而不是加粗的文字。

**核心矛盾：** 框架为了保护你，默认把一切视为文本。但你也需要一个"信任我，让我直接插入 HTML"的通道。这个通道就是 `inner_html`。

---

## 从问题出发

先看一个具体的例子。假设有一段合法的 HTML 字符串：

```rust
let content = "<h2>标题</h2><p>段落内容</p>";
```

如果放在 `view!` 里：

```rust
view! {
    <p>{content}</p>
}
```

浏览器显示的是：`<h2>标题</h2><p>段落内容</p>`——**全部作为纯文本**，所有标签可见但不生效。

这是框架有意为之：**默认安全**。`{content}` 在 DOM 中等价于 `textContent` 赋值，所有 HTML 特殊字符被转义。

但你真的想让它生效。这时就需要 `inner_html`：

```rust
view! {
    <div inner_html=content></div>
}
```

这个 `<div>` 的 innerHTML 被直接设置为 `content` 的值，浏览器会解析其中的 HTML 标签并渲染——标题和段落就正常显示了。

> **一句话：** `{expr}` = `textContent`（安全，自动转义），`inner_html=expr` = `innerHTML`（强大，有风险）。

---

## `inner_html` 的工作原理

### 它绕过了什么？

`view!` 宏生成的代码，对于 `{expr}` 这种插入方式，会在运行期调用 DOM 的 `textContent` setter 或 `createTextNode`。这些 API 会将 `<` 转义为 `&lt;`，`>` 转义为 `&gt;`，从而确保浏览器不会将内容中的标签解释为 HTML。

而 `inner_html` 直接编译为 DOM 元素的 `innerHTML` setter。这个 setter 会把字符串传给浏览器的 HTML 解析器——**标签就是标签，文本就是文本，没有任何转义**。

```
你写的                          底层 DOM 操作                  浏览器看到
─────────────────────────────────────────────────────────────────────
{p: "Hello <World>"}    →    textContent = "Hello <World>"   →   Hello <World>
inner_html=p             →    innerHTML   = "Hello <World>"   →   Hello <World>
```

左边写的一模一样，底层操作不同，最终效果天差地别。

### 为什么 `inner_html` 不是默认行为？

因为**安全不是默认的，而是设计出来的**。

浏览器界的教训——jQuery 的 `.html()` 方法太方便了，几乎所有开发者都用它来插入内容。直到 XSS 攻击成为 Web 第一大安全漏洞，大家才意识到："把任意字符串塞进 innerHTML" 约等于"帮黑客执行代码"。

Leptos 的默认转义不是"多此一举"，而是吸取了整个 Web 生态二十年的教训。

---

## XSS —— 为什么你该在意

### 一个最简单的 XSS

假设你的应用允许用户输入姓名并显示在页面上。你用了 `inner_html`：

```rust
// 用户输入: <img src=x onerror="alert('XSS')">
view! {
    <p inner_html=user_input></p>
}
```

浏览器解析这个字符串时，会：

1. 创建一个 `<img>` 元素
2. `src="x"` 导致图片加载失败
3. `onerror` 事件触发，执行攻击者的 JavaScript

这段 JS 可以做什么？**一切**。偷 cookie、篡改页面、跳转到钓鱼网站、在后台发起请求……它拥有当前域名的全部权限。

### 自动转义如何阻止 XSS

```rust
// 同样的用户输入，用 view! 的 { } 插值
view! {
    <p>{user_input}</p>
}
```

`<img src=x onerror="alert('XSS')">` 被安全地显示为文本字符串。浏览器不会创建 `<img>` 元素，不会触发 `onerror`，因为整个内容被当作**文本节点**处理。

> **关键认知：** XSS 不是"用户故意使坏"才发生。绝大多数 XSS 漏洞源于数据中包含预料之外的 HTML——第三方 API 返回值变了、数据库里的旧数据有未转义的内容、Markdown 渲染器产出了意料之外的标签。**自动转义是一种保险，不是在防坏人，而是在防意外。**

---

## 什么时候可以用 `inner_html`

| 场景 | 安全性 | 建议 |
|------|--------|------|
| 你自己写死的常量 HTML | ✅ 安全 | 可以直接用，但想想是否能用 view! 替代 |
| Markdown 渲染器输出 | ⚠️ 取决于渲染器 | 使用经过安全测试的渲染器（如 comrak 的 safe 模式） |
| CMS 后端产出的富文本 | ⚠️ 取决于 CMS | 确保 CMS 输出时做了 HTML 消毒，或在前端用 DOMPurify |
| 用户提交的内容 | ❌ 不安全 | 绝不要直接用。如果需要，先消毒（sanitize） |
| URL 参数解析出的内容 | ❌ 不安全 | 同用户提交，绝不信任 URL 参数 |
| SVG / MathML 模板 | ✅ 安全（如果模板完全可控） | 确保没有任何插值点可能引入不可控内容 |

### 安全使用的黄金法则

**任何可能包含不可控字符的数据，在传给 `inner_html` 之前必须经过消毒（sanitize）。**

在前端，最常用的消毒库是 [DOMPurify](https://github.com/cure53/DOMPurify)，Leptos 生态中也建议集成类似的方案。消毒不是"可选的步骤"，而是使用 `inner_html` 的**前提条件**。

> **实用建议：** 90% 的情况下你不需要 `inner_html`。想要加粗文字？用 `<strong>` 组件。想要分段？用多个 `<p>` 组件。想要嵌入富文本？考虑用 Markdown 解析 + 组件化渲染。只有当你真的有一串「未知但可控」的 HTML 字符串时，才考虑 `inner_html`。

---

## `view!` 文本插值 vs `inner_html`

| 特性 | `{expr}` 文本插值 | `inner_html=expr` |
|------|------------------|-------------------|
| 底层操作 | `textContent` 或 `createTextNode` | `innerHTML` |
| HTML 标签 | 被转义为纯文本 | 被解析为真实 DOM |
| XSS 风险 | 无（天生安全） | 有（取决于内容来源） |
| 性能 | 快（纯文本操作） | 较慢（需要 HTML 解析 + DOM 树构建） |
| 适用场景 | 绝大多数文本展示 | 渲染预先格式化好的 HTML 字符串 |
| 使用方式 | `<p>{expr}</p>` | `<div inner_html=expr></div>` |

---

## 一个原则

```
view! 的默认行为（转义）＝ 框架的边界责任
inner_html                  ＝ 你主动承担安全责任的通道
```

`view!` 对你写的每个 `{expr}` 做转义，这是框架履行的安全承诺。而 `inner_html` 是这个承诺的显式豁免——你告诉框架："这段内容我负责，你别管。"

选择 `inner_html` 不是选择"更强大"，而是选择"我承担这个风险"。每次写 `inner_html`，都应该有一个对应的理由：不是把用户输入直接塞进去，而是"这是我信任的内容"。

这个权衡不是 Leptos 独有的——React 的 `dangerouslySetInnerHTML`、Vue 的 `v-html`、Angular 的 `[innerHTML]`，都是同一个模式。Leptos 没有把它叫做 `dangerouslySetInnerHTML`，但**你应该把它当做那么危险的东西来对待**。
