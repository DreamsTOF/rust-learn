# 练习 05: 组件定义

## 为什么要学这个

从本节开始，我们不再只是排列 HTML 元素，而是开始**定义自己的组件**。这是你从"用 HTML 写页面"到"用框架构建应用"的分水岭。

组件是 Leptos（以及所有现代前端框架）的核心抽象。理解组件定义，就是理解三个根本问题：

1. **为什么要把 UI 拆成一个个函数？** — 从"一次性的页面"到"可复用的 UI 单元"
2. **`#[component]` 对函数做了什么？** — 它不只是贴了一个标签
3. **组件参数（Props）和函数参数有什么不同？** — 为什么框架需要跟踪这些参数

如果前四节让你学会了"怎么用 `view!` 写 UI"，这一节让你学会"怎么把 UI 打包成可复用的组件"。

---

## 从问题出发

假设你在构建一个用户资料页，需要显示多个用户的卡片，每个卡片包含姓名和简介。

**没有组件时：**

```rust
#[component]
fn Page() -> impl IntoView {
    view! {
        <div>
            <h3>"Alice"</h3>
            <p>"热爱 Rust 和 Web 开发"</p>
        </div>
        <div>
            <h3>"Bob"</h3>
            <p>"全栈开发者，喜欢开源"</p>
        </div>
        // 每多一个用户，就复制粘贴一次同样的结构
    }
}
```

这很快变成噩梦：如果你要改卡片的样式，需要修改每一处复制粘贴的代码。如果要添加新功能（比如头像），每一处都要单独更新。

**组件化后：**

```rust
#[component]
fn ProfileCard(name: String, bio: String) -> impl IntoView {
    view! {
        <div class="profile-card">
            <h3>{name}</h3>
            <p>{bio}</p>
        </div>
    }
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <ProfileCard name={"Alice"} bio={"热爱 Rust 和 Web 开发"}/>
        <ProfileCard name={"Bob"} bio={"全栈开发者，喜欢开源"}/>
    }
}
```

**变化：** 卡片的 HTML 结构只写一次。数据从函数参数传入。如果要改结构，只改 `ProfileCard` 的定义就够了。

这就是组件化的核心价值：**复用 + 封装**。

---

## 1. 为什么 UI 需要"组件"这个抽象？

### 从函数类比

你每天都在用函数来组织代码：

```rust
// 不用函数
let x = 1 + 2;
let y = 3 + 4;
let z = 5 + 6;

// 用函数
fn add(a: i32, b: i32) -> i32 { a + b }
let x = add(1, 2);
let y = add(3, 4);
let z = add(5, 6);
```

函数让你把一段逻辑打包，给它一个名字，通过参数传入变化的部分。**组件就是 UI 的函数。**

| | 普通函数 | 组件 |
|---|---|---|
| 目的 | 逻辑复用 | UI 复用 |
| 输入 | 参数 | 属性（Props） |
| 输出 | 返回值 | 视图（DOM 节点） |
| 调用方式 | `fn_name(args)` | `<ComponentName props/>` |
| 调用时机 | 开发者控制 | 框架控制 |

### 组件的三个核心能力

1. **复用** — 同一套 UI 结构用不同数据渲染多次
2. **封装** — 组件内部的实现细节（HTML 结构、样式逻辑）不暴露给外面
3. **组合** — 组件可以包含其他组件，构成更大的 UI 单元

这三点的组合，让你能用"搭积木"的方式构建任意复杂的 UI。

---

## 2. `#[component]` 宏 — 它到底做了什么？

当你写：

```rust
#[component]
fn Greeting(name: String) -> impl IntoView {
    view! {
        <p>"你好，" {name} "！"</p>
    }
}
```

你看到的是一个普通的 Rust 函数，加上 `#[component]` 属性宏。**但框架看到的不是这样。**

### 宏展开后的变化

`#[component]` 宏在编译期执行以下转换：

```
你写的函数                          宏展开后
┌─────────────────────────┐       ┌──────────────────────────┐
│ fn Greeting(name: String)│  ─►  │ GreetingProps 结构体     │
│   -> impl IntoView       │       │   + 为它实现 IntoView    │
│ { ... }                  │       │   + 注册到组件系统       │
└─────────────────────────┘       │   + 保持原函数体不变     │
                                   └──────────────────────────┘
```

具体来说：

1. **生成 Props 结构体** — 函数的每个参数变成结构体的一个字段：
   ```rust
   // 自动生成（简化）
   struct GreetingProps {
       name: String,
   }
   ```

2. **为 Props 实现 IntoView** — 使得 `GreetingProps` 可以在 `view!` 中使用：
   ```rust
   // 自动生成（简化）
   impl IntoView for GreetingProps {
       fn into_view(self) -> View {
           // 调用原来的函数体
           Greeting(self.name)
       }
   }
   ```

3. **组件注册** — Leptos 的运行时知道有哪些组件可用

### 为什么需要这个宏？

Rust 没有"组件"这个概念，也没有办法直接把一个函数标记为"可供模板调用"。

宏是 Rust 中实现这种转换的标准方式——**它在编译期分析函数签名，自动生成框架需要的胶水代码**。

