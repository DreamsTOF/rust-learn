// ============================================================
// Exercise 57 - Answer
// Effect 不追踪外部变更
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // Effect 不读取任何信号，只执行一次
    Effect::new(move |_| {
        leptos::logging::log!("Effect 执行了（但不会再次执行）");
    });

    view! {
        <div>
            <h2>"练习 57 (effect_no_tracking)"</h2>
            <p>"Effect 没有追踪任何信号，所以只执行一次"</p>
            <p>"count: " {count}</p>
            <button on:click=move |_| set_count(count() + 1)>"增加 count"</button>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}
