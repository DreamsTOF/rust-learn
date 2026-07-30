// ============================================================
// Exercise 52 - Answer
// 闭包派生 vs create_memo
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let a = RwSignal::new(3);
    let b = RwSignal::new(5);

    // 方式 A: 闭包派生 —— 每次读取时重新计算
    let sum_closure = move || a() + b();

    // 方式 B: create_memo —— 缓存结果，仅依赖变化时重算
    let sum_memo = Memo::new(move |_| a() + b());

    view! {
        <div>
            <p>"练习 52: signal_vs_create_memo"</p>
            <p>"闭包派生 sum = " {sum_closure}</p>
            <p>"create_memo sum = " {sum_memo}</p>
            <button on:click=move |_| a.set(a.get() + 1)>"a += 1"</button>
            <button on:click=move |_| b.set(b.get() + 1)>"b += 1"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
