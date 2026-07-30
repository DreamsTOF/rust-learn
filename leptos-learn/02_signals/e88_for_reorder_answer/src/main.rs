use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(vec![3, 1, 4, 1, 5, 9, 2, 6]);

    let sort = move |_| {
        set_items.update(|v| v.sort());
    };

    let reverse = move |_| {
        set_items.update(|v| v.reverse());
    };

    let reset = move |_| {
        set_items.set(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    };

    view! {
        <h3>"排序与反转"</h3>
        <div style="display: flex; gap: 8px; margin-bottom: 8px;">
            <button on:click=sort>"升序排序"</button>
            <button on:click=reverse>"反转"</button>
            <button on:click=reset>"重置"</button>
        </div>
        <div style="display: flex; gap: 10px; flex-wrap: wrap;">
            <For each=move || items.get() key=|&x| x let:n>
                <span style="border: 1px solid #888; border-radius: 4px; padding: 6px 12px; font-size: 18px;">
                    {n}
                </span>
            </For>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
