// ============================================================
// Exercise 56 - Answer
// Effect 只追踪内部读取的信号
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (a, set_a) = signal(0);
    let (b, set_b) = signal(0);

    // Effect 只读取 a，因此只追踪 a
    Effect::new(move |_| {
        leptos::logging::log!("Effect 执行: a = {}", a());
    });

    view! {
        <div>
            <h2>"练习 56 (effect_tracking)"</h2>
            <p>"Effect 只读取 a，不读取 b"</p>
            <p>"a: " {a}</p>
            <p>"b: " {b}</p>
            <button on:click=move |_| set_a(a() + 1)>"增加 a"</button>
            <button on:click=move |_| set_b(b() + 1)>"增加 b"</button>
            <p>"点击「增加 b」不会触发 Effect 重新执行"</p>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}
