// ============================================================
// Exercise 108 - Answer: props_destructure
// ============================================================

use leptos::prelude::*;

#[derive(Clone)]
struct PersonProps {
    name: String,
    age: u32,
}

fn PersonView(PersonProps { name, age }: PersonProps) -> impl IntoView {
    view! {
        <p>"姓名：" {name} "，年龄：" {age}</p>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h3>"练习 108: props_destructure"</h3>
        {PersonView(PersonProps { name: "Alice".to_string(), age: 30 })}
        {PersonView(PersonProps { name: "Bob".to_string(), age: 25 })}
    }
}

fn main() {
    mount_to_body(App);
}
