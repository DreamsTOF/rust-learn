// ============================================================
// 练习 255 — 参考答案
// ============================================================

use leptos::prelude::*;

fn main() {
    mount_to_body(Exercise);
}

#[component]
fn Exercise() -> impl IntoView {
    let content = RwSignal::new(String::new());

    view! {
        <div>
            <h2>"练习 255: 受控文本域"</h2>
            <textarea prop:value={content} on:input=move |ev| content.set(event_target_value(&ev)) />
            <p>"字符数: " {move || content.read().len()}</p>
        </div>
    }
}
