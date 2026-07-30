// ============================================================
// 练习 51: derived_as_prop
//
// 目标: 演示将派生信号作为组件属性 (prop) 传递给子组件
//
// 难度: ⭐⭐
// 核心知识点: 派生信号作 prop
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 定义一个子组件 `DisplayValue`，接收一个 i32 类型的 prop `value`
// 提示: 使用 #[component] 宏

// === 步骤 2 ——————————————————————————————————————————
// TODO: 在父组件 Exercise 中：
//   1. 创建 i32 信号 `count`，初始值为 1
//   2. 使用 `move || count() * 2` 派生值，作为 prop 传给 DisplayValue
//   3. 添加按钮更新 count

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <p>"练习 51: derived_as_prop"</p>
            // TODO: 渲染 DisplayValue 组件，传入派生信号作为 value prop
            // TODO: 添加按钮更新 count
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
// fn DisplayValue(value: i32) -> impl IntoView {
//     view! { <p>"value = " {value}</p> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let count = RwSignal::new(1);
//
//     view! {
//         <div>
//             <p>"练习 51: derived_as_prop"</p>
//             <p>"count = " {count}</p>
//             <DisplayValue value=move || count() * 2 />
//             <button on:click=move |_| count.set(count.get() + 1)>"count += 1"</button>
//         </div>
//     }
// }
// ```
//
// ### 知识点
// - 派生闭包 `move || count() * 2` 可以像普通值一样作为 prop 传递
// - 组件接收 i32 类型，Leptos 自动将闭包求值结果传入
// - 当 count 变化时, DisplayValue 自动重新渲染
// - 这是 Leptos 中传递响应式派生值的标准方式
//
// </details>
