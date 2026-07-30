// ============================================================
// Exercise 54 - Answer
// 创建第一个 Effect
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    Effect::new(move |_| {
        leptos::logging::log!("Effect 运行: count = {}", count());
    });

    view! {
        <div>
            <h2>"练习 54 (effect_new)"</h2>
            <p>"打开控制台 (F12) 查看 Effect 的输出"</p>
            <p>"count: " {count}</p>
            <button on:click=move |_| set_count(count() + 1)>"增加"</button>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}
