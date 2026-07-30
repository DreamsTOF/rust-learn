// ============================================================
// 练习 e03: HTML 元素与属性 — 参考答案
//
// 核心知识点:
//   - class、id、style 属性
//   - <a> 链接元素
//   - <img> 图片元素
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        // 第 1 个 <h1> — 使用 id 和 class 属性
        <h1 id="title" class="heading">"HTML 元素与属性"</h1>

        // 第 1 个 <p> — 使用 style 属性设置内联样式
        <p style="color: blue; font-size: 18px;">"这是一个带样式的段落"</p>

        // 第 1 个 <a> — 链接元素，使用 href 属性，target="_blank" 在新标签页打开
        <a href="https://leptos.dev" target="_blank">"访问 Leptos 官网"</a>

        // 第 1 个 <br/> — 换行（自闭合标签）
        <br/>

        // 第 1 个 <img> — 图片元素，使用 src 和 alt 属性
        <img src="https://placehold.co/200x100" alt="占位图片"/>
    }
}

fn main() {
    mount_to_body(Exercise);
}
