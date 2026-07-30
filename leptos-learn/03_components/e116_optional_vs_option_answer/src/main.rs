// ============================================================
// Exercise 116 — Answer: #[prop(optional)] vs Option<T>
// ============================================================

use leptos::prelude::*;

#[component]
fn DualProps(
    #[prop(optional)]
    optional_msg: Option<&'static str>,
    explicit_msg: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div style="border:1px solid #999; padding:8px; margin:8px 0; border-radius:4px;">
            <p><strong>"optional_msg:"</strong> {optional_msg.unwrap_or("(未提供 — None)")}</p>
            <p><strong>"explicit_msg:"</strong> {explicit_msg.unwrap_or("(未提供 — None)")}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="padding:8px;">
            <h3>"#[prop(optional)] vs Option<T>"</h3>
            <DualProps explicit_msg=Some("必须用 Some 包裹") />
            <DualProps
                optional_msg="省略式传入"
                explicit_msg=Some("仍需 Some 包裹")
            />
            <DualProps explicit_msg=None />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
