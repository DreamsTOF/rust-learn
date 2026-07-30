use crate::types::{DocNode, TabInfo, TagInfo, UserInfo};
use leptos::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub docs: RwSignal<Vec<DocNode>>,
    pub selected_doc_id: RwSignal<Option<String>>,
    pub open_tabs: RwSignal<Vec<TabInfo>>,
    pub active_tab_id: RwSignal<Option<String>>,
    pub current_user: RwSignal<Option<UserInfo>>,
    pub theme: RwSignal<String>,
    pub tags: RwSignal<Vec<TagInfo>>,
    pub search_query: RwSignal<String>,
    pub is_online: RwSignal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            docs: RwSignal::new(Vec::new()),
            selected_doc_id: RwSignal::new(None),
            open_tabs: RwSignal::new(Vec::new()),
            active_tab_id: RwSignal::new(None),
            current_user: RwSignal::new(None),
            theme: RwSignal::new(String::from("light")),
            tags: RwSignal::new(Vec::new()),
            search_query: RwSignal::new(String::new()),
            is_online: RwSignal::new(true),
        }
    }
}
