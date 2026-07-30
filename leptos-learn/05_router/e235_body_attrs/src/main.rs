// ============================================================
// 练习 e235: Body 属性 (body_attrs)
//
// 目标: 使用 leptos_meta::Body 组件为路由页面设置不同的 <body> 属性
//
// 难度: ⭐⭐
// 核心知识点: Body, 路由级 body 属性
// ============================================================

use leptos::prelude::*;
use leptos_meta::Body;
use leptos_router::components::*;
use leptos_router::path;

// 首页组件 — 设置 body class 和背景色
#[component]
fn Home() -> impl IntoView {
    view! {
        <>
            <Body {..} class="home-page" style="background: #f5f5f5"/>
            <h1>"首页"</h1>
            <p>"当前 body 具有 'home-page' class 和浅灰色背景"</p>
            <A href="/about">"关于页面"</A>
        </>
    }
}

// 关于页面组件 — 设置不同的 body class 和背景色
#[component]
fn About() -> impl IntoView {
    view! {
        <>
            <Body {..} class="about-page" style="background: #e3f2fd"/>
            <h1>"关于页面"</h1>
            <p>"当前 body 具有 'about-page' class 和浅蓝色背景"</p>
            <A href="/">"返回首页"</A>
        </>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "页面未找到">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/about") view=About/>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
