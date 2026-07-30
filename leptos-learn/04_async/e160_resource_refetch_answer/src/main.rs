// ============================================================
// Exercise 160 - Resource Refetch
// ============================================================

use leptos::prelude::*;

async fn fetch_timestamp() -> String {
    format!("当前时间戳: {:?}", std::time::SystemTime::now())
}

#[component]
fn Exercise() -> impl IntoView {
    let data = Resource::new(
        move || (),
        move |_| async move { fetch_timestamp().await },
    );

    view! {
        <div>
            <p>"练习 160: resource_refetch — 手动刷新"</p>
            <p>"数据: " {move || data.map(|v| v.clone())}</p>
            <button on:click=move |_| data.refetch()>
                "刷新数据"
            </button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
