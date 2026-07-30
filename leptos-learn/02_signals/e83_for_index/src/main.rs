// ============================================================
// 练习 e83: For Index — .enumerate() 获取索引
//
// 核心知识点:
//   - each 闭包中使用 .enumerate() 获取元素的索引
//   - 同时渲染索引和元素内容
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号 items 存储 Vec<&str>（编程语言列表）
    let (items, set_items) = signal(vec!["HTML", "CSS", "JavaScript", "Rust"]);

    view! {
        <h3>"编程语言"</h3>
        // TODO: 用 enumerate 获取索引，渲染 "序号. 名称"
        <For each=move || items.get().into_iter().enumerate()
            key=|(i, _)| *i
            let:entry
        >
            <p style="margin: 4px 0;">
                {entry.0 + 1}". " {entry.1}
            </p>
        </For>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (items, set_items) = signal(vec!["HTML", "CSS", "JavaScript", "Rust"]);
//
//     view! {
//         <h3>"编程语言"</h3>
//         <For each=move || items.get().into_iter().enumerate()
//             key=|(i, _)| *i
//             let:(i, item)
//         >
//             <p style="margin: 4px 0;">{i + 1}". " {item}</p>
//         </For>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - each 闭包中可以使用所有标准迭代器方法（如 .enumerate()、.filter()、.map()）
// - 索引可以用于显示序号；`let:entry` 绑定整个元组，通过 `entry.0`/`entry.1` 访问
// - 注意当列表增删时，用索引作 key 可能导致错误的 DOM 复用
// </details>
