// ============================================================
// 练习 e14: 无宏构建器模式 — 参考答案
//
// 核心知识点:
//   - div().child("text").on(ev::click, handler) 使用构建器 API
//   - 构建器链直接实现 IntoView，无需 .build()
//   - 构建器 API 优点：类型安全、编译快、无需宏
// ============================================================

use leptos::ev;
use leptos::html::{button, div, h1, p};
use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    div()
        .child(h1().child("构建器模式"))
        .child(p().child("使用构建器 API 创建，无需 view! 宏"))
        .child(button().child("点击: ").child(count).on(ev::click, move |_| set_count(count() + 1)))
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
