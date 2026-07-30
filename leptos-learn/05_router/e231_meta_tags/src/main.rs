// ============================================================
// 练习 e231: Meta 标签 (meta_tags)
//
// 目标: 使用 leptos::meta::Meta 组件设置页面 meta 标签
//
// 难度: ⭐⭐
// 核心知识点: Meta 组件、SEO 基础标签
// ============================================================

// TODO: 导入 leptos、leptos_meta 和 leptos_router
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

// TODO: 首页 — 包含 <Meta> 标签
// 使用 <Meta name="description" content="..."/> 设置描述
// 使用 <Meta name="keywords" content="..."/> 设置关键词
#[component]
fn Home() -> impl IntoView {
    view! {
        <Meta name="description" content="Leptos 学习教程 - 路由与 Meta 标签示例"/>
        <Meta name="keywords" content="leptos, rust, wasm, web"/>
        <h2>"首页"</h2>
        <p>"查看 <head> 中的 meta 标签"</p>
    }
}

// TODO: 关于页 — 不同的 Meta 标签
// 使用 <Meta name="description" content="..."/>
// 使用 <Meta name="author" content="..."/>
#[component]
fn About() -> impl IntoView {
    view! {
        <Meta name="description" content="关于页面 - 学习 Leptos Meta 组件"/>
        <Meta name="author" content="Leptos Learner"/>
        <h2>"关于"</h2>
        <p>"每个页面可以有不同的 Meta 标签"</p>
    }
}

// TODO: 补全路由配置
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
