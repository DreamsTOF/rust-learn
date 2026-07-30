use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn LinkGraph() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let backlinks = RwSignal::new(Vec::<(String, String)>::new()); // (id, title)

    // Parse [[WikiLink]] syntax
    let extract_wikilinks = move |content: &str| -> Vec<String> {
        // TODO: 练习 - 解析 [[WikiLink]] 语法
        // 提示: 遍历每一行，查找 [[ 和 ]] 之间的内容，提取链接目标
        let mut links = Vec::new();
        for line in content.lines() {
            let mut remaining = line;
            while let Some(start) = remaining.find("[[").map(|i| i + 2) {
                remaining = &remaining[start..];
                if let Some(end) = remaining.find("]]") {
                    let link_target = &remaining[..end];
                    links.push(link_target.to_string());
                    remaining = &remaining[end + 2..];
                }
            }
        }
        links
    };

    // Find backlinks
    let update_backlinks = move || {
        let docs = state.docs.get();
        let active_id = state.active_tab_id.get();
        if let Some(id) = active_id {
            let mut found = Vec::new();
            for doc in docs.iter() {
                if doc.id != id {
                    let links = extract_wikilinks(&doc.content);
                    if links.iter().any(|l| doc.title.contains(l) || l.contains(&doc.title)) {
                        found.push((doc.id.clone(), doc.title.clone()));
                    }
                }
            }
            backlinks.set(found);
        }
    };

    // Run initially
    update_backlinks();

    view! {
        <div class="link-graph">
            <h4>"链接图谱"</h4>
            <div class="backlinks-section">
                <h5>"反向链接"</h5>
                <For
                    each=move || backlinks.get()
                    key=|(id, _)| id.clone()
                    children=move |(id, title)| {
                        view! {
                            <div
                                class="backlink-item"
                                on:click=move |_| {
                                    state.selected_doc_id.set(Some(id.clone()));
                                    state.active_tab_id.set(Some(id.clone()));
                                }
                            >
                                "← " {title.clone()}
                            </div>
                        }
                    }
                />
                {move || {
                    if backlinks.get().is_empty() {
                        view! { <div class="no-links">"暂无反向链接"</div> }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
