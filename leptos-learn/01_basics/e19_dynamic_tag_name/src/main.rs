// ============================================================
// 练习 e19: 动态标签名
//
// 核心知识点:
//   - leptos::html::h1、leptos::html::h2 等动态标签函数
//   - 根据信号值动态选择 HTML 标签
//   - into_any() 统一不同标签类型
//
// 难度: ⭐⭐ (关键位置有 TODO — 补全约 50%)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 创建信号跟踪当前标题级别
    let (level, set_level) = signal(1);

    view! {
        // TODO: 根据 level() 的值动态渲染 h1/h2/h3 标签
        // 提示: 使用 leptos::html::h1/h2/h3 函数创建元素
        // 提示: 不同标签类型用 .into_any() 统一
        {match level() {
            1 => leptos::html::h1()
                .child(format!("标题级别 {}", level()))
                .into_any(),
            2 => leptos::html::h2()
                .child(format!("标题级别 {}", level()))
                .into_any(),
            3 => leptos::html::h3()
                .child(format!("标题级别 {}", level()))
                .into_any(),
            _ => leptos::html::h1()
                .child(format!("标题级别 {}", level()))
                .into_any(),
        }}

        <p>"当前标题级别: " {level()}</p>

        // TODO: 添加三个按钮，分别将 level 设置为 1、2、3
        <button on:click=move |_| set_level(1)>"h1"</button>
        <button on:click=move |_| set_level(2)>"h2"</button>
        <button on:click=move |_| set_level(3)>"h3"</button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (level, set_level) = signal(1);
//
//     view! {
//         {match level() {
//             1 => leptos::html::h1()
//                 .child(format!("标题级别 {}", level()))
//                 .into_any(),
//             2 => leptos::html::h2()
//                 .child(format!("标题级别 {}", level()))
//                 .into_any(),
//             3 => leptos::html::h3()
//                 .child(format!("标题级别 {}", level()))
//                 .into_any(),
//             _ => leptos::html::h1()
//                 .child(format!("标题级别 {}", level()))
//                 .into_any(),
//         }}
//
//         <p>"当前标题级别: " {level()}</p>
//
//         <button on:click=move |_| set_level(1)>"h1"</button>
//         <button on:click=move |_| set_level(2)>"h2"</button>
//         <button on:click=move |_| set_level(3)>"h3"</button>
//     }
// }
//
// fn main() {
//     console_error_panic_hook::set_once();
//     mount_to_body(Exercise);
// }
//
// 知识点:
// - leptos::html::h1/h2/h3 是构建器函数，返回不同 HtmlElement 类型
// - .into_any() 将具体类型擦除为 AnyView，使 match 分支类型统一
// - 动态标签适用于需要根据数据切换 HTML 语义标签的场景
// </details>
