// ============================================================
// 练习 e07: Fragment 语法 — 参考答案
//
// 核心知识点:
//   - Fragment: <></> 包裹多个兄弟根节点
//   - view! 宏支持多根节点返回
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <>
            <h2>"Fragment 语法"</h2>
            <p>"这是第一个段落"</p>
            <p>"这是第二个段落"</p>
        </>
    }
}

fn main() {
    mount_to_body(Exercise);
}
