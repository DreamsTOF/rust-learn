// ============================================================
// Exercise 110 - Answer: generic_component
// ============================================================

use leptos::prelude::*;

#[component]
fn List<T: 'static + std::fmt::Display>(
    items: Vec<T>,
) -> impl IntoView {
    view! {
        <ul>
            {items.into_iter().map(|item| view! { <li>{item.to_string()}</li> }).collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h3>"练习 110: generic_component"</h3>
        <h4>"数字列表"</h4>
        <List items=vec![10, 20, 30, 40, 50] />
        <h4>"字符串列表"</h4>
        <List items=vec!["苹果", "香蕉", "樱桃"] />
    }
}

fn main() {
    mount_to_body(App);
}
