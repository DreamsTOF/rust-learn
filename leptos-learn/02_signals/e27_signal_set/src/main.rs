// ============================================================
// 练习 27: signal_set — .set() 设置
//
// 目标: 按钮调用 count.set(42) 修改值
//
// 难度: ⭐
// 核心知识点:
//   - .set(new_value) 直接替换信号的值
//   - 按钮点击事件驱动信号变更
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // ──── 步骤 1 ──────────────────────────────────────────────────
    // TODO: 使用 signal(0) 创建信号
    let (count, set_count) = signal(0);

    view! {
        <div>
            <h1>"练习 27: signal.set()"</h1>
            <p>"当前值: " {count}</p>
            <button on:click=move |_| set_count.set(42)>
                "设值为 42"
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
//     <button on:click=move |_| set_count.set(42)>
//         "设值为 42"
//     </button>
// }
// ```
//
// ### 知识点
// - .set(val) 直接将信号值替换为 val, 触发所有订阅者更新。
// - 在 Leptos 0.8 中, WriteSignal 有 .set() 方法, ReadSignal 没有。
//
// </details>
