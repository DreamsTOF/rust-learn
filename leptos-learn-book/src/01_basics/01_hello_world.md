# 练习 01: Hello World

每个 Leptos 应用都有三个基本要素：一个**挂载点**、一个**组件**、一个**视图**。这个练习把这三者组合成最简单的可运行程序。

---

## 概念讲解

### 1. `mount_to_body` — 挂载点

`mount_to_body` 是 Leptos 应用的**入口函数**。它的作用是把一个组件渲染到网页的 `<body>` 元素中。

```rust
fn main() {
    mount_to_body(App);
}
```

这里有几个值得注意的点：

- **`main` 函数是 Rust 程序的入口**，它在这里的角色就是启动 Leptos 应用。调用 `mount_to_body` 后，控制权交给 Leptos 运行时，`main` 的任务就完成了。
- **`mount_to_body` 不是一个宏，就是一个普通函数**。它接收一个**组件**作为参数。关于组件是什么，下一节说明。
- **挂载（mount）** 这个词来源于 DOM 操作：将一个元素/组件插入到 DOM 树中。这里的 `<body>` 就是挂载目标。

> 在 Leptos 0.7 之前的版本中，入口方式略有不同，需要手动调用 `mount_to_body` 并传入 `|| App()` 闭包。0.8 版本简化了这一点，直接传组件名即可。

---

### 2. `#[component]` — 组件标记

`#[component]` 是一个**属性宏**（attribute macro），它把一个普通的 Rust 函数标记为 Leptos 组件：

```rust
#[component]
fn App() -> impl IntoView {
    // ... 返回视图
}
```

**组件是什么？**

组件是 Leptos 应用的**基本构建块**。每个组件负责渲染 UI 的一个独立部分。你可以把组件看作一个"UI 工厂"：它接收一些输入（属性），返回一些输出（视图）。整个 Leptos 应用就是由组件组成的树状结构。

```rust
// 最简单的组件
#[component]
fn Hello() -> impl IntoView {
    view! { <p>"你好"</p> }
}

// 带标签属性的组件（后面练习会涉及）
#[component]
fn Greeting(name: String) -> impl IntoView {
    view! { <p>"你好，" {name}</p> }
}
```

**几点规则：**

- **组件名必须用 PascalCase**（首字母大写驼峰），例如 `App`、`HelloWorld`、`UserProfile`。Leptos 用大小写区分组件和 HTML 元素：`<App/>` 是组件，`<div>` 是原生 HTML 元素。
- **返回值类型是 `impl IntoView`**。`IntoView` 是一个 trait，表示"可以转换成视图的东西"。`view!` 宏返回的类型实现了这个 trait，所以直接写 `impl IntoView` 即可。你不需要关心具体返回什么类型。
- **`#[component]` 宏在背后做了一些工作**：它会给函数生成辅助代码，使得该函数可以在 `view!` 中被当作标签调用。如果没有这个宏，`<App/>` 在 `view!` 中是不合法的。

> `IntoView` 是 Leptos 的核心 trait。它类似于标准库的 `IntoIterator`，但作用是将各种类型统一转换为框架的视图表示。不仅 `view!` 宏的输出实现了它，像字符串、数字等基本类型也实现了它，这使得在 `view!` 中可以灵活使用 `{变量}` 插值。

---

### 3. `view!` 宏 — 声明式 UI

`view!` 是 Leptos 中最常用的**声明式 UI 宏**。它让你用类似 HTML 的语法来描述界面，而不是手动调用 DOM API：

```rust
view! {
    <p>"Hello, Leptos!"</p>
}
```

**这是 Leptos 最核心的 API。** 理解它的语法规则，就能读懂和编写绝大部分 Leptos 代码。

**基本规则：**

1. **标签语法**：`<p>`、`<div>`、`<button>` 等 HTML 标签直接在 `view!` 中使用，写法与 HTML 几乎一致。

2. **文本节点必须用双引号**：注意上面例子中的 `"Hello, Leptos!"`，它被双引号包裹。这不是 Rust 字符串语义上的要求，而是 `view!` 宏的语法约定——**只有用 `"..."` 包裹的内容才被视为文本节点**。如果写成：
   ```rust
   <p>Hello, Leptos!</p>  // ❌ 编译错误
   ```
   宏会把 `Hello,` 解析成 Rust 代码标识符，导致编译失败。所以**文本节点始终加双引号**。

3. **Rust 表达式嵌入**：用 `{ }` 可以在 `view!` 中嵌入任意 Rust 表达式：
   ```rust
   <p>{42}</p>                         // 数字
   <p>{"你好"}</p>                      // 字符串（等同于 "你好"）
   <p>{format!("1 + 1 = {}", 2)}</p>   // 函数调用
   ```

4. **`view!` 也是表达式**：它返回一个实现了 `IntoView` 的值，可以直接作为函数返回值：
   ```rust
   #[component]
   fn App() -> impl IntoView {
       view! { <p>"Hello, Leptos!"</p> }
   }
   ```

**与 JSX 的区别：**

如果你有 React 经验，`view!` 看起来很像 JSX，但有一个关键差异——JSX 的属性是"可选的"（某些属性有默认值），而 `view!` 中的属性会严格按照 HTML 规范处理。另外，`view!` 是 Rust 宏而非 JavaScript 语法扩展，所以它受 Rust 语法约束，例如元素必须正确闭合。

---

### 组合起来

本章三个知识点的组合关系：

```
mount_to_body(App)     ← 入口：把 App 挂到 <body>
    │
    └── App 组件       ← 定义：#[component] fn App() -> impl IntoView
           │
           └── view!   ← UI：用声明式语法描述界面
```

一个 Leptos 应用的启动流程：`main` → 调用 `mount_to_body` → 传入根组件 → 根组件内的 `view!` 生成 DOM → 渲染到页面。

---

