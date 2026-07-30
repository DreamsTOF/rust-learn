// ============================================================
// 练习 e277: <Style/> 组件 — 组件级 CSS 样式
//
// 核心知识点:
//   - <Style/> 组件定义组件级 CSS
//   - Scoped CSS 概念（样式不会泄漏到其他组件）
//   - CSS 文本作为 <Style/> 的子节点
//
// 难度: ⭐⭐ (补全关键代码)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            // TODO: 使用 <Style/> 组件定义以下 CSS 类:
            //   .card: padding: 1.5rem, border-radius: 8px, background: #f0f0f0
            //   .title: color: #2c3e50, font-size: 1.25rem, margin-bottom: 0.5rem
            //   .desc: color: #555, line-height: 1.6
            // 提示: <Style>{"CSS 规则..."}</Style>
            <Style>
            </Style>

            // TODO: 为 div 添加 class="card"
            <div>
                // TODO: 为 h3 添加 class="title"
                <h3>"组件级 CSS"</h3>
                // TODO: 为 p 添加 class="desc"
                <p>"这个组件的样式由 &lt;Style/&gt; 组件定义，不会泄漏到其他组件。"</p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
