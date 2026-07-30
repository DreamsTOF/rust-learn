// ============================================================
// Exercise 211 - Answer: path_params — 路径参数 (:id)
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params_map;
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p><a href="/user/42">"查看用户 42"</a></p>
        <p><a href="/user/100">"查看用户 100"</a></p>
    }
}

#[component]
fn User() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id");

    view! {
        <h2>"用户信息"</h2>
        <p>
            {move || id().map(|s| format!("用户 ID: {}", s)).unwrap_or_else(|| "未指定用户 ID".to_string())}
        </p>
        <p><a href="/">"返回首页"</a></p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <a href="/">"首页"</a>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/user/:id") view=User/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
