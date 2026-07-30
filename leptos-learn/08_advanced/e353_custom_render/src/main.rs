// ============================================================
// 练习 e353: 自定义渲染 — 使用 leptos::html 构建器 API
//
// 核心知识点:
//   - leptos::html::div / p / h2 / button 等构建器函数
//   - .child() 链式添加子元素
//   - .attr() 设置 HTML 属性
//   - .on() 绑定事件处理
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::ev;
use leptos::html::{button, div, h2, p};
use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 使用 html::div() 构建器创建根容器
    //   - 添加 h2 标题："练习 e353: 自定义渲染 (Builder API)"
    //   - 添加 p 描述："这是通过 leptos::html 构建器 API 手动创建的 DOM 元素"
    //   - 添加 p 显示 count 值
    //   - 添加 button，点击时 count+1
    //
    // 提示: .child() 可嵌套调用，最终整个链直接 impl IntoView
    div()
        .attr("id", "builder-root")
        .attr("class", "builder-card")
        .child(
            h2().child("练习 e353: 自定义渲染 (Builder API)"),
        )
        .child(
            p().child(
                "这是通过 leptos::html 构建器 API 手动创建的 DOM 元素。",
            ),
        )
        .child(
            p().child(("计数: ", move || count().to_string())),
        )
        .child(
            button()
                .child("增加计数")
                .on(ev::click, move |_| set_count.update(|c| *c += 1)),
        )
}

fn main() {
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 完整代码
// ```rust
// use leptos::ev;
// use leptos::html::{button, div, h2, p};
// use leptos::prelude::*;
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (count, set_count) = signal(0);
//
//     div()
//         .attr("id", "builder-root")
//         .attr("class", "builder-card")
//         .child(h2().child("练习 e353: 自定义渲染 (Builder API)"))
//         .child(p().child("这是通过 leptos::html 构建器 API 手动创建的 DOM 元素。"))
//         .child(p().child(("计数: ", move || count().to_string())))
//         .child(button().child("增加计数").on(ev::click, move |_| set_count.update(|c| *c += 1)))
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - `html::div()` 等构建器函数返回 `HtmlElement<T>`，直接实现 `IntoView`
// - `.child()` 可接受静态文本、闭包、嵌套构建器、元组等多种形式
// - `.attr("name", "value")` 设置 HTML 属性
// - `.on(ev::click, handler)` 绑定 DOM 事件
// - 构建器 API 优势：类型安全、编译快、无宏依赖、IDE 自动补全友好
// - 与 `view!` 宏对比：view! 更简洁，builder 更灵活
//
// </details>
