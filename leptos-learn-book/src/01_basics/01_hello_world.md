# 练习 01: Hello World

## 概念讲解

这是 Leptos 框架中最简单的一个应用。通过这个练习，你将了解 Leptos 应用的三个基本要素：

### 1. `mount_to_body` — 挂载点

每个 Leptos 应用都需要一个入口点。`mount_to_body` 函数接受一个组件，并将其渲染到页面的 `<body>` 中：

```rust
fn main() {
    mount_to_body(App);
}
```

### 2. `#[component]` — 组件标记

`#[component]` 属性宏将普通的 Rust 函数标记为 Leptos 组件：

```rust
#[component]
fn App() -> impl IntoView {
    // ...
}
```

> **注意：** 组件名必须使用 **PascalCase** 命名规范。

### 3. `view!` 宏 — 声明式 UI

```rust
view! {
    <p>"Hello, Leptos!"</p>
}
```

---

## 练习代码

```rust
{{#include ../../../leptos-learn/sandbox/src/exercises/e01_hello_world.rs}}
```

---

## 参考答案

<details>
<summary><strong>点击展开参考答案</strong></summary>

```rust
{{#include ../../../leptos-learn/sandbox/src/exercises/e01_hello_world_answer.rs}}
```

</details>
---
## 本地沙箱

> 🚀 **只需一次启动**，即可预览所有练习。
> 在项目根目录运行：
> 
> **终端 1** — 编译器：
> ```bash
> cd leptos-learn/sandbox
> trunk serve
> ```
> 
> **终端 2** — 编辑器：
> ```bash
> node leptos-learn/sandbox/editor-server.js
> ```
> 
> 然后打开 mdbook，下面的 iframe 就是完整的分屏编辑器。
> 你也可以直接打开 `http://localhost:3002/?e=01` 全屏使用。

### 练习实时预览

<iframe
    src="http://localhost:3002/?e=01"
    style="width:100%; height:550px; border:1px solid #444c56; border-radius: 4px; background:#0d1117;"
    title="练习 01 实时预览"
></iframe>

### 答案实时预览

<details class="sandbox">
<summary>🟢 答案运行效果（点击展开）</summary>
<div>

<iframe
    src="http://localhost:3002/?e=01_answer"
    style="width:100%; height:550px; border:1px solid #444c56; border-radius: 4px; background:#0d1117;"
    title="练习 01 答案实时预览"
></iframe>

</div>
</details>
