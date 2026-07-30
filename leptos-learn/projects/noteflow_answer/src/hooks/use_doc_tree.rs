use crate::types::DocNode;
use leptos::prelude::*;
use crate::hooks::use_local_storage::use_local_storage;

pub fn use_doc_tree() -> (ReadSignal<Vec<DocNode>>, WriteSignal<Vec<DocNode>>) {
    use_local_storage::<Vec<DocNode>>("noteflow-docs")
}

pub fn find_doc_by_id(docs: &[DocNode], id: &str) -> Option<DocNode> {
    for doc in docs {
        if doc.id == id {
            return Some(doc.clone());
        }
        if !doc.children.is_empty() {
            if let found @ Some(_) = find_doc_by_id(&doc.children, id) {
                return found;
            }
        }
    }
    None
}

pub fn update_doc_in_tree(docs: &mut Vec<DocNode>, updated: DocNode) {
    for doc in docs.iter_mut() {
        if doc.id == updated.id {
            doc.title = updated.title;
            doc.content = updated.content;
            doc.tags = updated.tags;
            doc.status = updated.status;
            doc.updated_at = chrono::Utc::now().timestamp();
            return;
        }
        if !doc.children.is_empty() {
            update_doc_in_tree(&mut doc.children, updated.clone());
        }
    }
}

pub fn remove_doc_from_tree(docs: &mut Vec<DocNode>, id: &str) {
    docs.retain(|doc| doc.id != id);
    for doc in docs.iter_mut() {
        if !doc.children.is_empty() {
            remove_doc_from_tree(&mut doc.children, id);
        }
    }
}
