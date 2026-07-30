// ============================================================
// 练习 e97: 可选 Props (optional_props)
//
// 核心知识点:
//   - #[prop(optional)] 配合 Option<T> 实现可选 Props
//   - 在组件内部用 unwrap_or / unwrap_or_else 提供回退值
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 Greet 组件，name 为 Option<String> 可选 prop
// 使用 #[prop(optional)] 使其可以省略
// 当 name 为 None 时默认显示 "World"
#[component]
fn Greet(#[prop(optional)] name: Option<String>) -> impl IntoView {
    // TODO: 使用 unwrap_or_else 处理 Option，默认 "World"
    let name = name.unwrap_or_else(|| "World".to_string());
    view! {
        <p>"Hello, " {name} "!"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            // TODO: 传入 name
            <Greet name={"Leptos".to_string()} />
            // TODO: 不传 name prop，测试默认值
            <Greet />
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
// fn Greet(#[prop(optional)] name: Option<String>) -> impl IntoView {
//     let name = name.unwrap_or_else(|| "World".to_string());
//     view! {
//         <p>"Hello, " {name} "!"</p>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <Greet name={"Leptos".to_string()} />
//             <Greet />
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
// - #[prop(optional)] 使 Prop 可省略，默认值为 None
// - 配合 Option<T> 使用，组件内部用 unwrap_or/unwrap_or_else 提供回退值
// - 有 #[prop(optional)] 时，调用方传 T 而非 Option<T>
//
// </details>
