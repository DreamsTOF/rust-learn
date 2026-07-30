use leptos::prelude::*;
use leptos::html;

#[component]
fn Exercise() -> impl IntoView {
    let (level, set_level) = signal(1u8);

    let heading = move || {
        let tag = match level.get() {
            1 => "h1",
            2 => "h2",
            3 => "h3",
            _ => "h1",
        };
        html::custom(tag).child("动态标签标题")
    };

    view! {
        <h2>"动态标签名"</h2>
        <p>"当前标签: " {move || format!("h{}", level.get())}</p>

        <div style="border: 1px solid #ccc; padding: 1rem; margin: 0.5rem 0;">
            {heading}
        </div>

        <button on:click=move |_| set_level.update(|v| *v = 1)>
            "h1"
        </button>
        <button on:click=move |_| set_level.update(|v| *v = 2)>
            "h2"
        </button>
        <button on:click=move |_| set_level.update(|v| *v = 3)>
            "h3"
        </button>
    }
}

fn main() {
    mount_to_body(Exercise);
}
