// ============================================================
// Exercise 19 - Answer: Dynamic Tag Name
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (level, set_level) = signal(1);

    view! {
        {match level() {
            1 => leptos::html::h1()
                .child(format!("标题级别 {}", level()))
                .into_any(),
            2 => leptos::html::h2()
                .child(format!("标题级别 {}", level()))
                .into_any(),
            3 => leptos::html::h3()
                .child(format!("标题级别 {}", level()))
                .into_any(),
            _ => leptos::html::h1()
                .child(format!("标题级别 {}", level()))
                .into_any(),
        }}

        <p>"当前标题级别: " {level()}</p>

        <button on:click=move |_| set_level(1)>"h1"</button>
        <button on:click=move |_| set_level(2)>"h2"</button>
        <button on:click=move |_| set_level(3)>"h3"</button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
