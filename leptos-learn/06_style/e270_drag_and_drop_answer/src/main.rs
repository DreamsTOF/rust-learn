use leptos::prelude::*;
use leptos::web_sys;

#[component]
fn Exercise() -> impl IntoView {
    let items = RwSignal::new(vec![
        "项目 A".to_string(),
        "项目 B".to_string(),
        "项目 C".to_string(),
        "项目 D".to_string(),
    ]);
    let drag_idx = RwSignal::new(None::<usize>);

    let handle_dragstart = move |i: usize| {
        move |_ev: web_sys::DragEvent| {
            drag_idx.set(Some(i));
        }
    };

    let handle_dragover = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
    };

    let handle_drop = move |i: usize| {
        move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            if let Some(from) = drag_idx.get() {
                if from != i {
                    items.update(|items| {
                        let item = items.remove(from);
                        items.insert(i, item);
                    });
                }
            }
            drag_idx.set(None);
        }
    };

    let item_style = "padding: 0.5rem; margin: 0.25rem 0; background: #f0f0f0; border: 1px solid #ccc; cursor: grab; display: flex; align-items: center; gap: 0.5rem;";

    view! {
        <div style="padding: 1rem; max-width: 400px;">
            <h2>"拖拽排序"</h2>
            <p>"拖拽项目以重新排序:"</p>
            <div style="padding: 0; display: flex; flex-direction: column;">
                {move || items.get().iter().enumerate().map(|(i, item)| {
                    let item = item.clone();
                    view! {
                        <div draggable="true"
                            on:dragstart=handle_dragstart(i)
                            on:dragover=handle_dragover
                            on:drop=handle_drop(i)
                            style={item_style}
                        >
                            <span>::</span>
                            <span>{item}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()
                }
            </div>
            <hr />
            <h3>"当前顺序"</h3>
            <ol>
                {move || items.get().iter().map(|item| {
                    view! { <li>{item.clone()}</li> }
                }).collect::<Vec<_>>()
                }
            </ol>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
