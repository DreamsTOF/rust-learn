// ============================================================
// 练习 251 — 参考答案
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
            <h2>"练习 251: on:input 事件"</h2>
            <input type="text" on:input=move |ev| value.set(event_target_value(&ev)) />
            <p>"你输入的是: " {value}</p>
        </div>
    }
}
