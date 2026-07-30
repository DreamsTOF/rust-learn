// ============================================================
// Exercise 172 - spawn_local
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;

async fn do_work() -> String {
    "任务完成！".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    let (status, set_status) = signal("点击按钮启动任务".to_string());

    view! {
        <div>
            <h2>"e172: spawn_local"</h2>
            <p>{status}</p>
            <button on:click=move |_| {
                set_status.set("执行中...".to_string());
                spawn_local(async move {
                    let result = do_work().await;
                    set_status.set(result);
                });
            }>"启动任务"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
