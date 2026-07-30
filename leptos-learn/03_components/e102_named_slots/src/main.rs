// ============================================================
// 练习 e102: Named Slots — 具名插槽
//
// 核心知识点:
//   - 通过独立的 prop 参数实现具名插槽
//   - 使用 Fn() -> AnyView 泛型约束接收闭包插槽
//   - 闭包插槽通过 .into_any() 统一返回类型
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// BlogPost 组件有三个插槽：
//   - title: 标题区（接收闭包）
//   - meta: 元信息区（接收闭包）
//   - children: 正文区（默认插槽，接收 Children）
//
// title 和 meta 使用泛型 T / M 约束为 Fn() -> AnyView，
// 这样调用者可以用 move || view! { ... }.into_any() 传入任意视图
#[component]
fn BlogPost<T, M>(
    title: T,
    meta: M,
    children: Children,
) -> impl IntoView
where
    T: Fn() -> AnyView + Send + Sync + 'static,
    M: Fn() -> AnyView + Send + Sync + 'static,
{
    view! {
        <article style="border:1px solid #ddd;padding:16px;margin:10px 0;border-radius:8px;">
            <header style="border-bottom:1px solid #eee;padding-bottom:8px;margin-bottom:8px;">
                {title()}
            </header>
            <div style="color:#888;font-size:0.9em;margin:8px 0;">
                {meta()}
            </div>
            <section>{children()}</section>
        </article>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 BlogPost 组件，传入 title、meta 和默认子节点
    view! {
        <BlogPost
            title=move || view! { <h1>"Rust 学习笔记"</h1> }.into_any()
            meta=move || view! { <span>"发布于 2024-01-15"</span> }.into_any()
        >
            <p>"所有权系统是 Rust 最独特的特性之一。"</p>
            <p>"它让 Rust 在不使用垃圾回收器的前提下保证内存安全。"</p>
        </BlogPost>
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
// fn BlogPost<T, M>(
//     title: T,
//     meta: M,
//     children: Children,
// ) -> impl IntoView
// where
//     T: Fn() -> AnyView + Send + Sync + 'static,
//     M: Fn() -> AnyView + Send + Sync + 'static,
// {
//     view! {
//         <article style="border:1px solid #ddd;padding:16px;margin:10px 0;border-radius:8px;">
//             <header style="border-bottom:1px solid #eee;padding-bottom:8px;margin-bottom:8px;">
//                 {title()}
//             </header>
//             <div style="color:#888;font-size:0.9em;margin:8px 0;">
//                 {meta()}
//             </div>
//             <section>{children()}</section>
//         </article>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <BlogPost
//             title=move || view! { <h1>"Rust 学习笔记"</h1> }.into_any()
//             meta=move || view! { <span>"发布于 2024-01-15"</span> }.into_any()
//         >
//             <p>"所有权系统是 Rust 最独特的特性之一。"</p>
//             <p>"它让 Rust 在不使用垃圾回收器的前提下保证内存安全。"</p>
//         </BlogPost>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// </details>
