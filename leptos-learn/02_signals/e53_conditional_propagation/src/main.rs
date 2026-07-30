// ============================================================
// 练习 53: conditional_propagation
//
// 目标: 演示派生信号的条件传播机制——只有依赖变化时才通知下游消费者
//
// 难度: ⭐⭐⭐
// 核心知识点: 派生信号的条件传播
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建三个信号: `x` (i32, 初始 10), `threshold` (i32, 初始 15), `label` (String, 初始 "A")
    // signal! 宏或 RwSignal::new() 均可

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 派生一个布尔信号 `is_above`: `move || x() > threshold()`
    // 注意: 当 threshold 不变而 x 变化时，只有当 x 跨越阈值时 is_above 才变化
    // 这意味着下游消费者不会在 x 的每次变化时都更新——条件避免了不必要传播

    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 派生一个字符串信号 `status`: 根据 is_above 返回 "above" 或 "below"
    // 这个信号依赖于 is_above（间接依赖于 x 和 threshold）

    // === 步骤 4 ——————————————————————————————————————————
    // TODO: 添加三个按钮分别更新 x, threshold, label
    // 渲染 x, threshold, is_above, status 的值
    // 观察: 当 x 从 11→12→13（未跨越 15）时，is_above 不变→下游不更新
    //       当 x 从 14→16（跨越 15）时，is_above 变化→下游更新

    view! {
        <div>
            <p>"练习 53: conditional_propagation"</p>
            // TODO: 渲染各信号的值
            // TODO: x += 1 按钮
            // TODO: threshold += 1 按钮
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
// #[component]
// fn Exercise() -> impl IntoView {
//     let x = RwSignal::new(10);
//     let threshold = RwSignal::new(15);
//     let _label = RwSignal::new("A".to_string());
//
//     let is_above = move || x() > threshold();
//     let status = move || if is_above() { "above" } else { "below" };
//
//     view! {
//         <div>
//             <p>"练习 53: conditional_propagation"</p>
//             <p>"x = " {x}", threshold = " {threshold}</p>
//             <p>"is_above = " {is_above}</p>
//             <p>"status = " {status}</p>
//             <button on:click=move |_| x.set(x.get() + 1)>"x += 1"</button>
//             <button on:click=move |_| threshold.set(threshold.get() + 1)>"threshold += 1"</button>
//         </div>
//     }
// }
// ```
//
// ### 知识点
// - Leptos 的派生信号在依赖变化时不会自动传播——条件是依赖值本身变化了才触发
// - 本例中 `is_above` 依赖 x 和 threshold，但只有当 `x() > threshold()` 的
//   布尔结果发生变化时，`is_above` 的下游（如 `status`）才会被通知
// - 如果 x 从 10 变为 11（仍在 threshold 之下），`is_above` 值不变，
//   不会造成下游重新计算——这就是条件传播的精髓
// - 这种机制避免了"每个中间状态都传播"的性能浪费，是响应式系统的关键优化
//
// </details>
