// ============================================================
// 练习 e04: 元素嵌套 — 参考答案
//
// 核心知识点:
//   - <section> 区块元素
//   - <div> 容器元素
//   - <p> 段落元素
//   - 元素的多层嵌套
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 嵌套结构: section > div > p，多层缩进表示层级关系
    view! {
        // 外层 <section> — 设置 class 和 id
        <section class="container" id="main">
            // 内层 <div> — 设置 class="card"
            <div class="card">
                // 最内层 <p> — 显示文本
                <p>"这是嵌套在 section > div 内的段落"</p>
            </div>
        </section>
    }
}

fn main() {
    mount_to_body(Exercise);
}
