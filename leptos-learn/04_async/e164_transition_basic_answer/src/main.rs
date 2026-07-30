// ============================================================
// Exercise 164 - Transition Basic
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

async fn delay(ms: u64) {
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(ms));
    rx.await.unwrap();
}

async fn fetch_item(id: u32) -> String {
    delay(2000).await;
    format!("Item #{}: some data loaded at {:?}", id, std::time::SystemTime::now())
}

#[component]
fn Exercise() -> impl IntoView {
    let (id, set_id) = signal(1u32);
    let data = Resource::new(
        move || id.get(),
        |id| async move { fetch_item(id).await },
    );

    view! {
        <div>
            <h2>"Exercise 164: Transition Basic"</h2>
            <p>"Current ID: " {move || id.get()}</p>
            <button on:click=move |_| set_id.update(|n| *n += 1)>
                "Load Item " {move || id.get() + 1}
            </button>
            <hr/>
            <Transition fallback=|| view! { <p>"Loading..."</p> }>
                <div style="padding: 8px; border: 1px solid #ccc;">
                    {move || data.get()}
                </div>
            </Transition>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
