// ============================================================
// 练习 e232 — search_form — 参考答案
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::{use_navigate, use_query_map};
use leptos_router::{path, NavigateOptions};

#[component]
fn Exercise() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    let current_q = move || query().get("q").unwrap_or_default();

    let on_input = move |ev| {
        let value = event_target_value(&ev);
        let _ = navigate(&format!("/?q={value}"), NavigateOptions::default());
    };

    view! {
        <Router>
            <h1>"e232: 搜索表单"</h1>
            <div>
                <input
                    type="text"
                    placeholder="输入搜索词..."
                    prop:value=current_q
                    on:input=on_input
                />
            </div>
            <p>"当前搜索词: " {current_q}</p>
            <Routes fallback=|| "页面未找到">
                <Route path=path!("") view=move || {
                    let q = current_q();
                    if q.is_empty() {
                        view! { <p>"请输入关键词搜索"</p> }.into_any()
                    } else {
                        view! { <p>"正在搜索 \"" {q} "\" 的结果..."</p> }.into_any()
                    }
                }/>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
