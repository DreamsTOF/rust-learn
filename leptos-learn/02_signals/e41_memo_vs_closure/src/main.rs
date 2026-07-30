// ============================================================
// 练习 41: memo_vs_closure
//
// 目标: 对比普通闭包派生与 Memo 缓存的差异——Memo 只在其
//       依赖变化时重算，而非每次重新求值
//
// 难度: ⭐⭐
// 核心知识点: Memo 只在其依赖变化时重算
//
// TODO: 按照注释提示补全代码
// ============================================================
use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建信号 `x`，i32 初始值为 1

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 创建闭包派生 `x_triple_closure = move || x() * 3`
    //   创建 Memo 派生 `x_triple_memo = Memo::new(move |_| x() * 3)`

    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 在视图中显示两种派生的值
    //   添加按钮更新 x，观察两者结果相同但 Memo 有缓存优化

    view! {
        <div>
            <p>"练习 41: memo_vs_closure"</p>
            // TODO: 显示 x 的值
            // TODO: 显示闭包派生 x_triple_closure 的值
            // TODO: 显示 Memo 派生 x_triple_memo 的值（带 "(memo)" 标识）
            // TODO: 按钮 "x += 1"
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
//     let (x, set_x) = signal(1);
//
//     // 闭包派生：每次访问时重新计算
//     let x_triple_closure = move || x() * 3;
//
//     // Memo 派生：缓存结果，仅在 x 变化时重算
//     let x_triple_memo = Memo::new(move |_| x() * 3);
//
//     view! {
//         <div>
//             <p>"练习 41: memo_vs_closure"</p>
//             <p>"x = " {x}</p>
//             <p>"闭包 x*3 = " {x_triple_closure}</p>
//             <p>"Memo x*3 = " {x_triple_memo}</p>
//             <button on:click=move |_| set_x.update(|v| *v += 1)>"x += 1"</button>
//         </div>
//     }
// }
// ```
//
// ### 知识点
// - **闭包派生**: 每次在视图中渲染时重新求值，无额外缓存开销
// - **Memo**: 内部缓存计算结果，只有当依赖信号变化时才重新计算
// - 对于简单算术两者结果相同，但 Memo 在昂贵计算场景有优势
// - 选型: 廉价计算用闭包派生，昂贵计算用 Memo
//
// </details>
