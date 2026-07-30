use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (items, _set_items) = signal(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    let (min_value, set_min_value) = signal(3);

    let filtered = move || {
        let min = min_value.get();
        items.get().into_iter().filter(move |&n| n >= min).collect::<Vec<i32>>()
    };

    view! {
        <h3>"列表过滤"</h3>
        <div style="display: flex; gap: 8px; margin-bottom: 4px;">
            <span>"最小值: " {min_value}</span>
        </div>
        <div style="display: flex; gap: 4px; margin-bottom: 8px;">
            <button on:click=move |_| set_min_value.set(1)>"≥1"</button>
            <button on:click=move |_| set_min_value.set(3)>"≥3"</button>
            <button on:click=move |_| set_min_value.set(5)>"≥5"</button>
            <button on:click=move |_| set_min_value.set(7)>"≥7"</button>
        </div>
        <p>"原始: " {move || format!("{:?}", items.get())}</p>
        <p>"过滤后: "</p>
        <div style="display: flex; gap: 8px; flex-wrap: wrap;">
            <For each=move || filtered() key=|&x| x let:n>
                <span style="border: 1px solid #4a90d9; border-radius: 4px; padding: 4px 10px;">
                    {n}
                </span>
            </For>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
