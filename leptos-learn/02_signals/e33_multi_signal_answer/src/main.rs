use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (name, set_name) = signal("Leptos".to_string());

    let increment = move |_| set_count.update(|n| *n += 1);
    let toggle_name = move |_| {
        set_name.update(|n| {
            *n = if n == "Leptos" {
                "Rust".to_string()
            } else {
                "Leptos".to_string()
            };
        });
    };

    view! {
        <div>
            <p>"count: " {count}</p>
            <p>"name: " {name}</p>
            <button on:click=increment>"count +1"</button>
            <button on:click=toggle_name>"切换 name"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
