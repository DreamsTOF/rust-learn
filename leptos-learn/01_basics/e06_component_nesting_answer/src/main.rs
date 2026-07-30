// ============================================================
// 练习 e06: 组件嵌套 — 参考答案
//
// 核心知识点:
//   - 组件嵌套: 在 view! 中通过 <子组件/> 调用子组件
//   - 布局拆分: 将页面拆分为 Header / Main / Footer 三部分
// ============================================================

use leptos::prelude::*;

#[component]
fn Header() -> impl IntoView {
    view! {
        <header><h1>"组件嵌套练习"</h1></header>
    }
}

#[component]
fn Main() -> impl IntoView {
    view! {
        <main><p>"这是主体内容区域"</p></main>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer><small>"© 2026 Leptos 教程"</small></footer>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Header/>
        <Main/>
        <Footer/>
    }
}

fn main() {
    mount_to_body(Exercise);
}
