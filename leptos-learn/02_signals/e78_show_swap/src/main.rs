// ============================================================
// 练习 e78: Show Swap — when=true 显示 A，否则 B
//
// 核心知识点:
//   - Show 配合 fallback 实现两视图互斥切换
//   - 两个视图可以是对等的组件而不是"内容 vs 提示"
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建布尔信号 show_a (初始 true)
    let (show_a, set_show_a) = signal(true);

    view! {
        <button on:click=move |_| set_show_a.update(|v| *v = !*v)>
            "切换视图"
        </button>
        // TODO: 用 Show 实现 A/B 视图切换
        <Show
            when=move || show_a.get()
            fallback=|| view! {
                <div style="padding: 1rem; background: #e8f5e9;">
                    <p>"这是视图 B"</p>
                    <small>"绿色背景"</small>
                </div>
            }
        >
            <div style="padding: 1rem; background: #e3f2fd;">
                <p>"这是视图 A"</p>
                <small>"蓝色背景"</small>
            </div>
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
//     let (show_a, set_show_a) = signal(true);
//
//     view! {
//         <button on:click=move |_| set_show_a.update(|v| *v = !*v)>"切换视图"</button>
//         <Show
//             when=move || show_a.get()
//             fallback=|| view! {
//                 <div style="padding: 1rem; background: #e8f5e9;">
//                     <p>"这是视图 B"</p>
//                     <small>"绿色背景"</small>
//                 </div>
//             }
//         >
//             <div style="padding: 1rem; background: #e3f2fd;">
//                 <p>"这是视图 A"</p>
//                 <small>"蓝色背景"</small>
//             </div>
//         </Show>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - children 和 fallback 各自包含完整的视图结构
// - 点击按钮在两个视图之间平滑切换
// - 与 e76 不同，此处两个视图是对等的"组件"
// </details>
