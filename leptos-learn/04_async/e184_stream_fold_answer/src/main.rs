// ============================================================
// Exercise 184 - Stream Fold
// ============================================================

use futures::stream::{self, StreamExt};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    let result: RwSignal<Option<i32>> = RwSignal::new(None);

    spawn_local(async move {
        let sum = stream::iter(1..=5)
            .fold(0, |acc, x| async move { acc + x })
            .await;
        result.set(Some(sum));
    });

    view! {
        <div>
            <p>"练习 184 — Stream Fold (stream_fold)"</p>
            <p>"1 + 2 + 3 + 4 + 5 = " {move || {
                match result.get() {
                    Some(sum) => sum.to_string(),
                    None => "计算中...".to_string(),
                }
            }}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
