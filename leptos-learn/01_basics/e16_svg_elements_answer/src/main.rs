// ============================================================
// 练习 e16: SVG 元素 — 参考答案
//
// 核心知识点:
//   - <svg> 标签与 viewBox 属性
//   - <circle> 圆形元素
//   - <rect> 矩形元素
//   - <text> 文本元素
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <svg viewBox="0 0 200 100" xmlns="http://www.w3.org/2000/svg">
            <circle cx="50" cy="50" r="30" fill="red"/>
            <rect x="90" y="20" width="80" height="60" fill="blue"/>
            <text x="50" y="90" text-anchor="middle" fill="white" font-size="12">"圆形"</text>
            <text x="130" y="55" text-anchor="middle" fill="white" font-size="12">"矩形"</text>
        </svg>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
