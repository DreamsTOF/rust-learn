// ============================================================
// 练习 e84: For Empty Fallback — 空列表 fallback
//
// 核心知识点:
//   - 用 <Show when fallback> 包裹 <For> 实现空列表提示
//   - For 本身无 fallback，需要外层 Show 配合
//
// 难度: ⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建信号 items（Vec<&str>），初始为空
    let (items, set_items) = signal(Vec::<&str>::new());

    view! {
        <h3>"待办事项"</h3>

        <button on:click=move |_| set_items.update(|v| v.push("新事项"))>
            "➕ 添加"
        </button>
        <button on:click=move |_| set_items.set(Vec::new())>
            "🗑 清空"
        </button>

        // TODO: 用 Show 的 fallback 处理空列表，显示 "暂无待办事项"
        <Show when=move || !items.get().is_empty()
            fallback=|| view! { <p style="color: #999;">"📭 暂无待办事项"</p> }
        >
            <For each=move || items.get() key=|item| *item let:item>
                <p style="margin: 4px 0;">"📋 " {item}</p>
            </For>
        </Show>
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
//     let (items, set_items) = signal(Vec::<&str>::new());
//
//     view! {
//         <h3>"待办事项"</h3>
//         <button on:click=move |_| set_items.update(|v| v.push("新事项"))>"➕ 添加"</button>
//         <button on:click=move |_| set_items.set(Vec::new())>"🗑 清空"</button>
//         <Show when=move || !items.get().is_empty()
//             fallback=|| view! { <p style="color: #999;">"📭 暂无待办事项"</p> }
//         >
//             <For each=move || items.get() key=|item| *item let:item>
//                 <p style="margin: 4px 0;">"📋 " {item}</p>
//             </For>
//         </Show>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - Leptos 的 For 组件不直接支持 fallback；用外层 Show 实现空列表提示
// - Show 的 when 检查 `items.get().is_empty()` 判断列表是否为空
// - 当列表为空时 Show 渲染 fallback，有数据时渲染 For 列表
// </details>
