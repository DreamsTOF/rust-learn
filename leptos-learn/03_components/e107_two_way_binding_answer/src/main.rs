// ============================================================
// Exercise 107 - Answer: two_way_binding
// ============================================================

use leptos::prelude::*;

#[component]
fn BindedInput<F>(value: String, on_change: F) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    view! {
        <input
            type="text"
            prop:value={value}
            on:input=move |ev| {
                on_change(event_target_value(&ev));
            }
        />
    }
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal(String::from("Leptos"));

    view! {
        <h3>"练习 107: two_way_binding"</h3>
        <p>"父组件值：" {name}</p>
        <BindedInput value={name()} on_change=move |val: String| {
            set_name(val);
        }/>
        <p>"再次展示：" {name}</p>
    }
}

fn main() {
    mount_to_body(App);
}
