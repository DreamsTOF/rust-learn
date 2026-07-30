// ============================================================
// Exercise 182 - Stream Map
// ============================================================

use futures::stream::{self, StreamExt};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    let items: RwSignal<Vec<i32>> = RwSignal::new(Vec::new());

    spawn_local(async move {
        let mut stream = stream::iter(1..=5).map(|x| x * 2);
        while let Some(value) = stream.next().await {
            items.update(|v| v.push(value));
            leptos::task::tick().await;
        }
    });

    view! {
        <div>
            <p>"练习 182 — Stream Map (stream_map)"</p>
            <p>"原始值 1..=5，map(x2) 后: " {move || {
                let v = items.get();
                if v.is_empty() {
                    "处理中...".to_string()
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
