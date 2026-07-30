use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (items, _set_items) = signal(vec!["HTML", "CSS", "JavaScript", "Rust"]);

    view! {
        <h3>"编程语言"</h3>
        <For each=move || items.get().into_iter().enumerate()
            key=|(i, _)| *i
            let:entry
        >
            <p style="margin: 4px 0;">
                {entry.0 + 1}". " {entry.1}
            </p>
        </For>
    }
}

fn main() {
    mount_to_body(Exercise);
}
