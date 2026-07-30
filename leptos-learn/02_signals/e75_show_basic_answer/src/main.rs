use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <button on:click=move |_| set_visible.update(|v| *v = !*v)>
            "切换显示"
        </button>
        <Show when=move || visible.get()>
            <p>"现在你看到我了 👋"</p>
        </Show>
    }
}

fn main() {
    mount_to_body(Exercise);
}
