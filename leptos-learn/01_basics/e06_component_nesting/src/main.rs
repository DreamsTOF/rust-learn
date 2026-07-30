// ============================================================
// 练习 e06: 组件嵌套 — 在组件中调用子组件
//
// 核心知识点:
//   - 组件嵌套: 在 view! 中通过 <子组件/> 调用子组件
//   - 布局拆分: 将页面拆分为 Header / Main / Footer 三部分
//
// 难度: ⭐⭐ (补全约 50%，关键位置有 TODO)
// ============================================================

use leptos::prelude::*;

// TODO: 创建 Header 组件 — 渲染页面顶部
// 提示: 使用 #[component] 标记，返回 <header> 元素
// 完成度: 组件结构已给出
#[component]
fn Header() -> impl IntoView {
    view! {
        /* TODO: 添加 <header> 和 <h1> 元素 */
    }
}

// TODO: 创建 Main 组件 — 渲染页面主体内容
// 提示: 返回 <main> 元素
// 完成度: 组件结构已给出
#[component]
fn Main() -> impl IntoView {
    view! {
        /* TODO: 添加 <main> 和 <p> 元素 */
    }
}

// TODO: 创建 Footer 组件 — 渲染页面底部信息
// 提示: 返回 <footer> 元素
// 完成度: 组件结构已给出
#[component]
fn Footer() -> impl IntoView {
    view! {
        /* TODO: 添加 <footer> 和 <small> 元素 */
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 在 view! 中依次嵌套 Header、Main、Footer 三个子组件
    // 提示: 使用 <组件名/> 语法调用子组件
    view! {
        <Header/>
        <Main/>
        <Footer/>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[component]
// fn Header() -> impl IntoView {
//     view! {
//         <header><h1>"组件嵌套练习"</h1></header>
//     }
// }
//
// #[component]
// fn Main() -> impl IntoView {
//     view! {
//         <main><p>"这是主体内容区域"</p></main>
//     }
// }
//
// #[component]
// fn Footer() -> impl IntoView {
//     view! {
//         <footer><small>"© 2026 Leptos 教程"</small></footer>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Header/>
//         <Main/>
//         <Footer/>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// 知识点:
// 1. 组件嵌套: 在父组件的 view! 中，用 <子组件名/> 或 <子组件名></子组件名> 调用
// 2. 布局拆分: 将页面拆分成逻辑独立的小组件，便于维护和复用
// 3. 子组件不需要额外注册，只要在作用域内定义了即可使用
// </details>
