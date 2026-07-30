// ============================================================
// 练习 28: fn_call_syntax_set — 函数调用语法设置
//
// 目标: 按钮调用 set_count(42) 修改值
//
// 难度: ⭐
// 核心知识点:
//   - 函数调用语法 set_count(new_value) 等价于 .set()
//   - nightly 特性下的便捷写法
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 使用 signal(0) 创建信号
    let (count, set_count) = signal(0);

    view! {
        <div>
            <h1>"练习 28: 函数调用语法 set"</h1>
            <p>"当前值: " {count}</p>
            <button on:click=move |_| set_count(42)>
                "set_count(42)"
            </button>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
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
//
// view! {
//     <p>"当前值: " {count}</p>
//     <button on:click=move |_| set_count(42)>
//         "set_count(42)"
//     </button>
// }
// ```
//
// ### 知识点
// - `set_count(42)` 与 `set_count.set(42)` 完全等价。
// - 这是通过 nightly Rust 的 `FnOnce`、`FnMut`、`Fn` 实现，
//   `WriteSignal<i32>` 实现了这些 trait，使它可以像函数一样被调用。
// - 同理，`count()` 函数调用语法也可用于读取（练习 23）。
//
// </details>
