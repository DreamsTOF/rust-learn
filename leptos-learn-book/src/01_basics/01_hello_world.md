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
