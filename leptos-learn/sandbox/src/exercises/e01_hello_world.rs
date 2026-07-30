// ============================================================
// 练习 e01: Hello, Leptos! — 最简单的 Leptos 应用
//
// 核心知识点:
//   - mount_to_body: 将组件挂载到 <body>
//   - view! 宏: 编写声明式 UI
//   - #[component]: 标记组件函数
//
// 难度: ⭐ (填空题 — 每行都有 TODO 指引)
// ============================================================

use leptos::prelude::*;

// TODO: 使用 #[component] 属性标记此函数为组件
// 提示: 属性放在 fn 之前，组件名使用 PascalCase
#[component]
pub fn Exercise() -> impl IntoView {
    // TODO: 在 view! 宏的 <p> 标签中显示 "Hello, Leptos!"
    // 提示: 文本内容用双引号包裹，例如 "文本"
    view! {
        <p>"Hello, Leptos!"</p>
    }
}
