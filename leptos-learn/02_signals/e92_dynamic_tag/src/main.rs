// ============================================================
// 练习 e92: dynamic_tag — 动态标签名
//
// 核心知识点:
//   - html::custom(tag) 根据字符串动态创建 HTML 元素
//   - 信号驱动标签名变化（h1/h2/h3 切换）
//
// 难度: ⭐⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;
use leptos::html;

#[component]
fn Exercise() -> impl IntoView {
    let (level, set_level) = signal(1u8);

    let heading = move || {
        let tag = match level.get() {
            1 => "h1",
            2 => "h2",
            3 => "h3",
            _ => "h1",
        };
        html::custom(tag).child("动态标签标题")
    };

    view! {
        <h2>"动态标签名"</h2>
        <p>"当前标签: " {move || format!("h{}", level.get())}</p>

        <div style="border: 1px solid #ccc; padding: 1rem; margin: 0.5rem 0;">
            {heading}
        </div>

        <button on:click=move |_| set_level.update(|v| *v = 1)>
            "h1"
        </button>
        <button on:click=move |_| set_level.update(|v| *v = 2)>
            "h2"
        </button>
        <button on:click=move |_| set_level.update(|v| *v = 3)>
            "h3"
        </button>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
// use leptos::html;
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (level, set_level) = signal(1u8);
//     let heading = move || {
//         let tag = match level.get() {
//             1 => "h1", 2 => "h2", 3 => "h3",
//             _ => "h1",
//         };
//         html::custom(tag).child("动态标签标题")
//     };
//     view! {
//         <h2>"动态标签名"</h2>
//         <p>"当前标签: " {move || format!("h{}", level.get())}</p>
//         <div style="border: 1px solid #ccc; padding: 1rem; margin: 0.5rem 0;">{heading}</div>
//         <button on:click=move |_| set_level.update(|v| *v = 1)>"h1"</button>
//         <button on:click=move |_| set_level.update(|v| *v = 2)>"h2"</button>
//         <button on:click=move |_| set_level.update(|v| *v = 3)>"h3"</button>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
//
// ### 知识点
// - `html::custom(tag_name)` 可以创建任意 HTML 标签名
// - 返回的 HtmlElement 实现了 IntoView，可直接在视图中使用
// - 结合信号，可以实现动态标签切换
// - 注意：动态标签不会应用 view! 宏的编译时标签检查
// </details>
