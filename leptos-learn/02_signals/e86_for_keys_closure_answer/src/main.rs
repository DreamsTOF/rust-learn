use leptos::prelude::*;

#[derive(Debug, Clone)]
struct Item {
    id: u32,
    name: &'static str,
}

#[component]
fn Exercise() -> impl IntoView {
    let items = vec![
        Item { id: 1, name: "苹果" },
        Item { id: 2, name: "香蕉" },
        Item { id: 3, name: "樱桃" },
    ];

    view! {
        <h3>"水果列表"</h3>
        <ul>
            <For each=move || items.clone() key=|item| item.id let:item>
                <li>{item.name} " (ID: " {item.id} ")"</li>
            </For>
        </ul>
    }
}

fn main() {
    mount_to_body(Exercise);
}
