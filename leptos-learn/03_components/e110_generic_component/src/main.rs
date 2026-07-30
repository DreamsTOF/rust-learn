// ============================================================
// 练习 e110: 泛型组件 (Generic Component)
//
// 核心知识点:
//   - 在组件中使用泛型类型参数
//   - T: 'static 生命周期约束（Leptos 要求所有 prop 类型为 'static）
//   - 将泛型数据渲染为视图
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// 泛型列表组件：可接收任意实现了 Display 的类型
#[component]
fn List<T: 'static + std::fmt::Display>(
    /// 要渲染的列表项
    items: Vec<T>,
) -> impl IntoView {
    view! {
        <ul>
            // TODO: 遍历 items 并将每一项渲染为 <li>
            {items.into_iter().map(|item| view! { <li>{item.to_string()}</li> }).collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h3>"练习 110: generic_component"</h3>
        <h4>"数字列表"</h4>
        // TODO: 使用 List 组件显示数字
        <List items=vec![10, 20, 30, 40, 50] />

        <h4>"字符串列表"</h4>
        // TODO: 使用同一个 List 组件显示字符串
        <List items=vec!["苹果", "香蕉", "樱桃"] />
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
// #[component]
// fn List<T: 'static + std::fmt::Display>(
//     items: Vec<T>,
// ) -> impl IntoView {
//     view! {
//         <ul>
//             {items.into_iter().map(|item| view! { <li>{item.to_string()}</li> }).collect::<Vec<_>>()}
//         </ul>
//     }
// }
//
// #[component]
// fn App() -> impl IntoView {
//     view! {
//         <h3>"练习 110: generic_component"</h3>
//         <h4>"数字列表"</h4>
//         <List items=vec![10, 20, 30, 40, 50] />
//         <h4>"字符串列表"</h4>
//         <List items=vec!["苹果", "香蕉", "樱桃"] />
//     }
// }
//
// fn main() {
//     mount_to_body(App);
// }
// </details>
