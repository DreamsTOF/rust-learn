# 练习 06: 组件嵌套

## 为什么要学这个

你已经学会定义组件了。但真正的应用不是由孤立的组件构成的——**组件需要互相嵌套**，组成更大的结构。

这一节回答三个问题，它们将终结你对"组件化"这个概念的疑问：

1. **为什么要把一个页面拆成 Header / Main / Footer？** — 布局模式是所有 UI 框架的共同起点
2. **组件嵌套和 HTML 元素嵌套有什么异同？** — 语法一样，但概念不同
3. **子组件需要注册吗？** — 为什么组件定义在同作用域下就能直接用

这三个问题理解清楚，你就彻底掌握了组件组合（component composition）——这是构建任意复杂 UI 的方法论。

---

## 从问题出发

一个典型的网页布局：

```
┌─────────────────────────────────────┐
│  <header>                           │
│    <h1>网站标题</h1>                 │
│    <nav>导航链接</nav>               │
│  </header>                          │
├─────────────────────────────────────┤
│  <main>                             │
│    <article>文章内容</article>       │
│  </main>                            │
├─────────────────────────────────────┤
│  <footer>                           │
│    <small>版权信息</small>          │
│  </footer>                          │
└─────────────────────────────────────┘
```

这是 Web 上最常见的布局模式——Header / Main / Footer。在前面的练习中，你可能会这样写：

```rust
#[component]
fn Page() -> impl IntoView {
    view! {
        <header><h1>"网站标题"</h1></header>
        <main><p>"主体内容"</p></main>
        <footer><small>"版权信息"</small></footer>
    }
}
```

这个写法现在没问题。但当页面变复杂后，`Page` 组件的 `view!` 会越来越长，越来越难以管理。

**组件嵌套的思路：** 把 `header`、`main`、`footer` 分别写成独立的组件，然后在 `Page` 中组合它们。

```rust
#[component]
fn Header() -> impl IntoView {
    view! { <header><h1>"网站标题"</h1></header> }
}

#[component]
fn Main() -> impl IntoView {
    view! { <main><p>"主体内容"</p></main> }
}

#[component]
fn Footer() -> impl IntoView {
    view! { <footer><small>"版权信息"</small></footer> }
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <Header/>
        <Main/>
        <Footer/>
    }
}
```

**区别在哪？** 从最终渲染结果来看，两种写法生成的 DOM 完全一样。但第二种写法做了**逻辑拆分**——`Page` 组件不再关心 header 里是什么内容，它只负责说"这里有 header、main 和 footer"。

---

## 1. 布局拆分 — 为什么要分而治之？

### 单一职责原则

你熟悉的"函数应该只做一件事"（单一职责原则），同样适用于组件。

```
没有拆分的 Page 组件            拆分的 Page 组件
┌────────────────────┐        ┌────────────────────┐
│ 渲染 header 内容   │        │ 声明布局：         │
│ 渲染 main 内容      │        │ ├─ Header 组件    │
│ 渲染 footer 内容    │        │ ├─ Main 组件      │
│ 处理 main 的业务逻辑│        │ └─ Footer 组件    │
│ 处理 footer 的业务  │        │                    │
│ 处理 header 的导航  │        │ 每个子组件独立：     │
│ ... 修改一个地方    │        │ ├─ Header: 只做导航 │
│     影响所有逻辑    │        │ ├─ Main: 只做内容  │
└────────────────────┘        │ └─ Footer: 只做页脚│
                                └────────────────────┘
```

拆分的组件各自专注自己的职责：
- `Header` 只关心导航和标题
- `Main` 只关心主体内容
- `Footer` 只关心底部信息

修改 header 的样式时，你不会误改到 main 的代码。

### 可维护性的提升

| 场景 | 没有拆分 | 拆分成组件 |
|------|---------|-----------|
| 修改导航栏 | 从几百行代码中找到 header 部分 | 直接找到 `Header` 组件 |
| 复用页脚 | 复制粘贴 | 在其他页面中使用 `<Footer/>` |
| 测试 | 需要渲染整个页面 | 可以单独渲染 `Footer` 组件 |
| 团队协作 | 多人改同一个文件的不同部分，容易冲突 | 各自改自己的组件文件 |

### 可复用的组件

一旦把布局拆分成组件，这些组件就可能被其他页面复用：

```rust
// 主页
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Header/>
        <HomeMainContent/>
        <Footer/>
    }
}

// 关于页面
#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <Header/>
        <AboutMainContent/>
        <Footer/>
    }
}
```

`Header` 和 `Footer` 被两个页面共享——这就是组件复用的威力。你定义一次，用到无数地方。

---

## 2. 元素嵌套 vs 组件嵌套 — 相同语法，不同概念

### 语法上相同

```rust
// 元素嵌套
view! {
    <div>
        <p>"文本"</p>
    </div>
}

// 组件嵌套
view! {
    <Header>
        <Navigation/>
    </Header>
}
```

表面上看，都是"一个标签包含另一个标签"。

### 概念上不同

| | HTML 元素 | 自定义组件 |
|---|---|---|
| 本质 | 浏览器原生支持的 DOM 节点 | Leptos 框架管理的函数调用 |
| 创建方式 | 宏直接生成 DOM 元素 | 宏展开为函数调用 + Props 结构体 |
| 生命周期 | 由浏览器管理 | 由 Leptos 运行时管理 |
| 状态 | 无状态 | 可以有信号、effect 等 |
| 子元素 | 浏览器 DOM 子节点 | 传入的 children prop（概念上） |

**关键区别：** `<div>` 告诉浏览器"创建一个 div 节点"；`<Header>` 告诉 Leptos 运行时"调用 Header 函数，把结果插入到这个位置"。

从编译的角度看：

