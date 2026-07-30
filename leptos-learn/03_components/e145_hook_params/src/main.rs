// ============================================================
// 练习 e145: hook_params — 参数化 Hook
//
// 核心知识点:
//   - Hook 接收参数: use_counter(start: i32)
//   - 灵活的初始值设定
//   - 与 e144 相同返回类型 (count, set_count, increment)
//
// 难度: ⭐⭐ (参数传递)
// ============================================================

use leptos::prelude::*;

// TODO: 定义一个参数化的 use_counter hook
// 接收 start: i32 作为初始值
// 返回 (ReadSignal<i32>, WriteSignal<i32>, impl Fn())
fn use_counter(start: i32) -> (ReadSignal<i32>, WriteSignal<i32>, impl Fn()) {
    // TODO: 使用 start 作为初始值创建信号
    let (count, set_count) = signal(start);
    let increment = move || {
        set_count.set(count() + 1);
    };
    (count, set_count, increment)
}

fn main() {
    mount_to_body(move || {
        // TODO: 调用 use_counter(10)，初始值为 10
        let (count, _set_count, increment) = use_counter(10);

        view! {
            <div>
                <p>"练习 145: 参数化 Hook — use_counter(10)"</p>
                <p style="font-size: 24px; font-weight: bold;">
                    "计数: " {count}
                </p>
                <button on:click=move |_| {
                    increment();
                }>"+1"</button>
                <p style="color: #888; font-size: 14px;">
                    "初始值为 10，点击按钮递增"
                </p>
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
// fn use_counter(start: i32) -> (ReadSignal<i32>, WriteSignal<i32>, impl Fn()) {
//     let (count, set_count) = signal(start);
//     let increment = move || { set_count.set(count() + 1); };
//     (count, set_count, increment)
// }
//
// fn main() {
//     mount_to_body(move || {
//         let (count, _set_count, increment) = use_counter(10);
//
//         view! {
//             <div>
//                 <p>"练习 145: 参数化 Hook — use_counter(10)"</p>
//                 <p style="font-size: 24px; font-weight: bold;">"计数: " {count}</p>
//                 <button on:click=move |_| { increment(); }>"+1"</button>
//                 <p style="color: #888; font-size: 14px;">"初始值为 10，点击按钮递增"</p>
//             </div>
//         }
//     });
// }
// ```
//
// ### 知识点
// 1. Hook 可以像普通函数一样接收参数
// 2. 参数化让 Hook 更灵活——指定初始值、步长等配置
// 3. `signal(start)` 接受任意实现了 `Into<SignalGet>` 的值
// 4. 这是组合更复杂 Hook（如 use_counter_with_step）的基础
// </details>
