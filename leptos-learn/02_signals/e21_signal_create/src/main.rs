// ============================================================
// 练习 21: signal_create
//
// 目标: 使用 signal() 创建信号并在视图中显示
//
// 难度: ⭐
// 核心知识点: signal() 创建信号
// ============================================================
use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 使用 signal(0) 创建计数信号 ===
    let (count, _set_count) = signal(0);

    view! {
        <div>
            <p>"计数: " {count}</p>
        </div>
    }
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
// ### 代码
// ```rust
// let (count, set_count) = signal(0);
// view! { <p>{count}</p> }
// ```
//
// ### 知识点
// - `signal(初始值)` 返回元组 `(ReadSignal, WriteSignal)`
// - 在 view 中直接使用信号名，leptos 会自动读取其值
//
// </details>