```rust
// view! 中的 <Header/>
// 编译后大致等价于
Header().into_view()

// view! 中的 <div>
// 编译后大致等价于
leptos::html::div()
```

一个是函数调用，一个是 DOM 工厂函数。你的 `<Header/>` 在 DOM 中最终会渲染成 `<header>` 元素（如果 Header 返回 `<header>`），但框架知道这是一个组件边界。

### 组件边界的意义

组件嵌套创建了**组件边界**（component boundary）：

```
组件树                                     最终 DOM 树
┌─ App                                    ┌─ <main>
│  ├─ Header  ─── 组件边界 ───┐           │  ├─ <header>
│  │                          │           │  │  └─ <h1>标题</h1>
│  │  ┌─ <header>             │  展开     │  ├─ <section>
│  │  │  └─ <h1>标题</h1>    │  ──────►  │  │  └─ <p>内容</p>
│  │  └─                      │           │  └─ <footer>
│  ├─ Main ──── 组件边界 ────┤           │     └─ <small>版权</small>
│  │  └─ <section>...</section>│          └─
│  └─ Footer ── 组件边界 ────┤
│     └─ <footer>...</footer>  │
└─                             ┘
```

组件边界的作用：

1. **错误隔离** — 一个组件出错不影响其他组件（React 的 Error Boundary 概念，Leptos 也有类似机制）
2. **更新范围** — 响应式更新可以在组件边界处停下，不需要穿透整个组件树
3. **逻辑封装** — 组件内部的信号、状态对外部不可见

---

## 3. 子组件需要注册吗？ — 作用域的魔法

### 答案：不需要

在 Leptos 中，定义一个组件后，只要在同一个作用域（或外层作用域）中，就可以直接在 `view!` 中使用：

```rust
// 定义 Header 组件
#[component]
fn Header() -> impl IntoView {
    view! { <header><h1>"标题"</h1></header> }
}

// 在另一个组件中直接使用，不需要额外注册
#[component]
fn Page() -> impl IntoView {
    view! {
        <Header/>  // ✅ 直接用
    }
}
```

**为什么不需要注册？**

因为 `#[component]` 宏生成的代码已经包含了组件所需的全部信息——生成的结构体、IntoView 实现等都在同一个 crate 的作用域内。当 `view!` 宏遇到 `<Header/>` 时：

1. 它知道 `Header` 是一个 Rust 标识符
2. 查找名为 `Header` 的组件（实际上是查找它生成的 Props 结构体）
3. 在编译期将其展开为对应的函数调用

**这跟 React 不同：** 在 React 中，你需要导入组件文件后才能在 JSX 中使用：

```jsx
// React: 需要显式导入
import Header from './Header';

function Page() {
    return (
        <Header/>  // 导入后才能用
    );
}
```

而 Leptos 中，如果组件定义在同一个文件或同一个模块中，直接可用。如果组件定义在其他模块中，跟普通的 Rust 函数一样——需要 `use` 导入。

### Rust 模块系统中的组件

```rust
// components.rs
#[component]
pub fn Header() -> impl IntoView { ... }

// main.rs
use crate::components::Header;

#[component]
fn Page() -> impl IntoView {
    view! { <Header/> }
}
```

因为组件本质上是 Rust 函数/结构体，所以 Rust 的模块规则完全适用——`pub` 公开、`use` 导入。没有任何框架特有的"组件注册表"概念。

---

## 4. 组件组合的模式

### 兄弟组件

```rust
view! {
    <Header/>
    <Main/>
    <Footer/>
}
```

三个组件平级排列，形成从上到下的流式布局。

### 父子组件（嵌套）

```rust
#[component]
fn Layout() -> impl IntoView {
    view! {
        <section class="app">
            <Header/>
            <Main>
                <Article/>
            </Main>
            <Footer/>
        </section>
    }
}
```

组件可以嵌套任意深度，就像 HTML 元素一样。

### 组件作为"积木"

```
学习路径：从元素到应用
─────────────────────────────────────────────

练习 02-03: 掌握单个元素和属性
    │
    ▼
练习 04:    掌握元素嵌套（树形结构）
    │
    ▼
练习 05:    掌握组件定义（可复用的 UI 单元）
    │
    ▼
练习 06:    掌握组件嵌套（组合成页面）
    │
    ▼
后续:       响应式 + 事件 + 路由 = 完整应用
```

组件嵌套是这条学习路径上最关键的一步：它把"你会写组件"变成了"你会用组件构建应用"。

---

## 回顾总结

| 知识点 | 核心思想 | 要点 |
|--------|---------|------|
| 布局拆分 | 将页面拆分成职责单一的组件 | Header/Main/Footer 是经典模式 |
| 组件嵌套 | 在 `view!` 中用 `<子组件/>` 组合 | 语法跟 HTML 元素嵌套一样 |
| 组件边界 | 每个组件是一个独立的逻辑单元 | 更新、错误、状态都在边界处隔离 |
| 无需注册 | 组件在作用域内即可直接使用 | Rust 的模块规则自然适用 |
| 可复用 | 拆分后的组件可在多个页面共享 | Header 和 Footer 是典型复用案例 |
| 元素 vs 组件 | 语法相同但概念不同 | 元素→DOM 节点；组件→函数调用 |

**一通百通的核心：** 组件嵌套就是你一直在等的那个"啊哈"时刻。

在前五节中，你在不断地积累"零件"：
- 文本节点（02）
- HTML 元素和属性（03）
- 元素嵌套结构（04）
- 组件定义（05）

而组件嵌套（06）让你把这些零件**组装起来**。从此，你的思维不再停留在"这一个 `<div>` 怎么写"，而是提升到"这个页面由哪几个部分组成，每个部分交给哪个组件负责"。

这就是组件化思维的全部。剩下的只是练习。
