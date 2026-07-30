// ============================================================
// 练习 e75: Show Basic — 用 Show 组件条件渲染内容
//
// 核心知识点:
//   - <Show when=cond> 根据信号布尔值控制显示/隐藏
//   - when=true 时显示子内容，false 时卸载
//
// 难度: ⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建布尔信号 visible (初始 true)
    let (visible, set_visible) = signal(true);

    view! {
        <button on:click=move |_| set_visible.update(|v| *v = !*v)>
            "切换显示"
        </button>
        // TODO: 用 <Show when=visible> 包裹要条件渲染的内容
        <Show when=move || visible.get()>
            <p>"现在你看到我了 👋"</p>
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
//     let (visible, set_visible) = signal(true);
//
//     view! {
//         <button on:click=move |_| set_visible.update(|v| *v = !*v)>"切换显示"</button>
//         <Show when=move || visible.get()>
//             <p>"现在你看到我了 👋"</p>
//         </Show>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - Show 的 when 接收一个返回 bool 的闭包（响应式）
// - when=true 时渲染 children，false 时从 DOM 卸载
// - 配合按钮 toggle 可以看到内容的显示/隐藏
// </details>
