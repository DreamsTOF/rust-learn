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

文件位置：`leptos-learn/01_basics/e01_hello_world/src/main.rs`

在项目根目录运行 `trunk serve` 启动：

```bash
cd leptos-learn/01_basics/e01_hello_world
trunk serve
```

然后在浏览器打开 `http://localhost:3001` 查看效果。

```rust
// ============================================================
// 练习 e01: Hello, Leptos! — 最简单的 Leptos 应用
//
// 核心知识点:
//   - mount_to_body: 将组件挂载到 <body>
//   - view! 宏: 编写声明式 UI
//   - #[component]: 标记组件函数
//
// 难度: ⭐ (填空题 — 每行都有 TODO 指引)
// ============================================================

use leptos::prelude::*;

// TODO: 使用 #[component] 属性标记此函数为组件
// 提示: 属性放在 fn 之前，组件名使用 PascalCase
// #[component]
fn Exercise() -> impl IntoView {
    // TODO: 在 view! 宏的 <p> 标签中显示 "Hello, Leptos!"
    // 提示: 文本内容用双引号包裹，例如 "文本"
    view! {
        <p>"Hello, Leptos!"</p>
    }
}

fn main() {
    // TODO: 使用 mount_to_body 将 Exercise 组件挂载到页面
    // 提示: mount_to_body(组件名)
    mount_to_body(Exercise);
}
```

---

## 参考答案

文件位置：`leptos-learn/01_basics/e01_hello_world_answer/src/main.rs`

<details>
<summary><strong>点击展开参考答案</strong></summary>

```rust
// ============================================================
// 练习 e01: Hello, Leptos! — 参考答案
//
// 核心知识点:
//   - mount_to_body: 将组件挂载到 <body>
//   - view! 宏: 编写声明式 UI
//   - #[component]: 标记组件函数
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <p>"Hello, Leptos!"</p>
    }
}

fn main() {
    mount_to_body(Exercise);
}
```

</details>

---
## 在线沙箱

### 练习沙箱

<details class="sandbox">
<summary>🟡 练习在线沙箱</summary>
<div>
<p>在浏览器中直接运行此练习，无需本地环境：</p>

<iframe
    src="https://codesandbox.io/embed/github/DreamsTOF/rust-learn/tree/main/leptos-learn/01_basics/e01_hello_world?view=editor+preview&expanddevtools=0&hidenavigation=1&fontsize=14"
    style="width:100%; height:500px; border:0; border-radius: 4px; overflow:hidden;"
    title="e01_hello_world-exercise"
    allow="accelerometer; ambient-light-sensor; camera; encrypted-media; geolocation; gyroscope; hid; microphone; midi; payment; usb; vr; xr-spatial-tracking"
    sandbox="allow-forms allow-modals allow-popups allow-presentation allow-same-origin allow-scripts"
></iframe>

<p>也可直接打开：<a href="https://codesandbox.io/s/github/DreamsTOF/rust-learn/tree/main/leptos-learn/01_basics/e01_hello_world" target="_blank">https://codesandbox.io/s/github/DreamsTOF/rust-learn/tree/main/leptos-learn/01_basics/e01_hello_world</a></p>
</div>
</details>

### 答案沙箱

<details class="sandbox">
<summary>🟡 答案在线沙箱</summary>
<div>
<p>参考答案的在线版本：</p>

<iframe
    src="https://codesandbox.io/embed/github/DreamsTOF/rust-learn/tree/main/leptos-learn/01_basics/e01_hello_world_answer?view=editor+preview&expanddevtools=0&hidenavigation=1&fontsize=14"
    style="width:100%; height:500px; border:0; border-radius: 4px; overflow:hidden;"
    title="e01_hello_world-answer"
    allow="accelerometer; ambient-light-sensor; camera; encrypted-media; geolocation; gyroscope; hid; microphone; midi; payment; usb; vr; xr-spatial-tracking"
    sandbox="allow-forms allow-modals allow-popups allow-presentation allow-same-origin allow-scripts"
></iframe>

<p>也可直接打开：<a href="https://codesandbox.io/s/github/DreamsTOF/rust-learn/tree/main/leptos-learn/01_basics/e01_hello_world_answer" target="_blank">https://codesandbox.io/s/github/DreamsTOF/rust-learn/tree/main/leptos-learn/01_basics/e01_hello_world_answer</a></p>
</div>
</details>
