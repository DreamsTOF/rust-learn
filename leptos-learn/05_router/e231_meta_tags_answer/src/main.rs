// ============================================================
// 练习 e231 — meta_tags — 参考答案
// ============================================================

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <Meta name="description" content="Leptos 学习教程 - 路由与 Meta 标签示例"/>
        <Meta name="keywords" content="leptos, rust, wasm, web"/>
        <h2>"首页"</h2>
        <p>"查看 <head> 中的 meta 标签"</p>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <Meta name="description" content="关于页面 - 学习 Leptos Meta 组件"/>
        <Meta name="author" content="Leptos Learner"/>
        <h2>"关于"</h2>
        <p>"每个页面可以有不同的 Meta 标签"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <h1>"e231: Meta 标签"</h1>
            <nav>
                <A href="/">"首页"</A>
                " | "
                <A href="/about">"关于"</A>
            </nav>
            <Routes fallback=|| "页面未找到">
                <Route path=path!("") view=Home/>
                <Route path=path!("about") view=About/>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
