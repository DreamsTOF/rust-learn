// ============================================================
// 练习 e86: For Keys Closure — 用闭包为 For 提供 key
//
// 核心知识点:
//   - key 属性接收 |item| item.id 形式的闭包
//   - 返回类型须实现 Eq + Hash
//   - 自定义 key 可控制列表 diff 行为
//
// 难度: ⭐⭐ (TODO 约 60%)
// ============================================================

use leptos::prelude::*;

#[derive(Debug, Clone)]
struct Item {
    id: u32,
    name: &'static str,
}

#[component]
fn Exercise() -> impl IntoView {
    let items = vec![
        Item { id: 1, name: "苹果" },
        Item { id: 2, name: "香蕉" },
        Item { id: 3, name: "樱桃" },
    ];

    view! {
        <h3>"水果列表"</h3>
        <ul>
            // TODO: 用 key=|item| item.id 显式指定键
            <For each=move || items.clone() key=|item| item.id let:item>
                <li>{item.name} " (ID: " {item.id} ")"</li>
            </For>
        </ul>
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
// #[derive(Debug, Clone)]
// struct Item {
//     id: u32,
//     name: &'static str,
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let items = vec![
//         Item { id: 1, name: "苹果" },
//         Item { id: 2, name: "香蕉" },
//         Item { id: 3, name: "樱桃" },
//     ];
//
//     view! {
//         <h3>"水果列表"</h3>
//         <ul>
//             <For each=move || items.clone() key=|item| item.id let:item>
//                 <li>{item.name} " (ID: " {item.id} ")"</li>
//             </For>
//         </ul>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - key 闭包返回的类型必须实现 Eq + Hash
// - Leptos 用 key 追踪每个元素的身份，实现最小化 DOM 更新
// - 合理的 key 可以在列表变更时维持组件状态
// </details>
