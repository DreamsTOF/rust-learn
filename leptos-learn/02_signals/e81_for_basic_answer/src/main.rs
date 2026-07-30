use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (items, _set_items) = signal(vec!["苹果", "香蕉", "橘子"]);

    view! {
        <h3>"水果列表"</h3>
        <For each=move || items.get() key=|item| *item let:item>
            <p style="margin: 4px 0;">"🍎 " {item}</p>
        </For>
    }
}

fn main() {
    mount_to_body(Exercise);
}
