use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, _set_count) = signal(42);
    let value = count.get();

    view! {
        <div>
            <p>"count.get() = " {value}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
