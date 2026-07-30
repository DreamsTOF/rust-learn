// ============================================================
// 练习 52: signal_vs_create_memo
//
// 目标: 比较闭包直接派生与 create_memo 的性能权衡
//
// 难度: ⭐⭐⭐
// 核心知识点: 性能权衡 Signal 派生 vs create_memo
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建两个信号 `a` 和 `b`，类型 i32，初始值分别为 3 和 5

    // === 步骤 2 ——————————————————————————————————————————
    // TODO:
    //   方式 A — 闭包派生: `move || a() + b()` (每次访问时重新求值)
    //   方式 B — create_memo: `Memoized` 缓存计算结果，仅依赖变化时重算
    //   提示: `let sum_memo = Memo::new(move |_| a() + b());`

    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 渲染两种方式的结果，添加按钮更新 a 和 b
    // 观察两种方式在效果上是一致的，但 create_memo 有缓存优化

    view! {
        <div>
            <p>"练习 52: signal_vs_create_memo"</p>
            // TODO: 显示闭包派生值
            // TODO: 显示 create_memo 值
            // TODO: 按钮更新 a 和 b
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
//     let a = RwSignal::new(3);
//     let b = RwSignal::new(5);
//
//     // 方式 A: 闭包派生 —— 每次读取时重新计算
//     let sum_closure = move || a() + b();
//
//     // 方式 B: create_memo —— 缓存结果，仅依赖变化时重算
//     let sum_memo = Memo::new(move |_| a() + b());
//
//     view! {
//         <div>
//             <p>"练习 52: signal_vs_create_memo"</p>
//             <p>"闭包派生 sum = " {sum_closure}</p>
//             <p>"create_memo sum = " {sum_memo}</p>
//             <button on:click=move |_| a.set(a.get() + 1)>"a += 1"</button>
//             <button on:click=move |_| b.set(b.get() + 1)>"b += 1"</button>
//         </div>
//     }
// }
// ```
//
// ### 知识点
// - **闭包派生**: 轻量，每次访问时重新求值。适合廉价计算，零开销但无缓存
// - **create_memo**: 内部缓存计算结果，只有当依赖信号变化时才重新计算
// - create_memo 适合昂贵计算（如大量数据过滤、排序），避免重复计算
// - 闭包派生适合简单转换（如加法、格式化），开销更低（无缓存层）
// - 选型原则: 计算廉价 → 用闭包派生；计算昂贵 → 用 create_memo
//
// </details>
