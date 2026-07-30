// ============================================================
// Exercise 175 - select_futures
// ============================================================

use futures::future::FutureExt;
use leptos::prelude::*;
use leptos::task::spawn_local;

async fn fast_task() -> String {
    "快速任务完成".to_string()
}

async fn slow_task() -> String {
    "慢速任务完成".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    let (winner, set_winner) = signal("等待竞速结果...".to_string());

    spawn_local(async move {
        let fut1 = fast_task().fuse();
        let fut2 = slow_task().fuse();
        futures::pin_mut!(fut1, fut2);

        futures::select! {
            result = fut1 => { set_winner.set(result); },
            result = fut2 => { set_winner.set(result); },
        }
    });

    view! {
        <div>
            <h2>"e175: select_futures"</h2>
            <p>{winner}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
