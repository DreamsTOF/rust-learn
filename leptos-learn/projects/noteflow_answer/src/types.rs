use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DocNode {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_folder: bool,
    pub children: Vec<DocNode>,
    pub parent_id: Option<String>,
    pub tags: Vec<String>,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl DocNode {
    pub fn new(title: String, is_folder: bool, parent_id: Option<String>) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content: String::new(),
            is_folder,
            children: Vec::new(),
            parent_id,
            tags: Vec::new(),
            status: String::from("todo"),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TabInfo {
    pub doc_id: String,
    pub title: String,
    pub is_dirty: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub role: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TagInfo {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: String,
    pub doc_id: String,
    pub user_id: String,
    pub username: String,
    pub selected_text: String,
    pub content: String,
    pub resolved: bool,
    pub created_at: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ActivityEvent {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub action: String,
    pub target_type: String,
    pub target_id: String,
    pub target_name: String,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub member_ids: Vec<String>,
    pub created_at: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TocItem {
    pub level: usize,
    pub text: String,
    pub anchor: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct FavoritesData {
    pub favorites: Vec<String>,
    pub recent_docs: Vec<(String, i64)>,
}
