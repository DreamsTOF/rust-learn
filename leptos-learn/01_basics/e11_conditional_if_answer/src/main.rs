// ============================================================
// 练习 e11: 条件 if 在 view 中 — 参考答案
//
// 核心知识点:
//   - { if cond { "A" } else { "B" } } 在 view! 中直接嵌入条件表达式
//   - signal(bool) 创建布尔响应式信号，show() 读取当前值
//   - 条件表达式是 Rust 原生语法，Leptos 的 view! 宏支持直接嵌入
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (show, set_show) = signal(false);

    view! {
        <div>
            <p>
                "当前状态: "
                { if show() { "已激活" } else { "未激活" } }
            </p>
            <button on:click=move |_| set_show(!show())>
                "切换状态"
            </button>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
