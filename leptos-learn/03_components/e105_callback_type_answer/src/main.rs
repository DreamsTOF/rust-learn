use leptos::prelude::*;

#[component]
fn ActionButton(action: Callback<()>) -> impl IntoView {
    view! {
        <button
            style="background:#3498db;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;"
            on:click=move |_| action.run(())
        >
            "执行操作"
        </button>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);
    let increment = Callback::new(move |_| {
        set_count.update(|n| *n += 1);
    });
    view! {
        <p>"次数: " {count}</p>
        <ActionButton action=increment />
    }
}

fn main() {
    mount_to_body(Exercise);
}
