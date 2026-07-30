// ============================================================
// Exercise 157 - Resource Dependencies
// ============================================================

use leptos::prelude::*;

async fn fetch_item(id: i32) -> String {
    format!("Item #{} — 加载时间: {:?}", id, std::time::SystemTime::now())
}

#[component]
fn Exercise() -> impl IntoView {
    let (id, set_id) = signal(1);
    let data = Resource::new(
        move || id(),
        move |id| async move { fetch_item(id).await },
    );

    view! {
        <div>
            <p>"练习 157: resource_deps — 响应式依赖"</p>
            <p>"当前 ID: " {id}</p>
            <p>"Resource 数据: " {move || data.map(|v| v.clone())}</p>
            <button on:click=move |_| set_id.update(|v| *v += 1)>
                "ID += 1"
            </button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
