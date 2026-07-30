// ============================================================
// Exercise 58 - Answer
// 多个 Effect 互不影响
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (x, set_x) = signal(0);
    let (y, set_y) = signal(0);

    // Effect 1: 追踪 x
    Effect::new(move |_| {
        leptos::logging::log!("Effect 1 (追踪 x): x = {}", x());
    });

    // Effect 2: 追踪 y
    Effect::new(move |_| {
        leptos::logging::log!("Effect 2 (追踪 y): y = {}", y());
    });

    view! {
        <div>
            <h2>"练习 58 (multi_effect)"</h2>
            <p>"两个 Effect 分别追踪 x 和 y，互不影响"</p>
            <p>"x: " {x}</p>
            <p>"y: " {y}</p>
            <button on:click=move |_| set_x(x() + 1)>"增加 x"</button>
            <button on:click=move |_| set_y(y() + 1)>"增加 y"</button>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}
