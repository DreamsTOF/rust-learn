use crate::state::AppState;
use leptos::prelude::*;
use crate::hooks::use_local_storage::use_local_storage;

pub struct ThemeManager;

impl ThemeManager {
    pub fn toggle(state: &AppState) {
        // TODO: 练习 - 实现主题切换
        // 提示: 读取当前主题，在 "light" 和 "dark" 之间切换，并持久化到 localStorage
        let current = state.theme.get_untracked();
        let new = if current == "light" {
            "dark".to_string()
        } else {
            "light".to_string()
        };
        state.theme.set(new.clone());
        let (_, set_stored) = use_local_storage::<String>("noteflow-theme");
        set_stored.set(new);
    }

    pub fn init(state: &AppState) {
        let (stored, _) = use_local_storage::<String>("noteflow-theme");
        let theme = stored.get_untracked();
        if !theme.is_empty() {
            state.theme.set(theme);
        }
    }

    pub fn is_dark(state: &AppState) -> bool {
        state.theme.get_untracked() == "dark"
    }
}
