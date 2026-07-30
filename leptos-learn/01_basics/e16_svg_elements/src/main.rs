// ============================================================
// 练习 e16: SVG 元素
//
// 核心知识点:
//   - <svg> 标签与 viewBox 属性
//   - <circle> 圆形元素
//   - <rect> 矩形元素
//   - <text> 文本元素
//
// 难度: ⭐⭐ (关键位置有 TODO — 补全约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 在 view! 中创建 SVG 图形
    // 提示: 使用 <svg> 包裹，设置 viewBox="0 0 200 100"
    // 提示: <circle> 需要 cx/cy/r/fill，<rect> 需要 x/y/width/height/fill，<text> 需要 x/y
    view! {
        <svg viewBox="0 0 200 100" xmlns="http://www.w3.org/2000/svg">
            // TODO: 添加一个红色圆形，圆心 (50, 50)，半径 30
            // 提示: <circle cx="50" cy="50" r="30" fill="red"/>

            // TODO: 添加一个蓝色矩形，位于 (90, 20)，宽 80 高 60
            // 提示: <rect x="90" y="20" width="80" height="60" fill="blue"/>

            // TODO: 在圆形下方添加白色文本"圆形"
            // 提示: text-anchor="middle"
            // 提示: <text x="50" y="90" text-anchor="middle" fill="white" font-size="12">"圆形"</text>

            // TODO: 在矩形中央添加白色文本"矩形"
            // 提示: <text x="130" y="55" text-anchor="middle" fill="white" font-size="12">"矩形"</text>
        </svg>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
