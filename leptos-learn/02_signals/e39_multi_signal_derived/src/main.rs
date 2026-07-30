// ============================================================
// 练习 39: multi_signal_derived
//
// 目标: 使用两个信号源，通过闭包派生计算和
//
// 难度: ⭐
// 核心知识点: 多信号派生
//
// TODO: 按照注释提示补全代码
// ============================================================
use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建两个信号 `a` 和 `b`，类型 i32，初始值分别为 10 和 5

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 使用闭包 `move || a() + b()` 派生和

    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 显示 a、b、sum 的值，添加按钮分别更新 a 和 b

    view! {
        <div>
            <p>"练习 39: multi_signal_derived"</p>
            // TODO: 显示 a 的值
            // TODO: 显示 b 的值
            // TODO: 显示 a + b 的和
            // TODO: 按钮 "a += 1"
            // TODO: 按钮 "b += 1"
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
//     let (a, set_a) = signal(10);
//     let (b, set_b) = signal(5);
//
//     // 派生出 a + b
//     let sum = move || a() + b();
//
//     view! {
//         <div>
//             <p>"练习 39: multi_signal_derived"</p>
//             <p>"a = " {a}</p>
//             <p>"b = " {b}</p>
//             <p>"a + b = " {sum}</p>
//             <button on:click=move |_| set_a.update(|v| *v += 1)>"a += 1"</button>
//             <button on:click=move |_| set_b.update(|v| *v += 1)>"b += 1"</button>
//         </div>
//     }
// }
// ```
//
// ### 知识点
// - 可以从多个信号派生新值，Leptos 会自动追踪所有依赖
// - 派生闭包 `move || a() + b()` 在每次访问时重新求值
// - 调用 `set_a.update()` 或 `set_b.update()` 会触发界面更新
//
// </details>
