// ============================================================
// 练习 e226: route_animation — 路由过渡动画
//
// 目标: 在路由切换时添加过渡动画效果
//
// 难度: ⭐⭐⭐
// 核心知识点: Router set_is_routing、CSS transition、过渡动画
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::path;

// 首页组件
#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p>"欢迎来到首页"</p>
    }
}

// 关于页面
#[component]
fn About() -> impl IntoView {
    view! {
        <h2>"关于"</h2>
        <p>"这是关于页面"</p>
    }
}

// 联系页面
#[component]
fn Contact() -> impl IntoView {
    view! {
        <h2>"联系"</h2>
        <p>"请联系我们"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建 is_routing 信号，通过 Router 的 set_is_routing 监听路由切换
    let (is_routing, set_is_routing) = signal(false);
    let main_class = move || {
        if is_routing.get() { "fading" } else { "" }
    };

    view! {
        <Router set_is_routing=set_is_routing>
            <nav>
                <A href="/">"首页"</A>
                <A href="/about">"关于"</A>
                <A href="/contact">"联系"</A>
            </nav>
            // 动态绑定 class，路由切换时添加 fading 类触发 CSS 过渡
            <main class=main_class>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/about") view=About/>
                    <Route path=path!("/contact") view=Contact/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码说明
// - `Router` 的 `set_is_routing` prop 接收 `SignalSetter<bool>`
// - 路由切换时 Router 自动设置信号为 true，切换完成后恢复 false
// - 动态 class 绑定：`is_routing` 为 true 时添加 `.fading` 类
// - CSS `transition: opacity 0.3s` 实现淡入淡出效果
// - `<A>` 组件实现声明式导航，避免页面刷新
//
// ### CSS
// ```css
// main { transition: opacity 0.3s; }
// main.fading { opacity: 0.2; }
// ```
//
// ### 升级方向
// 可以使用 View Transition API (`document.startViewTransition`) 实现更流畅的过渡，
// 设置 `<Routes transition=true>` 启用内置 View Transition 支持。
//
// </details>
