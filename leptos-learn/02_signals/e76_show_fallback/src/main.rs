// ============================================================
// 练习 e76: Show Fallback — when=false 时显示 fallback
//
// 核心知识点:
//   - <Show when fallback=|| view!{...}> 提供空值时的备用 UI
//   - fallback 是一个闭包，返回要渲染的视图
//
// 难度: ⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建布尔信号 logged_in (初始 false)
    let (logged_in, set_logged_in) = signal(false);

    view! {
        <button on:click=move |_| set_logged_in.update(|v| *v = !*v)>
            {move || if logged_in.get() { "退出" } else { "登录" }}
        </button>
        // TODO: 用 <Show when fallback> 实现登录/未登录切换
        <Show
            when=move || logged_in.get()
            fallback=|| view! { <p>"请先登录"</p> }
        >
            <p>"欢迎回来！"</p>
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
//     let (logged_in, set_logged_in) = signal(false);
//
//     view! {
//         <button on:click=move |_| set_logged_in.update(|v| *v = !*v)>
//             {move || if logged_in.get() { "退出" } else { "登录" }}
//         </button>
//         <Show
//             when=move || logged_in.get()
//             fallback=|| view! { <p>"请先登录"</p> }
//         >
//             <p>"欢迎回来！"</p>
//         </Show>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - fallback 是一个闭包 `|| -> impl IntoView`，只在 when=false 时调用
// - when=true → 渲染 children；when=false → 渲染 fallback
// - 常用于登录态切换、空状态提示等场景
// </details>
