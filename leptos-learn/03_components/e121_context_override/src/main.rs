// ============================================================
// 练习 e121: 嵌套 Context 覆盖 (context_override)
//
// 核心知识点:
//   - provide_context 在组件树中按类型覆盖
//   - 内层 provide_context 会隐藏外层的同类型值
//   - use_context 查找最近祖先的提供
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Theme {
    Light,
    Dark,
}

// TODO: 使用 use_context 获取 Theme 并显示
#[component]
fn ThemeDisplay() -> impl IntoView {
    let theme = use_context::<Theme>()
        .expect("Theme should be provided by an ancestor");

    view! {
        <p>"Current theme: " {format!("{:?}", theme)}</p>
    }
}

// TODO: 在内层提供 Theme::Dark，覆盖外层
#[component]
fn InnerSection() -> impl IntoView {
    provide_context(Theme::Dark);

    view! {
        <div style="border: 1px solid blue; padding: 8px; margin: 8px 0;">
            <h3>"Inner Section"</h3>
            <p>"内层提供 Theme::Dark"</p>
            <ThemeDisplay/>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 在此外层提供 Theme::Light
    provide_context(Theme::Light);

    view! {
        <div style="border: 1px solid gray; padding: 8px;">
            <h2>"Context Override Demo"</h2>
            <p>"外层提供 Light，内层提供 Dark，内层覆盖外层"</p>
            <ThemeDisplay/>
            <InnerSection/>
            <p>"回到外层后:" <ThemeDisplay/></p>
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
// #[derive(Clone, Debug, PartialEq)]
// enum Theme { Light, Dark }
//
// #[component]
// fn ThemeDisplay() -> impl IntoView {
//     let theme = use_context::<Theme>()
//         .expect("Theme should be provided");
//     view! { <p>"Theme: " {format!("{:?}", theme)}</p> }
// }
//
// #[component]
// fn InnerSection() -> impl IntoView {
//     provide_context(Theme::Dark);
//     view! {
//         <div>
//             <h3>"Inner"</h3>
//             <ThemeDisplay/>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     provide_context(Theme::Light);
//     view! {
//         <div>
//             <ThemeDisplay/>
//             <InnerSection/>
//             <ThemeDisplay/>
//         </div>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - provide_context 以类型为键存储值
// - 子组件调用 provide_context 会覆盖父组件的同类型值
// - use_context 返回最近祖先提供的值 (Option<T>)
// - 覆盖只在提供者的子树内生效，回到外层后恢复
//
// </details>
