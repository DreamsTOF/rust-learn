use crate::state::AppState;
use crate::types::TabInfo;
use leptos::prelude::*;

pub struct TabManager;

impl TabManager {
    pub fn open_tab(state: &AppState, doc_id: &str, title: &str) {
        let tabs = state.open_tabs.get_untracked();
        if !tabs.iter().any(|t| t.doc_id == doc_id) {
            state
                .open_tabs
                .update(|t| t.push(TabInfo { doc_id: doc_id.to_string(), title: title.to_string(), is_dirty: false }));
        }
        state.active_tab_id.set(Some(doc_id.to_string()));
    }

    pub fn close_tab(state: &AppState, doc_id: &str) {
        let tabs = state.open_tabs.get_untracked();
        if let Some(idx) = tabs.iter().position(|t| t.doc_id == doc_id) {
            state.open_tabs.update(|t| {
                t.remove(idx);
            });
            if state.active_tab_id.get_untracked().as_deref() == Some(doc_id) {
                let new_tabs = state.open_tabs.get_untracked();
                let new_id = if idx > 0 && idx < new_tabs.len() {
                    Some(new_tabs[idx - 1].doc_id.clone())
                } else if idx == 0 && !new_tabs.is_empty() {
                    Some(new_tabs[0].doc_id.clone())
                } else {
                    Some(new_tabs.last().map(|t| t.doc_id.clone()).unwrap_or_default())
                };
                state.active_tab_id.set(new_id);
            }
        }
    }

    pub fn set_active_tab(state: &AppState, doc_id: &str) {
        state.active_tab_id.set(Some(doc_id.to_string()));
    }

    pub fn mark_dirty(state: &AppState, doc_id: &str, dirty: bool) {
        state.open_tabs.update(|tabs| {
            if let Some(tab) = tabs.iter_mut().find(|t| t.doc_id == doc_id) {
                tab.is_dirty = dirty;
            }
        });
    }
}
