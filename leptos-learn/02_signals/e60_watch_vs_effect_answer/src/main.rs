// ============================================================
// Exercise 60 - Answer
// Effect::new 自动追踪 vs Effect::watch 显式依赖
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (a, set_a) = signal(0);
    let (b, set_b) = signal(0);

    // Effect::new — 自动追踪内部所有信号
    Effect::new(move || {
        println!("Effect::new 触发: a={}, b={}", a.read(), b.read());
    });

    // Effect::watch — 只追踪 dependency_fn 中的信号
    Effect::watch(
        move || a.get(),
        move |val, _prev, _| {
            println!("Effect::watch 触发: a={}", val);
        },
        false,
    );

    view! {
        <p>"a: " {a} " | b: " {b}</p>
        <button on:click=move |_| set_a.update(|n| *n += 1)>"a +1"</button>
        <button on:click=move |_| set_b.update(|n| *n += 1)>"b +1"</button>
    }
}

fn main() {
    mount_to_body(Exercise);
}
