// ============================================================
// Exercise 59 - Answer
// Effect::watch 显式追踪依赖
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // 使用 Effect::watch 显式追踪 count
    Effect::watch(
        move || count.get(),
        move |count, prev_count, _| {
            println!("watch 触发: count={}, prev={:?}", count, prev_count);
        },
        false,
    );

    view! {
        <p>"count: " {count}</p>
        <button on:click=move |_| set_count.update(|n| *n += 1)>"+1"</button>
    }
}

fn main() {
    mount_to_body(Exercise);
}
