use crate::state::AppState;
use crate::types::TagInfo;
use leptos::prelude::*;

#[component]
pub fn FilterPanel() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let selected_tags = RwSignal::new(Vec::<String>::new());
    let selected_status = RwSignal::new(String::new());
    let selected_sort = RwSignal::new(String::from("updated_at"));

    let toggle_tag = move |tag_id: String| {
        let mut tags = selected_tags.get_untracked();
        if tags.contains(&tag_id) {
            tags.retain(|t| t != &tag_id);
        } else {
            tags.push(tag_id);
        }
        selected_tags.set(tags);
    };

    view! {
        <div class="filter-panel">
            <div class="filter-section">
                <h4>"标签筛选"</h4>
                <div class="filter-tags">
                    <For
                        each=move || state.tags.get()
                        key=|tag| tag.id.clone()
                        children=move |tag: TagInfo| {
                            let tag_id = tag.id.clone();
                            let tag_name = tag.name.clone();
                            let tag_id2 = tag_id.clone();
                            let is_selected = move || selected_tags.get().contains(&tag_id);
                            view! {
                                <button
                                    class={move || {
                                        if is_selected() { "tag-btn active" } else { "tag-btn" }
                                    }}
                                    on:click=move |_| toggle_tag(tag_id2.clone())
                                >
                                    {tag_name.clone()}
                                </button>
                            }
                        }
                    />
                </div>
            </div>
            <div class="filter-section">
                <h4>"状态筛选"</h4>
                <select
                    prop:value=selected_status
                    on:change=move |ev| { selected_status.set(event_target_value(&ev)); }
                >
                    <option value="">"全部"</option>
                    <option value="todo">"待办"</option>
                    <option value="in_progress">"进行中"</option>
                    <option value="review">"审核中"</option>
                    <option value="done">"已完成"</option>
                </select>
            </div>
            <div class="filter-section">
                <h4>"排序方式"</h4>
                <select
                    prop:value=selected_sort
                    on:change=move |ev| { selected_sort.set(event_target_value(&ev)); }
                >
                    <option value="updated_at">"更新时间"</option>
                    <option value="created_at">"创建时间"</option>
                    <option value="title">"标题"</option>
                </select>
            </div>
        </div>
    }
}
