// ============================================================
// 答案 e353: 自定义渲染 — leptos::html 构建器 API
//
// 完整可编译实现，不含 TODO。
// 使用构建器 API 手动创建 DOM 元素，含响应式计数。
// ============================================================

use leptos::ev;
use leptos::html::{button, div, h2, p};
use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    // 使用 html::div() 构建器 API 创建完整 DOM 树
    div()
        .attr("id", "builder-root")
        .attr("class", "builder-card")
        .child(
            h2().child("答案 e353: 自定义渲染 (Builder API)"),
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
