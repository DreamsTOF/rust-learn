// ============================================================
// 练习 29: signal_update — .update() 原地更新
//
// 目标: 按钮调用 count.update(|n| *n += 1) 递增
//
// 难度: ⭐⭐
// 核心知识点:
//   - .update() 通过闭包可变借用原地修改值
//   - 适用于基于当前值计算新值的场景
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 使用 signal(0) 创建信号
    let (count, set_count) = signal(0);

    view! {
        <div>
            <h1>"练习 29: signal.update()"</h1>
            <p>"当前值: " {count}</p>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "递增 (+1)"
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
//     <button on:click=move |_| set_count.update(|n| *n += 1)>
//         "递增 (+1)"
//     </button>
// }
// ```
//
// ### 知识点
// - `.update(|n| ...)` 提供 `&mut T` 给闭包，直接修改内部值。
// - 与 `.set()` 不同，`.update()` 不需要先读取再写回，
//   适合需要对值做增量修改的场景（如计数器、数组 push 等）。
// - 闭包执行完后自动触发一次通知。
//
// </details>
