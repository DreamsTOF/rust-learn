// ============================================================
// 练习 e81: For Basic — <For each=items> 基础列表渲染
//
// 核心知识点:
//   - <For each=move || items.get()> 遍历响应式列表
//   - key 属性为每个元素提供唯一标识
//
// 难度: ⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号 items 存储 Vec<&str>，初始含 "苹果"、"香蕉"、"橘子"
    let (items, set_items) = signal(vec!["苹果", "香蕉", "橘子"]);

    view! {
        <h3>"水果列表"</h3>
        // TODO: 用 <For each=items> 渲染列表，以每个元素自身为 key
        <For each=move || items.get() key=|item| *item let:item>
            <p style="margin: 4px 0;">"🍎 " {item}</p>
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
//     let (items, set_items) = signal(vec!["苹果", "香蕉", "橘子"]);
//
//     view! {
//         <h3>"水果列表"</h3>
//         <For each=move || items.get() key=|item| *item let:item>
//             <p style="margin: 4px 0;">"🍎 " {item}</p>
//         </For>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - For 组件接收 three 个关键 prop: each（数据源）、key（唯一标识）、children（渲染函数）
// - each 是一个返回 IntoIterator 的闭包，保证响应式追踪
// - key 帮助 Leptos 高效对比新旧列表，只更新变化的部分
// - 简单场景可直接用元素自身（&str）作 key
// </details>
