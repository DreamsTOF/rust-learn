use leptos::prelude::*;

#[component]
fn Child() -> impl IntoView {
    let name = use_context::<String>().unwrap_or_default();
    let score = use_context::<u32>().unwrap_or(0);
    let ratio = use_context::<f64>().unwrap_or(0.0);

    view! {
        <p>"Name: " {name}</p>
        <p>"Score: " {score}</p>
        <p>"Ratio: " {ratio}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    provide_context(String::from("Leptos"));
    provide_context(95u32);
    provide_context(3.14f64);

    view! {
        <h2>"Context Type Erasure (AnyMap)"</h2>
        <p><em>"Different types distinguished by TypeId"</em></p>
        <Child/>
    }
}

fn main() {
    mount_to_body(Exercise);
}
