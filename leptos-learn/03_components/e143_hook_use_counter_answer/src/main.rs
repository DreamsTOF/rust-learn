// ============================================================
// 练习 e143: use_counter — 基础 Hook 封装计数逻辑
// ============================================================

use leptos::prelude::*;

fn use_counter() -> (ReadSignal<i32>, WriteSignal<i32>) {
    signal(0)
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = use_counter();

    view! {
        <div>
            <h3>"练习 e143: use_counter 基础 Hook"</h3>
            <p style="font-size: 24px; font-weight: bold;">"计数: " {count}</p>
            <button on:click=move |_| { set_count.set(count() + 1); }>"+1"</button>
            <button on:click=move |_| { set_count.set(0); }>"重置"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
