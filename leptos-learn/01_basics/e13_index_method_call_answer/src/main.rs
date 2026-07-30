// ============================================================
// 练习 e13: 索引/方法调用 — 参考答案
//
// 核心知识点:
//   - { items.len() } 在 view! 中调用 Vec 的 len() 方法
//   - { items[0] } 使用索引语法访问 Vec 元素
//   - 普通 Rust 表达式（非响应式）在 view! 中可直接嵌入
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let items = vec!["Rust", "Leptos", "WASM"];

    view! {
        <div>
            <h2>"编程语言列表"</h2>
            <p>"共有 " { items.len() } " 门语言"</p>
            <p>"第一门语言: " { items[0] }</p>
            <p>"第二门语言: " { items[1] }</p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
