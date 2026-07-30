use leptos::prelude::*;

#[component]
fn Counter(#[prop(default = 0)] count: i32) -> impl IntoView {
    view! {
        <p>"Count: " {count}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <Counter count=42 />
            <Counter />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
