// ============================================================
// 练习 e88: For Reorder — 按钮排序/反转触发列表重排
//
// 核心知识点:
//   - sort_by / reverse 修改 Vec 后 For 自动反映顺序
//   - 列表重排可保留 key 对应的 DOM 状态
//
// 难度: ⭐⭐⭐ (TODO 约 60%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(vec![3, 1, 4, 1, 5, 9, 2, 6]);

    let sort = move |_| {
        set_items.update(|v| v.sort());
    };

    let reverse = move |_| {
        set_items.update(|v| v.reverse());
    };

    let reset = move |_| {
        set_items.set(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    };

    view! {
        <h3>"排序与反转"</h3>
        <div style="display: flex; gap: 8px; margin-bottom: 8px;">
            <button on:click=sort>"升序排序"</button>
            <button on:click=reverse>"反转"</button>
            <button on:click=reset>"重置"</button>
        </div>
        <div style="display: flex; gap: 10px; flex-wrap: wrap;">
            // TODO: For 渲染数字列表，排序时 DOM 自动重排
            <For each=move || items.get() key=|&x| x let:n>
                <span style="border: 1px solid #888; border-radius: 4px; padding: 6px 12px; font-size: 18px;">
                    {n}
                </span>
            </For>
        </div>
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
//     let (items, set_items) = signal(vec![3, 1, 4, 1, 5, 9, 2, 6]);
//
//     let sort = move |_| set_items.update(|v| v.sort());
//     let reverse = move |_| set_items.update(|v| v.reverse());
//     let reset = move |_| set_items.set(vec![3, 1, 4, 1, 5, 9, 2, 6]);
//
//     view! {
//         <h3>"排序与反转"</h3>
//         <div style="display: flex; gap: 8px; margin-bottom: 8px;">
//             <button on:click=sort>"升序排序"</button>
//             <button on:click=reverse>"反转"</button>
//             <button on:click=reset>"重置"</button>
//         </div>
//         <div style="display: flex; gap: 10px; flex-wrap: wrap;">
//             <For each=move || items.get() key=|&x| x let:n>
//                 <span style="border: 1px solid #888; border-radius: 4px; padding: 6px 12px; font-size: 18px;">
//                     {n}
//                 </span>
//             </For>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - For 的 key 机制使 Leptos 能识别"移动"而非"重建"DOM 节点
// - sort 和 reverse 都是原地修改（update 闭包内）
// - 即使值相同但位置变了，key 相同则 Leptos 只移动 DOM 不销毁
// </details>
