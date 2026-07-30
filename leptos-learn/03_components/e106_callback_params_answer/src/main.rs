// ============================================================
// Exercise 106 - Answer: callback_params
// ============================================================

use leptos::prelude::*;

#[component]
fn SearchInput<F>(on_input: F) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    view! {
        <input
            type="text"
            placeholder="在此输入..."
            on:input=move |ev| {
                on_input(event_target_value(&ev));
            }
        />
    }
}

#[component]
fn App() -> impl IntoView {
    let (text, set_text) = signal(String::new());

    view! {
        <h3>"练习 106: callback_params"</h3>
        <SearchInput on_input=move |val: String| {
            set_text(val);
        }/>
        <p>"你输入了: " {text}</p>
    }
}

fn main() {
    mount_to_body(App);
}
