// ============================================================
// 练习 252 — 参考答案
// ============================================================

use leptos::prelude::*;

fn main() {
    mount_to_body(Exercise);
}

#[component]
fn Exercise() -> impl IntoView {
    let value = RwSignal::new(String::new());

    view! {
        <div>
            <h2>"练习 252: on:change 事件"</h2>
            <input type="text" on:change=move |ev| value.set(event_target_value(&ev)) />
            <p>"你提交的是: " {value}</p>
        </div>
    }
}
