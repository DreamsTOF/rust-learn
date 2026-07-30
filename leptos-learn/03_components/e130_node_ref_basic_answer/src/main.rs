use leptos::prelude::*;
use leptos::html;

#[component]
fn Exercise() -> impl IntoView {
    let div_ref: NodeRef<html::Div> = NodeRef::new();
    let (text, set_text) = signal(String::new());

    view! {
        <h2>"NodeRef Basics"</h2>
        <div
            node_ref=div_ref
            style="padding: 1rem; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
            on:click=move |_| {
                if let Some(el) = div_ref.get() {
                    let _ = set_text.set(el.text_content().unwrap_or_default());
                }
            }
        >
            "Click this box"
        </div>
        <p>"Last clicked content: " {text}</p>
    }
}

fn main() {
    mount_to_body(Exercise);
}
