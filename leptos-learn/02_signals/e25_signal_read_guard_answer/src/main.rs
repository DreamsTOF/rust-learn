use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, _set_count) = signal(99);
    let guard = count.read();
    let value = *guard;

    view! {
        <div>
            <p>"guard 读取的值: " {value}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
