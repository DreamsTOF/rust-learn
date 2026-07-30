// ============================================================
// Exercise 123 - Answer: Context Signal
// ============================================================

use leptos::prelude::*;

#[component]
fn CounterButton() -> impl IntoView {
    let count = use_context::<RwSignal<i32>>()
        .expect("RwSignal<i32> should be provided");

    view! {
        <div style="border: 1px solid blue; padding: 8px; margin: 8px 0;">
            <p>"Count: " {count}</p>
            <button on:click=move |_| count.update(|n| *n += 1)>"+1"</button>
            <button on:click=move |_| count.update(|n| *n -= 1)>"-1"</button>
        </div>
    }
}

#[component]
fn CounterDisplay() -> impl IntoView {
    let count = use_context::<RwSignal<i32>>()
        .expect("RwSignal<i32> should be provided");

    view! {
        <div style="border: 1px solid orange; padding: 8px; margin: 8px 0;">
            <p>"Current count (read-only view): " {count}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let count = RwSignal::new(0);
    provide_context(count);

    view! {
        <div style="border: 1px solid gray; padding: 8px;">
            <h2>"Context Signal Demo"</h2>
            <p>"通过 Context 传递 RwSignal，多个子组件共享同一状态"</p>
            <CounterButton/>
            <CounterDisplay/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
