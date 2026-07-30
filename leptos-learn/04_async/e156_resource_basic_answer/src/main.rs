// ============================================================
// Exercise 156 - Resource::new() Basic
// ============================================================

use leptos::prelude::*;

async fn fetch_greeting() -> String {
    "你好，Resource！".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    let data = Resource::new(
        move || (),
        move |_| async move { fetch_greeting().await },
    );

    view! {
        <div>
            <p>"练习 156: resource_basic — Resource::new() 基础"</p>
            <p>"来自 Resource 的数据: " {move || data.map(|v| v.clone())}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
