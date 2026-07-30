// ============================================================
// 练习 e98: 默认值 Props (default_props)
//
// 核心知识点:
//   - #[prop(default = ...)] 为 Props 提供默认值
//   - 不传时自动使用默认值，而非必需或 Option
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 Counter 组件，count prop 默认值为 0
// 使用 #[prop(default = 0)] 属性
#[component]
fn Counter(#[prop(default = 0)] count: i32) -> impl IntoView {
    view! {
        <p>"Count: " {count}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            // TODO: 传入 count=42
            <Counter count=42 />
            // TODO: 不传 count，测试默认值 0
            <Counter />
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
// use leptos::prelude::*;
//
// #[component]
// fn Counter(#[prop(default = 0)] count: i32) -> impl IntoView {
//     view! {
//         <p>"Count: " {count}</p>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <Counter count=42 />
//             <Counter />
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - #[prop(default = 值)] 让 Prop 可选且提供默认值
// - 优于 Option<T>：调用方不用包装 Some，组件内不需 unwrap
// - 默认值可以是字面量、简单表达式
//
// </details>
