use crate::state::AppState;
use crate::types::UserInfo;
use leptos::prelude::*;
use crate::hooks::use_local_storage::use_local_storage;
use uuid::Uuid;

pub struct AuthManager;

impl AuthManager {
    pub fn login(state: &AppState, username: &str, _password: &str) -> bool {
        let user = UserInfo {
            id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            email: format!("{}@noteflow.local", username),
            avatar: None,
            role: String::from("editor"),
        };
        state.current_user.set(Some(user.clone()));
        let (_, set_stored) = use_local_storage::<Option<UserInfo>>("noteflow-user");
        set_stored.set(Some(user));
        true
    }

    pub fn logout(state: &AppState) {
        state.current_user.set(None);
        let (_, set_stored) = use_local_storage::<Option<UserInfo>>("noteflow-user");
        set_stored.set(None);
    }

    pub fn register(state: &AppState, username: &str, email: &str, _password: &str) -> bool {
        let user = UserInfo {
            id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            email: email.to_string(),
            avatar: None,
            role: String::from("editor"),
        };
        state.current_user.set(Some(user.clone()));
        let (_, set_stored) = use_local_storage::<Option<UserInfo>>("noteflow-user");
        set_stored.set(Some(user));
        true
    }

    pub fn is_authenticated(state: &AppState) -> bool {
        state.current_user.get_untracked().is_some()
    }

    pub fn current_user(state: &AppState) -> Option<UserInfo> {
        state.current_user.get_untracked()
    }
}
