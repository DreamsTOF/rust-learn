// ============================================================
// 练习 e03: HTML 元素与属性
//
// 核心知识点:
//   - class、id、style 属性
//   - <a> 链接元素
//   - <img> 图片元素
//
// 难度: ⭐ (填空题 — 每行都有 TODO 指引)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 在 view! 中创建含多种 HTML 元素的页面
    view! {
        // TODO: 第 1 个 <h1> — 使用 id 和 class 属性
        // 提示: 属性语法与 HTML 一致: id="值", class="值"
        <h1 id="" class=""></h1>

        // TODO: 第 1 个 <p> — 使用 style 属性设置内联样式
        // 提示: style="color: 值; font-size: 值"
        <p style=""></p>

        // TODO: 第 1 个 <a> — 链接元素，使用 href 属性
        // 提示: 用 target="_blank" 在新标签页打开
        <a href="" target=""></a>

        // TODO: 第 1 个 <br/> — 换行（自闭合标签）
        <br/>

        // TODO: 第 1 个 <img> — 图片元素，使用 src 和 alt 属性
        // 提示: src 使用占位图服务，alt 提供替代文本
        <img src="" alt=""/>
    }
}

fn main() {
    mount_to_body(Exercise);
}
