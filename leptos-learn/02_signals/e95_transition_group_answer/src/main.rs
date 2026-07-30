use leptos::prelude::*;
use std::collections::HashSet;

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(Vec::<(usize, String)>::new());
    let (leaving, set_leaving) = signal(HashSet::<usize>::new());
    let next_id = std::cell::Cell::new(0usize);

    let add_item = move |_| {
        let id = next_id.get();
        next_id.set(id + 1);
        set_items.update(|v| v.push((id, format!("项目 {}", id + 1))));
    };

    let remove_item = move |id: usize| {
        set_leaving.update(|s| {
            s.insert(id);
        });
        set_timeout(
            move || {
                set_items.update(|v| v.retain(|(i, _)| *i != id));
                set_leaving.update(|s| {
                    s.remove(&id);
                });
            },
            std::time::Duration::from_millis(300),
        );
    };

    view! {
        <style>
            "
            @keyframes fadeIn {
                from { opacity: 0; transform: translateY(-10px); }
                to   { opacity: 1; transform: translateY(0); }
            }
            @keyframes fadeOut {
                from { opacity: 1; transform: translateY(0); }
                to   { opacity: 0; transform: translateY(-10px); }
            }
            .item-enter { animation: fadeIn 0.3s ease-out; }
            .item-leave { animation: fadeOut 0.3s ease-out; }
            "
        </style>

        <h2>"列表动画"</h2>
        <button on:click=add_item>"添加项目"</button>
        <ul style="list-style: none; padding: 0;">
            <For
                each=move || items.get()
                key=|(id, _)| *id
                children=move |(id, name)| {
                    let is_leaving = move || leaving.with(|s| s.contains(&id));
                    view! {
                        <li
                            class:item-enter=move || !leaving.with(|s| s.contains(&id))
                            class:item-leave=is_leaving
                            style="padding: 0.5rem; margin: 0.25rem 0;
                                   background: #e3f2fd; border-radius: 4px;
                                   display: flex; justify-content: space-between;
                                   align-items: center;"
                        >
                            <span>{name}</span>
                            <button on:click=move |_| remove_item(id)>
                                "删除"
                            </button>
                        </li>
                    }
                }
            />
        </ul>
    }
}

fn main() {
    mount_to_body(Exercise);
}
