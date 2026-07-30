// ============================================================
// 练习 48: derived_with_closure
//
// 目标: 演示在派生闭包中使用 `.with()` 方法读取信号值
//
// 难度: ⭐⭐
// 核心知识点: 闭包中使用 .with()
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建一个 i32 信号 `count`，初始值为 42
    // 提示: 使用 `signal!` 宏或 ` RwSignal::new()`

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 使用 `.with()` 派生一个字符串闭包 `count_str`
    // 要求: `move || count.with(|n| n.to_string())`
    // .with() 提供对信号的临时引用访问，避免不必要的 Clone

    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 添加按钮，点击时将 count 增加 1
    // 渲染 count_str 的值，观察变化

    view! {
        <div>
            <p>"练习 48: derived_with_closure"</p>
            // TODO: 渲染 count_str 的值
            // TODO: 添加"+"按钮更新 count
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
//     let count = RwSignal::new(42);
//     let count_str = move || count.with(|n| n.to_string());
//
//     view! {
//         <div>
//             <p>"练习 48: derived_with_closure"</p>
//             <p>"count_str = " {count_str}</p>
//             <button on:click=move |_| count.set(count.get() + 1)>"+"</button>
//         </div>
//     }
// }
// ```
//
// ### 知识点
// - `.with()` 接受一个闭包，对信号值进行临时借用访问，无需 Clone
// - 派生闭包 `move || count.with(...)` 捕获 `count` 信号本身（而非其值）
// - 每次读取 `count_str` 时都会重新调用 `.with()`，确保始终拿到最新值
// - 适合在闭包内做格式化/转换，避免不必要的分配
//
// </details>
