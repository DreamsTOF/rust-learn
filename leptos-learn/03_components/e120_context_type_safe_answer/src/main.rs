// ============================================================
// Exercise 120 — Answer: Type-Safe Context
// ============================================================

use leptos::prelude::*;

#[derive(Clone)]
struct UserId(u32);

#[component]
fn UserDisplay() -> impl IntoView {
    let user_id = use_context::<UserId>();
    let wrong_type = use_context::<&'static str>();

    view! {
        <div style="border:1px solid #999; padding:8px; margin:8px 0; border-radius:4px;">
            <p><strong>"UserId context: "</strong>
                {match user_id {
                    Some(id) => format!("{}", id.0),
                    None => "(未找到)".to_string(),
                }}
            </p>
            <p><strong>"&'static str context: "</strong>
                {match wrong_type {
                    Some(s) => s,
                    None => "(未找到 — 类型不匹配)",
                }}
            </p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    provide_context(UserId(42));

    view! {
        <div style="padding:8px;">
            <h3>"类型安全的 Context"</h3>
            <p>"provide_context(UserId(42)) — 尝试用不同类型读取"</p>
            <UserDisplay/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
