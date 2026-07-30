// ============================================================
// 练习 49: signal_array_derived
//
// 目标: 从单个基础信号通过 map 派生出一组信号（信号数组）
//
// 难度: ⭐⭐⭐
// 核心知识点: 信号数组派生
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建一个 i32 信号 `base`，初始值为 5
    // 提示: 使用 `RwSignal::new(5)`

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 使用 `(0..10).map(|i| move || base() + i).collect()` 派生一个 Vec 信号数组
    // 这个数组包含 10 个派生闭包（信号），每个都在 base 值上加一个偏移量
    // 提示: 用 `.collect::<Vec<_>>()` 收集

    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 添加按钮更新 base 值
    // 使用 for 循环或 .iter().map() 渲染所有派生值

    view! {
        <div>
            <p>"练习 49: signal_array_derived"</p>
            // TODO: 渲染派生信号数组的每个值
            // TODO: 添加按钮更新 base
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
//     let base = RwSignal::new(5);
//     let derived: Vec<_> = (0..10).map(|i| move || base() + i).collect();
//
//     view! {
//         <div>
//             <p>"练习 49: signal_array_derived"</p>
//             <p>"base = " {base}</p>
//             <ul>
//             {derived.iter().enumerate().map(|(idx, val)| {
//                 view! { <li>"[" {idx}"] = " {val}</li> }
//             }).collect::<Vec<_>>()}
//             </ul>
//             <button on:click=move |_| base.set(base.get() + 1)>"base += 1"</button>
//         </div>
//     }
// }
// ```
//
// ### 知识点
// - `(0..10).map(|i| move || base() + i)` 创建 10 个独立的派生闭包
// - 每个闭包都有自己的偏移量 `i`，通过 move 捕获
// - `collect::<Vec<_>>()` 将它们收集为 Vec<impl Fn() -> i32 + '_>
// - 更新 base 时，所有 10 个派生值都会自动更新
// - 该模式适合生成列表/表格等需要基于同一数据源的不同偏移量的场景
//
// </details>
