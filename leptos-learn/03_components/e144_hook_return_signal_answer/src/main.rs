// ============================================================
// 练习 e144: hook_return_signal — Hook 返回信号和便利闭包
// ============================================================

use leptos::prelude::*;

fn use_counter_with_increment() -> (ReadSignal<i32>, WriteSignal<i32>, impl Fn()) {
    let (count, set_count) = signal(0);
    let increment = move || {
        set_count.set(count() + 1);
    };
    (count, set_count, increment)
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, _set_count, increment) = use_counter_with_increment();

    view! {
        <div>
            <h3>"练习 e144: use_counter + increment 闭包"</h3>
            <p style="font-size: 24px; font-weight: bold;">"计数: " {count}</p>
            <button on:click=move |_| { increment(); }>"+1 (使用 increment)"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
