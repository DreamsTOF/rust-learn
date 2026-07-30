use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, _set_count) = signal(0);

    view! {
        <div>
            <p>"计数: " {count}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
