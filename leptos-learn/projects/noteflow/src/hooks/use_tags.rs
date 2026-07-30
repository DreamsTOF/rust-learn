use crate::state::AppState;
use crate::types::TagInfo;
use leptos::prelude::*;
use crate::hooks::use_local_storage::use_local_storage;
use uuid::Uuid;

pub struct TagManager;

impl TagManager {
    pub fn add(state: &AppState, name: &str, color: &str) {
        let tag = TagInfo {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            color: color.to_string(),
        };
        state.tags.update(|t| t.push(tag));
        Self::persist(state);
    }

    pub fn remove(state: &AppState, id: &str) {
        state.tags.update(|t| t.retain(|tag| tag.id != id));
        Self::persist(state);
    }

    pub fn persist(state: &AppState) {
        // TODO: 练习 - 持久化标签数据到 localStorage
        // 提示: 使用 use_local_storage 将 state.tags 保存到 "noteflow-tags" 键
        let tags = state.tags.get_untracked();
        let (_, set_stored) = use_local_storage::<Vec<TagInfo>>("noteflow-tags");
        set_stored.set(tags);
    }

    pub fn load(state: &AppState) {
        // TODO: 练习 - 从 localStorage 加载标签数据
        // 提示: 使用 use_local_storage 从 "noteflow-tags" 键读取数据并设置到 state.tags
        let (stored, _) = use_local_storage::<Vec<TagInfo>>("noteflow-tags");
        let tags = stored.get_untracked();
        state.tags.set(tags);
    }
}
