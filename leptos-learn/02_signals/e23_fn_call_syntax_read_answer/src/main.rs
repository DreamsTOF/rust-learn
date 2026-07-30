use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, _set_count) = signal(100);

    view! {
        <div>
            <p>"count() = " {count()}</p>
            <p>"count.get() = " {count.get()}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
