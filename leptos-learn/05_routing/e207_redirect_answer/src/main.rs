// ============================================================
// Exercise 207 - Answer: Redirect with <Redirect/> component
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::path;

#[component]
fn Login() -> impl IntoView {
    let logged_in = use_context::<RwSignal<bool>>().expect("logged_in not found");
    let navigate = use_navigate();

    view! {
        <h2>"登录"</h2>
        <p>"这是登录页面。请先登录。"</p>
        <button on:click=move |_| {
            logged_in.set(true);
            navigate("/dashboard", Default::default());
        }>"点击登录"</button>
    }
}

#[component]
fn Dashboard() -> impl IntoView {
    let logged_in = use_context::<RwSignal<bool>>().expect("logged_in not found");

    view! {
        {move || if !logged_in() {
            view! { <Redirect path="/login"/> }.into_any()
        } else {
            view! {
                <h2>"仪表盘"</h2>
                <p>"欢迎回来！"</p>
            }.into_any()
        }}
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let logged_in = RwSignal::new(false);
    provide_context(logged_in);

    view! {
        <Router>
            <nav>
                <A href="/dashboard">"仪表盘"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/login") view=Login/>
                    <Route path=path!("/dashboard") view=Dashboard/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
