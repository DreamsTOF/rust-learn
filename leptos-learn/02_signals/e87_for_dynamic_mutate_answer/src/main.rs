use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(vec!["A", "B", "C"]);

    let add_item = move |_| {
        set_items.update(|v| v.push("新项"));
    };

    let remove_item = move |_| {
        set_items.update(|v| {
            v.pop();
        });
    };

    view! {
        <h3>"动态列表"</h3>
        <div style="display: flex; gap: 8px; margin-bottom: 8px;">
            <button on:click=add_item>"添加"</button>
            <button on:click=remove_item>"删除末尾"</button>
        </div>
        <ul>
            <For each=move || items.get() key=|&x| x let:item>
                <li>{item}</li>
            </For>
        </ul>
    }
}

fn main() {
    mount_to_body(Exercise);
}
