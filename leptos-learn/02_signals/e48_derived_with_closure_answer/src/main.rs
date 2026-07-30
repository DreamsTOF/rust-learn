use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let count = RwSignal::new(42);
    let count_str = move || count.with(|n| n.to_string());

    view! {
        <div>
            <p>"练习 48: derived_with_closure"</p>
            <p>"count_str = " {count_str}</p>
            <button on:click=move |_| count.set(count.get() + 1)>"+"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
