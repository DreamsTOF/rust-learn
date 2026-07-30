// ============================================================
// 练习 e346: Signal vs Global — 比较 RwSignal 本地状态 vs create_context 全局状态
//
// 核心知识点:
//   - RwSignal::new() 创建本地状态，每个组件实例独立
//   - provide_context / expect_context 实现全局状态共享
//   - 对比两种状态管理方式的适用场景
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[component]
fn GlobalCounter() -> impl IntoView {
    let count = expect_context::<RwSignal<i32>>();

    view! {
        <div>
            <p>"全局计数: " {move || count.get()}</p>
            <button on:click=move |_| count.update(|n| *n += 1)>"+1"</button>
        </div>
    }
}

#[component]
fn LocalCounter() -> impl IntoView {
    let count = RwSignal::new(0);

    view! {
        <div>
            <p>"本地计数: " {move || count.get()}</p>
            <button on:click=move |_| count.update(|n| *n += 1)>"+1"</button>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    provide_context(RwSignal::new(0));

    view! {
        <div>
            <h2>"全局状态（共享）"</h2>
            <GlobalCounter/>
            <GlobalCounter/>

            <h2>"本地状态（独立）"</h2>
            <LocalCounter/>
            <LocalCounter/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
