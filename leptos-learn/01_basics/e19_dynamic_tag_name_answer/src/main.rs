// ============================================================
// 练习 e19: 动态标签名 — 参考答案
//
// 核心知识点:
//   - leptos::html::h1、leptos::html::h2 等动态标签函数
//   - 根据信号值动态选择 HTML 标签
//   - into_any() 统一不同标签类型
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (level, set_level) = signal(1);

    view! {
        {match level.get() {
            1 => leptos::html::h1().child(format!("标题级别 {}", level.get())).into_any(),
            2 => leptos::html::h2().child(format!("标题级别 {}", level.get())).into_any(),
            3 => leptos::html::h3().child(format!("标题级别 {}", level.get())).into_any(),
            _ => leptos::html::h1().child(format!("标题级别 {}", level.get())).into_any(),
        }}

        <p>"当前标题级别: " {level.get()}</p>

        <button on:click=move |_| set_level.set(1)>"h1"</button>
        <button on:click=move |_| set_level.set(2)>"h2"</button>
        <button on:click=move |_| set_level.set(3)>"h3"</button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
