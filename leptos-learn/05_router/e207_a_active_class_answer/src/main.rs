// ============================================================
// Exercise 207 - Answer
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h2>"首页"</h2>
            <p>"欢迎来到首页！"</p>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            <h2>"关于"</h2>
            <p>"这是关于页面。"</p>
        </div>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <div>
            <h2>"联系方式"</h2>
            <p>"请联系我们。"</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <style>
                "\n    .active {\n        font-weight: bold;\n        color: #c00;\n    }\n"
            </style>
            <nav>
                <A href="/home" class:active>"首页"</A>
                " | "
                <A href="/about" class:active>"关于"</A>
                " | "
                <A href="/contact" class:active>"联系方式"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/home") view=Home/>
                    <Route path=path!("/about") view=About/>
                    <Route path=path!("/contact") view=Contact/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
