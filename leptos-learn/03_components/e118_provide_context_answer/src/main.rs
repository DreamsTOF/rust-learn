// ============================================================
// Exercise 118 — Answer: provide_context
// ============================================================

use leptos::prelude::*;

#[component]
fn ThemedParagraph() -> impl IntoView {
    let theme = use_context::<&'static str>()
        .expect("theme should be provided by an ancestor");

    let style = match theme {
        "dark" => "background:#333; color:#fff; padding:8px; border-radius:4px;",
        _ => "background:#fff; color:#333; padding:8px; border-radius:4px;",
    };

    view! {
        <p style={style}>
            "当前主题: " {theme}
        </p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    provide_context("light");

    view! {
        <div style="padding:8px; border:1px solid #ccc; border-radius:4px;">
            <h3>"provide_context 示例"</h3>
            <p>"父组件通过 provide_context 注入了主题值"</p>
            <ThemedParagraph/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
