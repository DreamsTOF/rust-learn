// ============================================================
// Exercise e113: component_docs — Answer
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
