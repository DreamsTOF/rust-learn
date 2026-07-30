// ============================================================
// 练习 e96: 必需 Props (required_props)
//
// 核心知识点:
//   - #[component] 定义组件
//   - 函数参数即 Props，无默认值时是必需 Prop
//   - 传入 String 类型 Props 的方式
//
// 难度: ⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 Greet 组件，接收一个必需的 name: String prop
// 提示: 在参数中直接声明 name: String，然后在 view! 中使用它
#[component]
fn Greet(name: String) -> impl IntoView {
    view! {
        // TODO: 渲染 "Hello, {name}!"
        <p>"Hello, " {name} "!"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            // TODO: 使用 <Greet/> 组件并传入 name prop
            // 提示: 用 name={"World".to_string()} 语法
            <Greet name={"World".to_string()} />
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
// fn Greet(name: String) -> impl IntoView {
//     view! {
//         <p>"Hello, " {name} "!"</p>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <div>
//             <Greet name={"World".to_string()} />
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
// - #[component] 将 Rust 函数标记为 Leptos 组件
// - 函数参数直接映射为组件 Props，没有 #[prop(default)] 时即为必需 Prop
// - 传入 String 需用 {"...".to_string()} 或 String::from(...) 表达式语法
// - view! 宏中 {} 表示 Rust 表达式求值
//
// </details>
