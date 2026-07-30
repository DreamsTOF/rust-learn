// ============================================================
// Exercise 53 - Answer
// 派生信号的条件传播
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let x = RwSignal::new(10);
    let threshold = RwSignal::new(15);
    let _label = RwSignal::new("A".to_string());

    let is_above = move || x() > threshold();
    let status = move || if is_above() { "above" } else { "below" };

    view! {
        <div>
            <p>"练习 53: conditional_propagation"</p>
            <p>"x = " {x}", threshold = " {threshold}</p>
            <p>"is_above = " {is_above}</p>
            <p>"status = " {status}</p>
            <button on:click=move |_| x.set(x.get() + 1)>"x += 1"</button>
            <button on:click=move |_| threshold.set(threshold.get() + 1)>"threshold += 1"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
