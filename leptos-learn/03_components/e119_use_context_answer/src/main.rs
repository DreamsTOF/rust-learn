// ============================================================
// Exercise 119 — Answer: use_context
// ============================================================

use leptos::prelude::*;

#[component]
fn UserGreeting() -> impl IntoView {
    let username = use_context::<String>()
        .expect("String context should be provided by parent");

    view! {
        <div style="border:1px solid #4caf50; padding:8px; margin:8px 0; border-radius:4px;">
            <p>"👋 欢迎, " {username} "!"</p>
        </div>
    }
}

#[component]
fn StatusBar() -> impl IntoView {
    let username = use_context::<String>()
        .expect("String context should be provided by parent");

    view! {
        <div style="background:#f5f5f5; padding:4px 8px; border-radius:4px; font-size:0.85rem;">
            "已登录用户: " {username}
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    provide_context("Alice".to_string());

    view! {
        <div style="padding:8px; border:1px solid #ccc; border-radius:4px;">
            <h3>"use_context 示例"</h3>
            <p>"父组件通过 provide_context 注入了用户信息"</p>
            <UserGreeting/>
            <StatusBar/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
