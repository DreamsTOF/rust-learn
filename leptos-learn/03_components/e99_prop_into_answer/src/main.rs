use leptos::prelude::*;

#[component]
fn Greet(#[prop(into)] name: String) -> impl IntoView {
    view! {
        <p>"Hello, " {name} "!"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <Greet name="World" />
            <Greet name=String::from("Leptos") />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
