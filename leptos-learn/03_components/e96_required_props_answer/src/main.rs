use leptos::prelude::*;

#[component]
fn Greet(name: String) -> impl IntoView {
    view! {
        <p>"Hello, " {name} "!"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <Greet name={"World".to_string()} />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
