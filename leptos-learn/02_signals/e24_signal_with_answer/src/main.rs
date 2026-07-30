use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, _set_count) = signal(7);
    let doubled = count.with(|n| *n * 2);

    view! {
        <div>
            <p>"原始值: " {count()}</p>
            <p>"翻倍后: " {doubled}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
