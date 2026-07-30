use leptos::prelude::*;

#[component]
fn Button(on_click: impl Fn() + 'static) -> impl IntoView {
    view! {
        <button on:click=move |_| on_click()>"Click me"</button>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <Button on_click=|| leptos::logging::log!("Button clicked!") />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
