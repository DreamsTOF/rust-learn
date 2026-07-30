use crate::state::AppState;
use crate::types::FavoritesData;
use leptos::prelude::*;
use crate::hooks::use_local_storage::use_local_storage;

pub struct FavoritesManager;

impl FavoritesManager {
    pub fn toggle_favorite(_state: &AppState, doc_id: &str) {
        let (stored, set_stored) = use_local_storage::<FavoritesData>("noteflow-favorites");
        let mut data = stored.get_untracked();
        // TODO: 练习 - 实现收藏切换
        // 提示: 如果 doc_id 已在收藏列表中则移除，否则添加
        if data.favorites.contains(&doc_id.to_string()) {
            data.favorites.retain(|id| id != doc_id);
        } else {
            data.favorites.push(doc_id.to_string());
        }
        set_stored.set(data);
    }

    pub fn is_favorite(_state: &AppState, doc_id: &str) -> bool {
        let (stored, _) = use_local_storage::<FavoritesData>("noteflow-favorites");
        let data = stored.get_untracked();
        data.favorites.contains(&doc_id.to_string())
    }

    pub fn get_favorites(_state: &AppState) -> Vec<String> {
        let (stored, _) = use_local_storage::<FavoritesData>("noteflow-favorites");
        stored.get_untracked().favorites
    }

    pub fn add_recent(_state: &AppState, doc_id: &str) {
        let (stored, set_stored) = use_local_storage::<FavoritesData>("noteflow-favorites");
        let mut data = stored.get_untracked();
        // TODO: 练习 - 添加最近文档记录，最多保留 20 条
        let now = chrono::Utc::now().timestamp();
        data.recent_docs.retain(|(id, _)| id != doc_id);
        data.recent_docs.push((doc_id.to_string(), now));
        if data.recent_docs.len() > 20 {
            data.recent_docs.remove(0);
        }
        set_stored.set(data);
    }

    pub fn get_recent(_state: &AppState) -> Vec<(String, i64)> {
        let (stored, _) = use_local_storage::<FavoritesData>("noteflow-favorites");
        stored.get_untracked().recent_docs
    }
}
