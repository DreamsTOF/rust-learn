// ============================================================
// Exercise 51 - Answer
// 派生信号作为组件属性 (prop)
// ============================================================

use leptos::prelude::*;

#[component]
fn DisplayValue(value: Signal<i32>) -> impl IntoView {
    view! { <p>"value = " {value}</p> }
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <p>"练习 51: derived_as_prop"</p>
            <p>"count = " {count}</p>
            <DisplayValue value=Signal::derive(move || count() * 2) />
            <button on:click=move |_| set_count(count() + 1)>"count += 1"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
