use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(Vec::<&str>::new());

    view! {
        <h3>"待办事项"</h3>
        <button on:click=move |_| set_items.update(|v| v.push("新事项"))>
            "➕ 添加"
        </button>
        <button on:click=move |_| set_items.set(Vec::new())>
            "🗑 清空"
        </button>
        <Show when=move || !items.get().is_empty()
            fallback=|| view! { <p style="color: #999;">"📭 暂无待办事项"</p> }
        >
            <For each=move || items.get() key=|item| *item let:item>
                <p style="margin: 4px 0;">"📋 " {item}</p>
            </For>
        </Show>
    }
}

fn main() {
    mount_to_body(Exercise);
}
