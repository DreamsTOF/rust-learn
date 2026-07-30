use leptos::prelude::*;

#[component]
fn Greet(#[prop(optional)] name: Option<String>) -> impl IntoView {
    let name = name.unwrap_or_else(|| "World".to_string());
    view! {
        <p>"Hello, " {name} "!"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <Greet name={"Leptos".to_string()} />
            <Greet />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
