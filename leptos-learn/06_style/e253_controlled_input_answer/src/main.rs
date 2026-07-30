// ============================================================
// 练习 253 — 参考答案
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
            <h2>"练习 253: 受控输入框"</h2>
            <input type="text" prop:value={value} on:input=move |ev| value.set(event_target_value(&ev)) />
            <button on:click=move |_| value.set(String::new())>"重置"</button>
            <p>"值: " {value} "，长度: " {move || value.read().len()}</p>
        </div>
    }
}
