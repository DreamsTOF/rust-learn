// ============================================================
// 练习 30: signal_write_guard — .write() guard 更新
//
// 目标: 用 *count.write() = 42 修改值
//
// 难度: ⭐⭐
// 核心知识点:
//   - .write() 返回 WriteGuard，可以像 &mut T 一样使用
//   - 解引用赋值直接修改信号值
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 使用 signal(0) 创建信号
    let (count, set_count) = signal(0);

    view! {
        <div>
            <h1>"练习 30: signal.write() guard"</h1>
            <p>"当前值: " {count}</p>
            <button on:click=move |_| *set_count.write() = 42>
                "*count.write() = 42"
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
//     <button on:click=move |_| *set_count.write() = 42>
//         "*count.write() = 42"
//     </button>
// }
// ```
//
// ### 知识点
// - `.write()` 返回 `WriteGuard<T>`，实现了 `DerefMut<Target=T>`。
// - 通过 `*count.write() = 42` 直接赋值，就像是操作 `&mut i32`。
// - 守卫（WriteGuard）在语句结束时自动 drop，触发通知。
// - 适合需要多次修改或条件修改的场景，
//   比 `.update()` 更灵活（但要注意守卫生命周期）。
//
// </details>
