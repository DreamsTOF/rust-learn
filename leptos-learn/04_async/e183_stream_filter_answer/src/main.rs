// ============================================================
// Exercise 183 - Stream Filter
// ============================================================

use futures::future;
use futures::stream::{self, StreamExt};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    let items: RwSignal<Vec<i32>> = RwSignal::new(Vec::new());

    spawn_local(async move {
        let mut stream = stream::iter(1..=10).filter(|x| future::ready(*x % 2 == 0));
        while let Some(value) = stream.next().await {
            items.update(|v| v.push(value));
            leptos::task::tick().await;
        }
    });

    view! {
        <div>
            <p>"练习 183 — Stream Filter (stream_filter)"</p>
            <p>"1..=10 中的偶数: " {move || {
                let v = items.get();
                if v.is_empty() {
                    "过滤中...".to_string()
                } else {
                    v.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ")
                }
            }}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
