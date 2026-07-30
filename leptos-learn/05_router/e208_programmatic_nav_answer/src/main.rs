// ============================================================
// Exercise 208 - Answer
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p>"欢迎来到首页！"</p>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h2>"关于"</h2>
        <p>"这是关于页面。"</p>
    }
}

#[component]
fn NavButtons() -> impl IntoView {
    let navigate = use_navigate();
    let nav2 = navigate.clone();
    view! {
        <nav>
            <button on:click=move |_| navigate("/home", Default::default())>
                "Go Home"
            </button>
            " "
            <button on:click=move |_| nav2("/about", Default::default())>
                "Go About"
            </button>
        </nav>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <NavButtons/>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/home") view=Home/>
                    <Route path=path!("/about") view=About/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
