// ============================================================
// Exercise 55 - Answer
// Effect 响应信号变化
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    Effect::new(move |_| {
        leptos::logging::log!("count 变化: {}", count());
    });

    view! {
        <div>
            <h2>"练习 55 (effect_reactive)"</h2>
            <p>"Effect 追踪 count 的变化，每次变化都会重新执行"</p>
            <p>"count: " {count}</p>
            <button on:click=move |_| set_count(count() + 1)>"增加"</button>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}
