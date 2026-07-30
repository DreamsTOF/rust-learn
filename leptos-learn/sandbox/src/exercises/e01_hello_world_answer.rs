// ============================================================
// 练习 e01: Hello, Leptos! — 参考答案
//
// 核心知识点:
//   - mount_to_body: 将组件挂载到 <body>
//   - view! 宏: 编写声明式 UI
//   - #[component]: 标记组件函数
// ============================================================

use leptos::prelude::*;

#[component]
pub fn Exercise() -> impl IntoView {
    view! {
        <p>"Hello, Leptos!"</p>
    }
}
