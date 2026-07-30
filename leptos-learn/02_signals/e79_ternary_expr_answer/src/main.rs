use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <p>
            "数字 "
            {move || count.get()}
            " 是"
            {move || if count.get() % 2 == 0 { "偶数" } else { "奇数" }}
        </p>
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "加一"
        </button>
    }
}

fn main() {
    mount_to_body(Exercise);
}
