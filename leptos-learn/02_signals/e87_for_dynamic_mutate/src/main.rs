// ============================================================
// 练习 e87: For Dynamic Mutate — 按钮动态添加/删除列表项
//
// 核心知识点:
//   - For 搭配信号 Vec，push / remove 后自动更新 DOM
//   - 通过 set_items.update() 原地修改
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(vec!["A", "B", "C"]);

    let add_item = move |_| {
        set_items.update(|v| v.push("新项"));
    };

    let remove_item = move |_| {
        set_items.update(|v| {
            v.pop();
        });
    };

    view! {
        <h3>"动态列表"</h3>
        <div style="display: flex; gap: 8px; margin-bottom: 8px;">
            <button on:click=add_item>"添加"</button>
            <button on:click=remove_item>"删除末尾"</button>
        </div>
        <ul>
            // TODO: For 响应 items 信号自动更新
            <For each=move || items.get() key=|&x| x let:item>
                <li>{item}</li>
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
// #[component]
// fn Exercise() -> impl IntoView {
//     let (items, set_items) = signal(vec!["A", "B", "C"]);
//
//     let add_item = move |_| set_items.update(|v| v.push("新项"));
//     let remove_item = move |_| set_items.update(|v| { v.pop(); });
//
//     view! {
//         <h3>"动态列表"</h3>
//         <div style="display: flex; gap: 8px; margin-bottom: 8px;">
//             <button on:click=add_item>"添加"</button>
//             <button on:click=remove_item>"删除末尾"</button>
//         </div>
//         <ul>
//             <For each=move || items.get() key=|&x| x let:item>
//                 <li>{item}</li>
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
// - For 的 each 接收响应式闭包，信号变化时自动重渲染
// - set_items.update(|v| ...) 原地修改 Vec
// - 删除末尾只影响最后一个 DOM 节点，Leptos 做最小更新
// </details>
