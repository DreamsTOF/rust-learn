// ============================================================
// 练习 50: reactive_eq
//
// 目标: 演示派生信号中使用 `==` 进行响应式相等性判断
//
// 难度: ⭐⭐⭐
// 核心知识点: 响应式 Eq 判断
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建两个 i32 信号 `a` 和 `b`，初始值分别为 10 和 10

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 派生一个信号 `is_equal`，使用 `move || a() == b()`
    // 注意: == 比较本身自动追踪 a 和 b 的依赖

    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 添加两个按钮分别更新 a 和 b
    // 渲染 a、b 和 is_equal 的值，观察响应式更新

    view! {
        <div>
            <p>"练习 50: reactive_eq"</p>
            // TODO: 显示 a, b, is_equal 的值
            // TODO: 添加按钮修改 a 和 b
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
//     let a = RwSignal::new(10);
//     let b = RwSignal::new(10);
//     let is_equal = move || a() == b();
//
//     view! {
//         <div>
//             <p>"练习 50: reactive_eq"</p>
//             <p>"a = " {a}", b = " {b}</p>
//             <p>"a == b ? " {is_equal}</p>
//             <button on:click=move |_| a.set(a.get() + 1)>"a += 1"</button>
//             <button on:click=move |_| b.set(b.get() + 1)>"b += 1"</button>
//         </div>
//     }
// }
// ```
//
// ### 知识点
// - `move || a() == b()` 追踪 a 和 b 两个信号的依赖
// - 任意一个信号变化时，is_equal 都会重新计算
// - 只有当 a 和 b 的值相等时，is_equal 返回 true
// - Leptos 的派生闭包自动追踪其中调用的所有信号
//
// </details>
