// ============================================================
// Exercise 213 - Answer: wildcard — 通配符匹配
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params_map;
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p><a href="/files">"文件列表"</a></p>
        <p><a href="/files/docs/readme.txt">"docs/readme.txt"</a></p>
        <p><a href="/files/images/photo.jpg">"images/photo.jpg"</a></p>
        <p><a href="/files/a/b/c/deep.txt">"a/b/c/deep.txt"</a></p>
    }
}

#[component]
fn Files() -> impl IntoView {
    let params = use_params_map();
    let tail = move || params.get().get("tail");

    view! {
        <h2>"文件浏览"</h2>
        <p>
            {move || match tail() {
                Some(p) => format!("当前路径: {}", p),
                None => "文件根目录".to_string(),
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
                <a href="/files">"文件"</a>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/files/*tail") view=Files/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
