// ============================================================
// 练习 e17: 原始 HTML 渲染 — 参考答案
//
// 核心知识点:
//   - inner_html 属性（设置元素 innerHTML）
//   - XSS 跨站脚本攻击防范
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let html_content = "<h2>原始 HTML 内容</h2><p style='color: green;'>这段 HTML 是通过 inner_html 渲染的。</p>";

    view! {
        <div inner_html=html_content></div>
        <p>"这是 <b>文本插值</b> — 标签会被自动转义"</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
