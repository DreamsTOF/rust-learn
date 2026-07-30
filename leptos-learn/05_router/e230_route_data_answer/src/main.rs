// ============================================================
// Exercise 230 — Answer: route_data
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::path;

#[derive(Clone, Copy)]
struct SharedCount(RwSignal<i32>);

#[component]
fn Counter() -> impl IntoView {
    let count = use_context::<SharedCount>().expect("SharedCount 未提供");

    view! {
        <h2>"计数器"</h2>
        <p>"当前计数: "</p>
        <p style="font-size:2rem;font-weight:bold;margin:0.5rem 0;">
            {move || count.0.get()}
        </p>
        <button on:click=move |_| { count.0.update(|n| *n += 1); }>
            "增加"
        </button>
        <button on:click=move |_| { count.0.update(|n| *n -= 1); }>
            "减少"
        </button>
        <br/><br/>
        <A href="/display">"查看显示页"</A>
    }
}

#[component]
fn Display() -> impl IntoView {
    let count = use_context::<SharedCount>().expect("SharedCount 未提供");

    view! {
        <h2>"数据显示页"</h2>
        <p>"共享计数: "</p>
        <p style="font-size:2rem;font-weight:bold;margin:0.5rem 0;">
            {move || count.0.get()}
        </p>
        <p>"提示: 在计数器页面修改后，导航到此页面查看变化"</p>
        <A href="/counter">"回到计数器"</A>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let shared = SharedCount(RwSignal::new(0));
    provide_context(shared);

    view! {
        <Router>
            <nav>
                <A href="/counter">"计数器"</A>
                <A href="/display">"显示页"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/counter") view=Counter/>
                    <Route path=path!("/display") view=Display/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
