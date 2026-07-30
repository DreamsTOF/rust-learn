// ============================================================
// 练习 e108: Props 解构 (Props Destructure)
//
// 核心知识点:
//   - 在组件函数签名中使用 Rust 模式匹配解构 Props 结构体
//   - 自定义 Props 结构体用法
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义一个 Props 结构体，包含 name 和 age 字段
#[derive(Clone)]
struct PersonProps {
    name: String,
    age: u32,
}

// TODO: 在函数参数中直接解构 PersonProps
// 提示: fn 函数名(StructName { field1, field2 }: StructName) -> impl IntoView
fn PersonView(PersonProps { name, age }: PersonProps) -> impl IntoView {
    view! {
        <p>"姓名：" {name} "，年龄：" {age}</p>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h3>"练习 108: props_destructure"</h3>
        // 使用 Rust 表达式将结构体传入
        {PersonView(PersonProps { name: "Alice".to_string(), age: 30 })}
        {PersonView(PersonProps { name: "Bob".to_string(), age: 25 })}
    }
}

fn main() {
    mount_to_body(App);
}

// <details>
// 参考答案:
//
// use leptos::prelude::*;
//
// #[derive(Clone)]
// struct PersonProps {
//     name: String,
//     age: u32,
// }
//
// fn PersonView(PersonProps { name, age }: PersonProps) -> impl IntoView {
//     view! {
//         <p>"姓名：" {name} "，年龄：" {age}</p>
//     }
// }
//
// #[component]
// fn App() -> impl IntoView {
//     view! {
//         <h3>"练习 108: props_destructure"</h3>
//         {PersonView(PersonProps { name: "Alice".to_string(), age: 30 })}
//         {PersonView(PersonProps { name: "Bob".to_string(), age: 25 })}
//     }
// }
//
// fn main() {
//     mount_to_body(App);
// }
// </details>
