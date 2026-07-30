// ============================================================
// Exercise 159 - Resource Error Handling
// ============================================================

use leptos::prelude::*;

async fn fetch_toggle(should_error: bool) -> Result<String, String> {
    if should_error {
        Err("数据加载失败！".to_string())
    } else {
        Ok("数据加载成功 ✅".to_string())
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (trigger, set_trigger) = signal(false);
    let data = Resource::new(
        move || trigger(),
        move |val| async move { fetch_toggle(val).await },
    );

    view! {
        <div>
            <p>"练习 159: resource_error — 错误处理"</p>
            <p>"当前模式: " {move || if trigger() { "错误模式" } else { "正常模式" }}</p>
            <p>
                {move || match data.map(|r| r.clone()) {
                    Some(Ok(value)) => value.into_any(),
                    Some(Err(e)) => format!("❌ {}", e).into_any(),
                    None => "加载中...".into_any(),
                }}
            </p>
            <button on:click=move |_| set_trigger.update(|v| *v = !*v)>
                "切换模式"
            </button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
