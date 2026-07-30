// ============================================================
// Exercise 166 - LocalResource
// ============================================================

use leptos::prelude::*;

async fn fetch_greeting() -> String {
    "你好，世界！这是 LocalResource 加载的数据。".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    let greeting = LocalResource::new(|| async move {
        fetch_greeting().await
    });

    view! {
        <div>
            <h2>"LocalResource 示例"</h2>
            {move || match greeting.get() {
                None => view! { <p>"加载中..."</p> }.into_any(),
                Some(data) => view! { <p>{data}</p> }.into_any(),
            }}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
