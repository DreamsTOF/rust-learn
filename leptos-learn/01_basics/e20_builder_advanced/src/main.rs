// ============================================================
// 练习 e20: 构建器模式高级
//
// 核心知识点:
//   - 纯构建器 API: div().child(...).on(...).build()
//   - 事件监听器: .on(ev::click, move |_| ...)
//   - 样式与方法链
//
// 难度: ⭐⭐⭐ (仅描述目标 — 几乎全部自己写)
// ============================================================
//
// 目标: 完全使用构建器 API 创建一个带事件监听和样式的交互组件
//   1. 使用 div() 作为根容器
//   2. 使用 .child() 添加子元素
//   3. 使用 .on(ev::click, ...) 添加点击事件
//   4. 使用 .attr() 或 style 设置样式
//   5. 最后完成构建器链（无需 .build()）
// 要求: 不要使用 view! 宏
//
// 注意: 构建器链直接实现 IntoView，无需调用 .build()

use leptos::html::{button, div, h2, p};
use leptos::{ev, prelude::*};

#[component]
fn Exercise() -> impl IntoView {
    // 创建计数器信号
    let (count, set_count) = signal(0);

    // 使用纯构建器 API 构建 UI
    div()
        .child(h2().child("构建器模式高级"))
        .child(p().child(format!("计数: {}", count())))
        .child(
            button()
                .child("增加")
                .on(ev::click, move |_| {
                    set_count(count() + 1);
                }),
        )
        .child(
            button()
                .child("重置")
                .attr("style", "margin-left: 8px;")
                .on(ev::click, move |_| {
                    set_count(0);
                }),
        )
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::html::{button, div, h2, p};
// use leptos::{ev, prelude::*};
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (count, set_count) = signal(0);
//
//     div()
//         .child(h2().child("构建器模式高级"))
//         .child(p().child(format!("计数: {}", count())))
//         .child(
//             button()
//                 .child("增加")
//                 .on(ev::click, move |_| set_count(count() + 1)),
//         )
//         .child(
//             button()
//                 .child("重置")
//                 .attr("style", "margin-left: 8px;")
//                 .on(ev::click, move |_| set_count(0)),
//         )
// }
//
// fn main() {
//     console_error_panic_hook::set_once();
//     mount_to_body(Exercise);
// }
// </details>
