use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <h1>"练习 27: signal.set()"</h1>
            <p>"当前值: " {count}</p>
            <button on:click=move |_| set_count.set(42)>
                "设值为 42"
            </button>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
