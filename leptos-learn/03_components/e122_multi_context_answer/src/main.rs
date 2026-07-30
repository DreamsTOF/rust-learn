// ============================================================
// Exercise 122 - Answer: Multi Context
// ============================================================

use leptos::prelude::*;

#[derive(Clone)]
struct UserName(String);

#[derive(Clone)]
struct UserAge(u32);

#[component]
fn UserProfile() -> impl IntoView {
    let name = use_context::<UserName>()
        .expect("UserName should be provided");
    let age = use_context::<UserAge>()
        .expect("UserAge should be provided");

    view! {
        <div style="border: 1px solid green; padding: 8px; margin: 8px 0;">
            <h3>"User Profile"</h3>
            <p>"Name: " {name.0.clone()}</p>
            <p>"Age: " {age.0}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    provide_context(UserName("Alice".to_string()));
    provide_context(UserAge(30u32));

    view! {
        <div style="border: 1px solid gray; padding: 8px;">
            <h2>"Multi Context Demo"</h2>
            <p>"同时提供 UserName(String) 和 UserAge(u32) 两种 Context"</p>
            <UserProfile/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