如果没有这个宏，你被迫手动写这些胶水代码：

```rust
// 手动写——繁琐且容易出错
struct GreetingProps { name: String }

impl IntoView for GreetingProps {
    fn into_view(self) -> View {
        // 复制函数体
        view! { <p>"你好，" {self.name} "！"</p> }
    }
}

fn Greeting(name: String) -> impl IntoView {
    view! { <p>"你好，" {name} "！"</p> }
}
```

`#[component]` 宏帮你自动完成了这个转换。

---

## 3. 组件属性（Props）— 为什么参数就是 Props？

### 最直接的映射

在 Leptos 中，组件的 Props 就是函数的参数：

```rust
// 定义
#[component]
fn Greeting(name: String) -> impl IntoView {
    view! { <p>"你好，" {name} "！"</p> }
}

// 使用
<Greeting name={String::from("Leptos")}/>
```

参数名就是属性名，参数类型就是属性类型。没有额外的 `Props` 结构体定义，没有 `#[derive(...)]`，没有属性验证注解。

### 对比 React

在 React 中，Props 是一个对象：

```jsx
// React
function Greeting(props) {
    return <p>你好，{props.name}！</p>;
}

// 或者解构
function Greeting({ name }) {
    return <p>你好，{name}！</p>;
}
```

在 Leptos 中，每个参数直接声明：

```rust
// Leptos
#[component]
fn Greeting(name: String) -> impl IntoView {
    view! { <p>"你好，" {name} "！"</p> }
}
```

**区别：** React 的 Props 是运行时对象，Leptos 的 Props 是编译期生成的结构体。这意味着 Leptos 可以在编译期做类型检查——你传了一个 `i32` 给需要 `String` 的参数，编译器直接报错。

### 多个参数

```rust
#[component]
fn ProfileCard(name: String, bio: String) -> impl IntoView {
    view! {
        <div class="profile-card">
            <h3>{name}</h3>
            <p>{bio}</p>
        </div>
    }
}

// 使用
<ProfileCard name={String::from("Rustacean")} bio={String::from("热爱 Rust 和 Web 开发")}/>
```

多个参数用逗号分隔，使用时用空格分隔属性（跟 HTML 属性一样）。

---

## 4. 为什么返回类型是 `impl IntoView`？

### 不知道返回什么类型

一个组件的 `view!` 块可能返回不同的类型：

```rust
#[component]
fn Greeting(name: Option<String>) -> impl IntoView {
    match name {
        Some(n) => view! { <p>"你好，" {n} "！"</p> },
        None => view! { <p>"你好，世界！"</p> },
    }
}
```

两个分支返回的类型是相同的吗？不一定——`<p>` 是 `HtmlElement<P>`，但如果将来改成了不同的元素，类型就变了。

`impl IntoView` 让你不需要关心具体类型。它说："我返回一个可以当作视图的东西。"这就是**类型擦除**——对外暴露统一的接口，隐藏内部类型。

### IntoView 的自动实现

`IntoView` 被广泛实现，使得组件可以返回各种类型：

```rust
// 返回字符串
fn SimpleText() -> impl IntoView { "Hello" }

// 返回数字
fn SimpleNumber() -> impl IntoView { 42 }

// 返回列表
fn ListItems() -> impl IntoView {
    vec![
        view! { <li>"一"</li> },
        view! { <li>"二"</li> },
        view! { <li>"三"</li> },
    ]
}
```

**为什么不是直接返回 `View` 类型？** 因为 `View` 是一个具体枚举类型，用 `impl IntoView` 可以给编译器更多优化空间（零成本抽象），同时让代码更灵活。

> **设计原则：** 返回 `impl IntoView` 而不是具体类型，遵循了 Rust 的"静多态"原则——调用者只知道"这是一个视图"，不知道具体是什么视图。组件和调用者之间通过 trait 契约解耦。

---

## 回顾总结

| 知识点 | 核心思想 | 语法要点 |
|--------|---------|---------|
| 组件化 | UI 打包成可复用的函数 | `#[component] fn Name() -> impl IntoView` |
| `#[component]` 宏 | 自动生成 Props 结构体和 IntoView 实现 | 添加在函数定义上 |
| Props | 函数的参数直接成为组件属性 | `fn Card(name: String, bio: String)` |
| 使用组件 | 在 `view!` 中用 `<组件名 属性="值"/>` | `<Greeting name="World"/>` |
| 返回值 | `impl IntoView` 类型擦除 | 返回任何可渲染的类型 |
| 属性动态化 | 用 `{ }` 传 Rust 表达式 | `name={my_var}` 或 `name={"直接写字符串"}` |

**一通百通的核心：** 组件就是一个"带标签的 Rust 函数"——通过 `#[component]` 宏，一个普通的 Rust 函数被赋予了三个新能力：

1. **声明式调用：** 在 `view!` 中用 XML 语法调用
2. **类型安全的 Props：** 参数类型在编译期检查
3. **框架管理生命周期：** Leptos 决定何时渲染、何时更新

你在后续练习中将看到的组件嵌套、布局拆分、信号响应等，都是在这个基础上扩展的。组件定义能力是所有高级用法的基础。
