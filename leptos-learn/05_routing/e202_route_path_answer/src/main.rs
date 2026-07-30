// ============================================================
// Exercise 202 - Answer: Route Path Matching
// ============================================================

use leptos::prelude::*;
use leptos_router::path;
use leptos_router::components::{Router, Routes, Route, A};

#[component]
fn PageA() -> impl IntoView {
    view! { <h3>"你正在访问页面 A"</h3> }
}

#[component]
fn PageB() -> impl IntoView {
    view! { <h3>"你正在访问页面 B"</h3> }
}

#[component]
fn PageC() -> impl IntoView {
    view! { <h3>"你正在访问页面 C"</h3> }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/a">"Page A"</A>
                " | "
                <A href="/b">"Page B"</A>
                " | "
                <A href="/c">"Page C"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/a") view=PageA/>
                    <Route path=path!("/b") view=PageB/>
                    <Route path=path!("/c") view=PageC/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
