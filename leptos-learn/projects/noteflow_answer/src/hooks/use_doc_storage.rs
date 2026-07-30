use crate::types::DocNode;
use leptos::prelude::*;
use crate::hooks::use_local_storage::use_local_storage;

pub struct DocStorage;

impl DocStorage {
    pub fn save_docs(docs: &[DocNode]) {
        let (_, set_stored) = use_local_storage::<Vec<DocNode>>("noteflow-docs");
        set_stored.set(docs.to_vec());
    }

    pub fn load_docs() -> Vec<DocNode> {
        let (stored, _) = use_local_storage::<Vec<DocNode>>("noteflow-docs");
        stored.get_untracked()
    }

    pub fn save_doc_content(doc_id: &str, content: &str) {
        let key = format!("noteflow-content-{}", doc_id);
        let (_, set_stored) = use_local_storage::<String>(&key);
        set_stored.set(content.to_string());
    }

    pub fn load_doc_content(doc_id: &str) -> String {
        let key = format!("noteflow-content-{}", doc_id);
        let (stored, _) = use_local_storage::<String>(&key);
        stored.get_untracked()
    }
}
