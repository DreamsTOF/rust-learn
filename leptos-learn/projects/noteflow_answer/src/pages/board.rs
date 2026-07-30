use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn BoardPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");

    let columns = vec!["todo", "in_progress", "review", "done"];
    let column_names = vec!["待办", "进行中", "审核中", "已完成"];

    let get_docs_for_column = move |status: &str| -> Vec<(String, String)> {
        state
            .docs
            .get()
            .into_iter()
            .filter(|d| d.status == status)
            .map(|d| (d.id, d.title))
            .collect()
    };

    let move_doc = move |doc_id: String, new_status: String| {
        state.docs.update(|docs| {
            for doc in docs.iter_mut() {
                if doc.id == doc_id {
                    doc.status = new_status.clone();
                    doc.updated_at = chrono::Utc::now().timestamp();
                }
            }
        });
    };

    view! {
        <div class="board-page">
            <h1>"看板视图"</h1>
            <div class="board-columns">
                {columns.iter().enumerate().map(|(i, status)| {
                    let col_status = status.to_string();
                    let col_status2 = col_status.clone();
                    let col_name = column_names[i];
                    let cards = get_docs_for_column(status);

                    view! {
                        <div class="board-column"
                            on:dragover=move |ev| {
                                ev.prevent_default();
                            }
                            on:drop=move |ev: leptos::ev::DragEvent| {
                                ev.prevent_default();
                                let web_ev: &web_sys::DragEvent = ev.as_ref();
                                if let Some(dt) = web_ev.data_transfer() {
                                    if let Ok(doc_id) = dt.get_data("text/plain") {
                                        move_doc(doc_id, col_status.clone());
                                    }
                                }
                            }
                        >
                            <div class="board-column-header">
                                <h3>{col_name}</h3>
                                <span class="card-count">{cards.len()}</span>
                            </div>
                            <div class="board-cards">
                                <For
                                    each=move || get_docs_for_column(&col_status2)
                                    key=|(id, _)| id.clone()
                                    children=move |(doc_id, title)| {
                                        let drag_id = doc_id.clone();
                                        view! {
                                            <div
                                                class="board-card"
                                                draggable="true"
                                                on:dragstart=move |ev: leptos::ev::DragEvent| {
                                                    let web_ev: &web_sys::DragEvent = ev.as_ref();
                                                    if let Some(dt) = web_ev.data_transfer() {
                                                        let _ = dt.set_data("text/plain", &drag_id);
                                                    }
                                                }
                                            >
                                                {title.clone()}
                                            </div>
                                        }
                                    }
                                />
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
