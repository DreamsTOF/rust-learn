// ============================================================
// 练习 e04: 元素嵌套 — 展示层级结构
//
// 核心知识点:
//   - <section> 区块元素
//   - <div> 容器元素
//   - <p> 段落元素
//   - 元素的多层嵌套
//
// 难度: ⭐ (填空题 — 每行都有 TODO 指引)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建嵌套结构: section > div > p
    // 提示: 多层缩进表示层级关系
    view! {
        // TODO: 外层 <section> — 设置 class 和 id
        // 提示: class="container", id="main"
        <section class="" id="">
            // TODO: 内层 <div> — 设置 class="card"
            <div class="">
                // TODO: 最内层 <p> — 显示文本
                // 提示: 文本用双引号包裹
                <p></p>
            </div>
        </section>
    }
}

fn main() {
    mount_to_body(Exercise);
}
