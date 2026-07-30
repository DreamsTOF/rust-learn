// ============================================================
// 练习 e79: Ternary Expr — Rust 三元表达式条件渲染
//
// 核心知识点:
//   - 在 view! 中直接写 `{if cond { "A" } else { "B" }}`
//   - 这是 Rust 表达式，不是 Show 组件
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建计数信号 count (初始 0)
    let (count, set_count) = signal(0);

    view! {
        <p>
            "数字 "
            {move || count.get()}
            " 是"
            // TODO: 用 if/else 渲染 "偶数" 或 "奇数"
            {move || if count.get() % 2 == 0 { "偶数" } else { "奇数" }}
        </p>
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "加一"
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
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (count, set_count) = signal(0);
//
//     view! {
//         <p>
//             "数字 "
//             {move || count.get()}
//             " 是"
//             {move || if count.get() % 2 == 0 { "偶数" } else { "奇数" }}
//         </p>
//         <button on:click=move |_| set_count.update(|n| *n += 1)>"加一"</button>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - view! 中可以用 `{if cond { ... } else { ... }}` 直接条件渲染
// - 返回 &str 或任何 IntoView 类型均可
// - 这种方式比 Show 更轻量，适合简单的文本/类名切换
// - 注意每个分支都要用花括号包裹且类型必须一致
// </details>
