// ============================================================
// 练习 e99: prop_into — 自动类型转换
//
// 核心知识点:
//   - #[prop(into)] 属性自动对传入值调用 .into()
//   - 调用方可传入 &str 而 String 参数自动转换
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 Greet 组件，使用 #[prop(into)] 让 name: String
// 接受 &str 等可转换为 String 的类型
#[component]
fn Greet(#[prop(into)] name: String) -> impl IntoView {
    view! {
        <p>"Hello, " {name} "!"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            // TODO: 直接传入 &str 字符串字面量（无需 .to_string()）
            <Greet name="World" />
            // TODO: 也可以传入 String
            <Greet name=String::from("Leptos") />
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
// fn Greet(#[prop(into)] name: String) -> impl IntoView {
//     view! {
//         <p>"Hello, " {name} "!"</p>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <Greet name="World" />
//             <Greet name=String::from("Leptos") />
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
// - #[prop(into)] 在编译时生成 .into() 调用
// - &str -> String、i32 -> f64 等标准 Into 转换自动生效
// - 调用方可以传更灵活的类型，不必手动转换
//
// </details>
