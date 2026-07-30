// ============================================================
// 练习 e277: <Style/> 组件 — 参考答案
//
// 核心知识点:
//   - <Style/> 组件定义组件级 CSS
//   - Scoped CSS 概念（样式不会泄漏到其他组件）
//   - CSS 文本作为 <Style/> 的子节点
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <style>
                ".card { padding: 1.5rem; border-radius: 8px; background: #f0f0f0; }"
                ".title { color: #2c3e50; font-size: 1.25rem; margin-bottom: 0.5rem; }"
                ".desc { color: #555; line-height: 1.6; }"
            </style>

            <div class="card">
                <h3 class="title">"组件级 CSS"</h3>
                <p class="desc">"这个组件的样式由 &lt;style&gt; 标签定义。"</p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
