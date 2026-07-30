// ============================================================
// Exercise 229 — Answer: route_state
// ============================================================

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, A};
use leptos_router::hooks::{use_navigate, use_location};
use leptos_router::location::State;
use leptos_router::path;
use leptos_router::NavigateOptions;
use wasm_bindgen::JsValue;

#[component]
fn Home() -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let navigate = use_navigate();

    view! {
        <h2>"首页"</h2>
        <p>"在下方输入内容，然后点击按钮传递到详情页"</p>
        <input
            type="text"
            placeholder="输入要传递的数据"
            on:input:target=move |ev| set_input.set(ev.target().value())
        />
        <button on:click=move |_| {
            navigate("/detail", NavigateOptions {
                state: State::new(Some(JsValue::from_str(&input.get()))),
                ..Default::default()
            });
        }>
            "传递数据到详情页"
        </button>
        <br/><br/>
        <A href="/detail">"直接跳转详情页（无数据）"</A>
    }
}

#[component]
fn Detail() -> impl IntoView {
    let location = use_location();

    view! {
        <h2>"详情页"</h2>
        <p>"从首页传入的数据:"</p>
        <p style="color:#666;">
            {move || location.state.get().to_js_value().as_string().unwrap_or_else(|| "(没有接收到数据)".to_string())}
        </p>
        <A href="/">"返回首页"</A>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"首页"</A>
                <A href="/detail">"详情页"</A>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/detail") view=Detail/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
