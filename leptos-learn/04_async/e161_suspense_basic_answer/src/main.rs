// ============================================================
// Exercise 161 - Suspense Basic
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

async fn load_data() -> String {
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_secs(2));
    rx.await.unwrap();
    "Hello from Suspense!".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    let data = Resource::new(|| (), |_| async move { load_data().await });

    view! {
        <div>
            <h2>"Exercise 161: Suspense Basic"</h2>
            <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                <p>{move || data.get()}</p>
            </Suspense>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
