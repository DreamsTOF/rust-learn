// ============================================================
// Exercise 227 — Answer: lazy_route
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::path;

async fn fetch_lazy_data() -> String {
    "这是通过懒加载获取的数据".to_string()
}

#[component]
fn LazyHome() -> impl IntoView {
    let data = LocalResource::new(|| async { fetch_lazy_data().await });

    view! {
        <Suspense fallback=|| view! { <p>"正在加载路由..."</p> }>
            <h2>"懒加载页面"</h2>
            <p>{data.map(|d| d.clone())}</p>
        </Suspense>
    }
}

#[component]
fn FastAbout() -> impl IntoView {
    view! {
        <h2>"关于"</h2>
        <p>"这个页面是立即加载的"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"首页（懒加载）"</A>
                <A href="/about">"关于"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=LazyHome/>
                    <Route path=path!("/about") view=FastAbout/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
