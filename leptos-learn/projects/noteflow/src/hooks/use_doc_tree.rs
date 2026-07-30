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
    // TODO: 练习 - 在文档树中递归查找并更新文档
    // 提示: 遍历 docs，如果找到匹配 id 的文档则更新其 title、content、tags、status、updated_at
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
    // TODO: 练习 - 从文档树中递归删除文档
    docs.retain(|doc| doc.id != id);
    for doc in docs.iter_mut() {
        if !doc.children.is_empty() {
            remove_doc_from_tree(&mut doc.children, id);
        }
    }
}
