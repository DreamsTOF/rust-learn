// ============================================================
// Exercise 117 — Answer: Props Compile-Time Check
// ============================================================

use leptos::prelude::*;

#[component]
fn Greeting(
    name: &'static str,
    #[prop(optional)]
    title: Option<&'static str>,
) -> impl IntoView {
    let display = match title {
        Some(t) => format!("{t} {name}"),
        None => name.to_string(),
    };

    view! {
        <p style="font-size:1.2rem;">"Hello, " {display} "!"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="padding:8px;">
            <h3>"Props 编译期检查"</h3>
            <Greeting name="World" title="Dr." />
            <Greeting name="Rust" />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
