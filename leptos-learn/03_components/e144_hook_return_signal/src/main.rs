// ============================================================
// 练习 e144: hook_return_signal — Hook 返回信号
//
// 核心知识点:
//   - Hook 返回 (count, set_count, increment) 三元素
//   - increment 是一个闭包，封装了 count() + 1 逻辑
//   - 返回 impl Fn() + 'static 类型
//
// 难度: ⭐⭐ (闭包作为返回值)
// ============================================================

use leptos::prelude::*;

// TODO: 定义一个 use_counter hook，返回 (ReadSignal<i32>, WriteSignal<i32>, impl Fn())
// 第三个元素 increment 是一个闭包，调用时执行 set_count(count() + 1)
fn use_counter() -> (ReadSignal<i32>, WriteSignal<i32>, impl Fn()) {
    let (count, set_count) = signal(0);
    let increment = move || {
        // TODO: 实现自增逻辑
        set_count.set(count() + 1);
    };
    (count, set_count, increment)
}

fn main() {
    mount_to_body(move || {
        // TODO: 解构 use_counter 返回的三个值
        let (count, _set_count, increment) = use_counter();

        view! {
            <div>
                <p>"练习 144: Hook 返回信号 + increment"</p>
                <p style="font-size: 24px; font-weight: bold;">
                    "计数: " {count}
                </p>
                <button on:click=move |_| {
                    // TODO: 调用 increment 闭包
                    increment();
                }>"+1 (使用 increment)"</button>
            </div>
        }
    });
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// use leptos::prelude::*;
//
// fn use_counter() -> (ReadSignal<i32>, WriteSignal<i32>, impl Fn()) {
//     let (count, set_count) = signal(0);
//     let increment = move || { set_count.set(count() + 1); };
//     (count, set_count, increment)
// }
//
// fn main() {
//     mount_to_body(move || {
//         let (count, _set_count, increment) = use_counter();
//
//         view! {
//             <div>
//                 <p>"练习 144: Hook 返回信号 + increment"</p>
//                 <p style="font-size: 24px; font-weight: bold;">"计数: " {count}</p>
//                 <button on:click=move |_| { increment(); }>"+1 (使用 increment)"</button>
//             </div>
//         }
//     });
// }
// ```
//
// ### 知识点
// 1. Hook 可以返回闭包作为便利方法，封装常见操作
// 2. `impl Fn()` 是返回闭包的最宽泛 trait（可多次调用，无参数无返回值）
// 3. 闭包中通过 move 捕获信号，确保闭包拥有所有权
// 4. 调用方可以选择忽略 set_count（用 _ 前缀），统一通过 increment 修改
// </details>
