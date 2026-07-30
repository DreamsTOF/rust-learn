// ============================================================
// 练习 40: memo_basic
//
// 目标: 使用 Memo::new() 创建缓存派生值并显示
//
// 难度: ⭐⭐
// 核心知识点: Memo 基础、缓存派生
//
// TODO: 按照注释提示补全代码
// ============================================================
use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建信号 `count`，i32 初始值为 0

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 使用 Memo::new() 创建派生值 `double`
    //   提示: `let double = Memo::new(move |_| count() * 2);`

    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 在视图中显示 count 和 double（缓存双倍值）
    //   添加按钮更新 count，观察 double 自动更新

    view! {
        <div>
            <p>"练习 40: memo_basic"</p>
            // TODO: 显示 count 的值
            // TODO: 显示 double (memo) 的值
            // TODO: 按钮 "count += 1"
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
//     let (count, set_count) = signal(0);
//
//     // Memo::new 创建缓存派生值，仅在依赖变化时重算
//     let double = Memo::new(move |_| count() * 2);
//
//     view! {
//         <div>
//             <p>"练习 40: memo_basic"</p>
//             <p>"count = " {count}</p>
//             <p>"double (memo) = " {double}</p>
//             <button on:click=move |_| set_count.update(|v| *v += 1)>"count += 1"</button>
//         </div>
//     }
// }
// ```
//
// ### 知识点
// - `Memo::new(move |_| expr)` 创建一个 Memo 信号，内部缓存 expr 的结果
// - Memo 只在其依赖的信号发生变化时才重新计算 expr
// - 在 view 中可以直接使用 Memo 对象，Leptos 自动读取其缓存值
// - 适合对昂贵计算进行缓存优化（如过滤、排序、复杂转换）
//
// </details>
