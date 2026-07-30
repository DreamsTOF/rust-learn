// ============================================================
// Exercise 158 - Resource Get
// ============================================================

use leptos::prelude::*;

async fn fetch_score() -> String {
    "你的得分: 42".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    let data = Resource::new(
        move || (),
        move |_| async move { fetch_score().await },
    );

    view! {
        <div>
            <p>"练习 158: resource_get — 获取数据"</p>
            <p>
                {move || match data.map(|v| v.clone()) {
                    Some(score) => score.into_any(),
                    None => "正在加载...".into_any(),
                }}
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
