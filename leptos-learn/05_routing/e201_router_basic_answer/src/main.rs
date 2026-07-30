// ============================================================
// Exercise 201 - Answer: Basic Router Setup
// ============================================================

use leptos::prelude::*;
use leptos_router::path;
use leptos_router::components::{Router, Routes, Route, A};

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
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"首页"</A>
                " | "
                <A href="/about">"关于"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/about") view=About/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
