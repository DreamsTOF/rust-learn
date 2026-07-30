// ============================================================
// Exercise 226 — Answer: route_animation
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p>"欢迎来到首页"</p>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h2>"关于"</h2>
        <p>"这是关于页面"</p>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <h2>"联系"</h2>
        <p>"请联系我们"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
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
