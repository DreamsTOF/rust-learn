// ============================================================
// 练习 e143: hook_use_counter — 第一个自定义 Hook
//
// 核心知识点:
//   - 将信号创建逻辑提取为独立函数
//   - Hook 命名约定: use_ 前缀（借鉴 React Hooks 惯例）
//   - 返回值类型: (ReadSignal<i32>, WriteSignal<i32>)
//
// 难度: ⭐⭐ (模式识别 + 封装)
// ============================================================

use leptos::prelude::*;

// TODO: 定义一个名为 use_counter 的函数，封装 signal(0) 的创建
// 返回值: (ReadSignal<i32>, WriteSignal<i32>)
fn use_counter() -> (ReadSignal<i32>, WriteSignal<i32>) {
    // TODO: 创建并返回信号
    signal(0)
}

fn main() {
    mount_to_body(move || {
        // TODO: 调用 use_counter() 获取 count 和 set_count
        let (count, set_count) = use_counter();

        view! {
            <div>
                <p>"练习 143: 自定义 Hook — use_counter"</p>
                <p style="font-size: 24px; font-weight: bold;">
                    "计数: " {count}
                </p>
                <button on:click=move |_| {
                    set_count.set(count() + 1);
                }>"+1"</button>
                <button on:click=move |_| {
                    set_count.set(0);
                }>"重置"</button>
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
// fn use_counter() -> (ReadSignal<i32>, WriteSignal<i32>) {
//     signal(0)
// }
//
// fn main() {
//     mount_to_body(move || {
//         let (count, set_count) = use_counter();
//
//         view! {
//             <div>
//                 <p>"练习 143: 自定义 Hook — use_counter"</p>
//                 <p style="font-size: 24px; font-weight: bold;">"计数: " {count}</p>
//                 <button on:click=move |_| { set_count.set(count() + 1); }>"+1"</button>
//                 <button on:click=move |_| { set_count.set(0); }>"重置"</button>
//             </div>
//         }
//     });
// }
// ```
//
// ### 知识点
// 1. Hook 本质就是一个普通函数，封装了 leptos 响应式原语的创建
// 2. `use_` 前缀是 Rust Leptos 社区约定，表示该函数是一个 Hook
// 3. `signal(0)` 返回 (ReadSignal<i32>, WriteSignal<i32>) 元组
// 4. Hook 调用位置决定了其关联的响应式作用域
// </details>
