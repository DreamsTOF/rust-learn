// ============================================================
// Exercise 163 - Suspense Nested
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

async fn delay(ms: u64) {
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(ms));
    rx.await.unwrap();
}

async fn load_title() -> String { delay(1000).await; "Leptos 入门指南".to_string() }
async fn load_detail() -> String { delay(2000).await; "Suspense 是 Leptos 提供的强大工具...（此处省略 1000 字）".to_string() }

#[component]
fn Exercise() -> impl IntoView {
    let title = Resource::new(|| (), |_| async move { load_title().await });
    let detail = Resource::new(|| (), |_| async move { load_detail().await });

    view! {
        <div>
            <h2>"Exercise 163: Nested Suspense"</h2>
            <Suspense fallback=|| view! { <p>"Loading article..."</p> }>
                <article>
                    <h3>{move || title.get()}</h3>
                    <Suspense fallback=|| view! { <p>"Loading detail..."</p> }>
                        <p>{move || detail.get()}</p>
                    </Suspense>
                </article>
            </Suspense>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
