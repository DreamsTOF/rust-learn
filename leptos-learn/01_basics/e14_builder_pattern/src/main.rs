use leptos::prelude::*;

// ============================================================
// 练习 e14 — 无宏构建器模式
// 目标: 使用构建器 API 替代 view! 宏创建元素
// 难度: ⭐⭐⭐
// 核心知识点: div().child("text").on(ev::click, ...)
// ============================================================

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Exercise/> });
}

/// TODO: 使用 html::div() 等构建器替代 view! 宏
/// 链式调用 .child().on() 构建 DOM 树
/// 注意: 构建器返回的类型已实现 IntoView，无需调用 .build()
#[component]
fn Exercise() -> impl IntoView {
    // ========== 学生需要补全以下代码 ==========
    // TODO: 参考 answer 目录完成以下任务:
    //
    // 1. 添加必要的导入:
    //    use leptos::ev;
    //    use leptos::html::{button, div, h1, p};
    //
    // 2. 创建信号: let (count, set_count) = signal(0);
    //
    // 3. 使用构建器 API 构建 DOM 树:
    //    div()
    //        .child(h1().child("标题"))
    //        .child(p().child("描述"))
    //        .child(button().child("点击")
    //            .on(ev::click, move |_| set_count(count() + 1)))
    // ==========================================

    // 临时占位，完成任务后请删除以下内容
    let _placeholder = "请完成练习内容";
    view! {
        <p>{ _placeholder }</p>
    }
}
