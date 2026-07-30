use leptos::prelude::*;

#[derive(Clone, Default)]
struct AppConfig {
    title: String,
    version: u32,
}

#[component]
fn Child() -> impl IntoView {
    let config = use_context::<AppConfig>().unwrap_or_default();

    view! {
        <p>"Title: " {config.title}</p>
        <p>"Version: v" {config.version}</p>
    }
}

#[component]
fn WithConfig() -> impl IntoView {
    provide_context(AppConfig {
        title: String::from("Leptos App"),
        version: 1,
    });
    view! { <Child/> }
}

#[component]
fn WithoutConfig() -> impl IntoView {
    view! { <Child/> }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <h2>"Context Default Fallback"</h2>
        <div>
            <h3>"With Context"</h3>
            <WithConfig/>
        </div>
        <hr/>
        <div>
            <h3>"Without Context (defaults)"</h3>
            <WithoutConfig/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
