use crate::state::AppState;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct VersionEntry {
    id: String,
    doc_id: String,
    content: String,
    summary: String,
    timestamp: i64,
}

#[component]
pub fn VersionHistory() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");
    let selected_version = RwSignal::new(Option::<VersionEntry>::None);
    let preview_content = RwSignal::new(String::new());

    let doc_id = move || state.active_tab_id.get();
    let _versions_key = move || format!("noteflow-versions-{}", doc_id().unwrap_or_default());

    let current_doc = move || {
        let docs = state.docs.get();
        let id = state.active_tab_id.get();
        id.and_then(|id| docs.into_iter().find(|d| d.id == id))
    };

    let restore_version = move |version: VersionEntry| {
        let id = state.active_tab_id.get_untracked();
        if let Some(doc_id) = id {
            state.docs.update(|docs| {
                for doc in docs.iter_mut() {
                    if doc.id == doc_id {
                        doc.content = version.content.clone();
                        doc.updated_at = chrono::Utc::now().timestamp();
                    }
                }
            });
        }
    };

    view! {
        <div class="version-history">
            <h3>"版本历史"</h3>
            <div class="version-list">
                <For
                    each=move || {
                        // Use simulated versions
                        let doc = current_doc();
                        match doc {
                            Some(d) => vec![
                                VersionEntry {
                                    id: "v1".to_string(),
                                    doc_id: d.id.clone(),
                                    content: d.content.clone(),
                                    summary: "当前版本".to_string(),
                                    timestamp: d.updated_at,
                                }
                            ],
                            None => Vec::new(),
                        }
                    }
                    key=|v| v.id.clone()
                    children=move |version: VersionEntry| {
                        let v1 = version.clone();
                        view! {
                            <div class="version-item">
                                <div class="version-info">
                                    <span class="version-summary">{version.summary.clone()}</span>
                                    <span class="version-time">
                                        {chrono::DateTime::from_timestamp(version.timestamp, 0)
                                            .map(|dt| dt.format("%m-%d %H:%M").to_string())
                                            .unwrap_or_default()}
                                    </span>
                                </div>
                                <div class="version-actions">
                                    <button on:click=move |_| {
                                        preview_content.set(v1.content.clone());
                                        selected_version.set(Some(v1.clone()));
                                    }>"预览"</button>
                                    <button on:click=move |_| restore_version(version.clone())>"恢复"</button>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
            {move || selected_version.get().map(|v| {
                view! {
                    <div class="version-preview">
                        <h4>"版本预览"</h4>
                        <div class="preview-content">{v.content.clone()}</div>
                    </div>
                }
            })}
        </div>
    }
}
