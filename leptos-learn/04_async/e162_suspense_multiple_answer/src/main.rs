// ============================================================
// Exercise 162 - Suspense Multiple Resources
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

async fn delay(ms: u64) {
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(ms));
    rx.await.unwrap();
}

async fn load_user() -> String { delay(1500).await; "Alice".to_string() }
async fn load_score() -> u32 { delay(2500).await; 95 }
async fn load_level() -> String { delay(2000).await; "Gold".to_string() }

#[component]
fn Exercise() -> impl IntoView {
    let user = Resource::new(|| (), |_| async move { load_user().await });
    let score = Resource::new(|| (), |_| async move { load_score().await });
    let level = Resource::new(|| (), |_| async move { load_level().await });

    view! {
        <div>
            <h2>"Exercise 162: Suspense Multiple Resources"</h2>
            <Suspense fallback=|| view! { <p>"Loading user data..."</p> }>
                <ul>
                    <li>"User: " {move || user.get()}</li>
                    <li>"Score: " {move || score.get().map(|s| s.to_string())}</li>
                    <li>"Level: " {move || level.get()}</li>
                </ul>
            </Suspense>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
