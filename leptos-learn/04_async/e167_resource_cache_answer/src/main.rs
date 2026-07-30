// ============================================================
// Exercise 167 - Resource Cache Strategies
// ============================================================

use leptos::prelude::*;
use std::time::Duration;
use std::cell::Cell;

thread_local! {
    static API_CALL_COUNT: Cell<u32> = Cell::new(0);
}

async fn fetch_data() -> String {
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(500));
    rx.await.unwrap();
    API_CALL_COUNT.with(|c| c.set(c.get() + 1));
    format!("数据 (#{})", API_CALL_COUNT.with(|c| c.get()))
}

#[component]
fn Exercise() -> impl IntoView {
    let data = Resource::new(move || (), |_| async move { fetch_data().await });
    let (should_cache, set_should_cache) = signal(true);
    let (count, set_count) = signal(0u32);
    let (cached_value, set_cached_value) = signal(String::new());

    view! {
        <div>
            <h2>"Exercise 167: Resource Cache"</h2>
            <p>"API calls: " {move || count.get()}</p>
            <Suspense fallback=|| view! { <p>"加载中..."</p> }>
                <p>{move || data.map(|v| format!("数据: {}", v))}</p>
            </Suspense>
            <button on:click=move |_| data.refetch()>"刷新"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
