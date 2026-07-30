// ============================================================
// Exercise 185 - Stream into Signal
// ============================================================

use futures::stream;
use leptos::prelude::*;
use leptos::reactive::traits::FromStream;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    let signal: ArcReadSignal<Option<i32>> =
        FromStream::from_stream(stream::iter(1..=5));

    spawn_local(async move {
        leptos::task::tick().await;
    });

    view! {
        <div>
            <p>"练习 185 — Stream → Signal (stream_into_signal)"</p>
            <p>"信号值: " {move || {
                match signal.get() {
                    None => "等待流数据...".to_string(),
                    Some(val) => format!("最新值: {}", val),
                }
            }}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
