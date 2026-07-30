// ============================================================
// 练习 e118: provide_context — 父组件提供上下文值
//
// 核心知识点:
//   - provide_context 将值注入组件树
//   - 以类型为键存储，子树中任何组件可消费
//   - 避免逐层手动传 prop（prop drilling）
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 子组件用 use_context 获取 Theme 并显示
#[component]
fn ThemedParagraph() -> impl IntoView {
    let theme = use_context::<&'static str>()
        .expect("theme should be provided by an ancestor");

    let style = match theme {
        "dark" => "background:#333; color:#fff; padding:8px; border-radius:4px;",
        _ => "background:#fff; color:#333; padding:8px; border-radius:4px;",
    };

    view! {
        <p style={style}>
            "当前主题: " {theme}
        </p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 用 provide_context 提供 &'static str 值
    provide_context("light");

    view! {
        <div style="padding:8px; border:1px solid #ccc; border-radius:4px;">
            <h3>"provide_context 示例"</h3>
            <p>"父组件通过 provide_context 注入了主题值"</p>
            <ThemedParagraph/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// use leptos::prelude::*;
//
// #[component]
// fn ThemedParagraph() -> impl IntoView {
//     let theme = use_context::<&'static str>()
//         .expect("theme should be provided");
//     let style = match theme {
//         "dark" => "background:#333; color:#fff;",
//         _ => "background:#fff; color:#333;",
//     };
//     view! { <p style=style>"主题: " {theme}</p> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     provide_context("light");
//     view! {
//         <div>
//             <h3>"provide_context"</h3>
//             <p>"父组件提供了 light 主题"</p>
//             <ThemedParagraph/>
//         </div>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - `provide_context(T)` 将类型为 T 的值注入到当前组件及其子树
// - 子树中任意层级的组件都可以通过 `use_context::<T>()` 获取
// - Context 以 Rust 类型为键，同一类型只能存在一个值（内层覆盖外层）
// - 适用于主题、用户信息、配置等全局数据传递
//
// </details>
