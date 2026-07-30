// ============================================================
// Exercise 124 - Answer: Context in Router Scenario
// ============================================================

use leptos::prelude::*;

#[derive(Clone)]
struct User {
    name: String,
    role: String,
}

#[component]
fn DashboardPage() -> impl IntoView {
    let user = use_context::<User>()
        .expect("User should be provided by Layout");

    view! {
        <div style="border: 1px solid green; padding: 8px; margin: 8px 0;">
            <h3>"Dashboard Page"</h3>
            <p>"Welcome, " {user.name.clone()}</p>
            <p>"Role: " {user.role.clone()}</p>
        </div>
    }
}

#[component]
fn AppLayout() -> impl IntoView {
    provide_context(User {
        name: "Alice".to_string(),
        role: "Admin".to_string(),
    });

    view! {
        <div style="border: 1px solid gray; padding: 8px;">
            <h2>"App Layout"</h2>
            <p>"Layout 提供 User Context"</p>
            <DashboardPage/>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"Context Router Demo"</h1>
            <p>"Layout 提供 Context，Page 消费 — 模拟路由场景"</p>
            <AppLayout/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
