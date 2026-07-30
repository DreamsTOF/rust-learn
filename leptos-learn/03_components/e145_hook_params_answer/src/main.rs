// ============================================================
// 练习 e145: hook_params — 参数化 Hook
// ============================================================

use leptos::prelude::*;

fn use_counter_start(start: i32) -> (ReadSignal<i32>, WriteSignal<i32>, impl Fn()) {
    let (count, set_count) = signal(start);
    let increment = move || {
        set_count.set(count() + 1);
    };
    (count, set_count, increment)
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, _set_count, increment) = use_counter_start(10);

    view! {
        <div>
            <h3>"练习 e145: use_counter(10) 参数化 Hook"</h3>
            <p style="font-size: 24px; font-weight: bold;">"计数: " {count}</p>
            <button on:click=move |_| { increment(); }>"+1"</button>
            <p style="color: #888; font-size: 14px;">"初始值为 10，点击按钮递增"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
