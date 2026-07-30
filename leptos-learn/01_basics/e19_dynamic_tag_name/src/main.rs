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
        // 提示: 使用 match + leptos::html::h1/h2/h3 + .into_any()
        { "请补全 match 表达式" }

        <p>"当前标题级别: " {level()}</p>

        // TODO: 添加三个按钮，分别将 level 设置为 1、2、3
        // 提示: <button on:click=move |_| set_level(N)>
        <button>"h1"</button>
        <button>"h2"</button>
        <button>"h3"</button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
