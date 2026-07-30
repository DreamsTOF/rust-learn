// ============================================================
// Exercise 171 - async fn
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;

async fn fetch_message() -> String {
    "你好，async！".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    let (message, set_message) = signal(String::new());

    spawn_local(async move {
        let msg = fetch_message().await;
        set_message.set(msg);
    });

    view! {
        <div>
            <h2>"e171: async fn"</h2>
            <p>{message}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
