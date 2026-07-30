// ============================================================
// Exercise 212 - Answer: optional_params — 可选路径参数
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params_map;
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p><a href="/greet">"匿名问候"</a></p>
        <p><a href="/greet/小明">"问候小明"</a></p>
        <p><a href="/greet/小红">"问候小红"</a></p>
    }
}

#[component]
fn Greet() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.get().get("name");

    view! {
        <h2>"问候"</h2>
        <p>
            {move || match name() {
                Some(n) => format!("你好，{}！", n),
                None => "你好，访客！".to_string(),
            }}
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
                    <Route path=path!("/greet/:name?") view=Greet/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
