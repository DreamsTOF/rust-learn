// ============================================================
// 练习 e113: component_docs — 组件文档注释
//
// 核心知识点:
//   - /// 组件说明文档
//   - 为组件添加文档注释，IDE 和 rustdoc 会自动显示
//
// 难度: ⭐
// ============================================================

use leptos::prelude::*;

/// 一个简单的问候组件
///
/// 向用户显示带有名字的个性化问候语。
///
/// # 参数
/// - `name`: 要问候的用户名
///
/// # 示例
/// ```rust
/// view! {
///     <Greeting name="小明" />
/// }
/// ```
#[component]
fn Greeting(name: &'static str) -> impl IntoView {
    view! {
        <p>"你好, " {name} "！欢迎来到 Leptos！"</p>
    }
}

/// 主练习组件
///
/// 展示文档注释在 Leptos 组件中的用法。
/// 在 IDE 中将鼠标悬停在 `Greeting` 组件名称上可以查看其文档。
#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h3>"组件文档注释"</h3>
            // TODO: 使用 Greeting 组件多次
            <Greeting name="小明" />
            <Greeting name="小红" />
            <p style="color: #888; font-size: 14px;">
                "提示：在 IDE 中将鼠标悬停在 Greeting 组件上查看文档"
            </p>
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
// /// 一个简单的问候组件
// ///
// /// 向用户显示带有名字的个性化问候语。
// ///
// /// # 参数
// /// - `name`: 要问候的用户名
// #[component]
// fn Greeting(name: &'static str) -> impl IntoView {
//     view! {
//         <p>"你好, " {name} "！欢迎来到 Leptos！"</p>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <h3>"组件文档注释"</h3>
//             <Greeting name="小明" />
//             <Greeting name="小红" />
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - `///` 是 Rust 文档注释，会被 rustdoc 和 IDE 识别
// - `#[component]` 宏会自动将函数上的文档注释转为组件文档
// - 文档注释支持 Markdown 格式，可以写参数说明、示例等
// - 好的文档让组件更易用，是生产项目的重要习惯
//
// </details>
