// ============================================================
// Exercise 173 - await_signal
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0i32);
    let (status, set_status) = signal("准备就绪".to_string());

    spawn_local(async move {
        for i in 1..=5 {
            set_count.set(i);
            set_status.set(format!("第 {i} 次更新"));
        }
    });

    view! {
        <div>
            <h2>"e173: await_signal"</h2>
            <p>"计数: " {count}</p>
            <p>{status}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
