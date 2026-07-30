use crate::state::AppState;
use crate::types::UserInfo;
use leptos::prelude::*;
use crate::hooks::use_local_storage::use_local_storage;
use uuid::Uuid;

pub struct AuthManager;

impl AuthManager {
    pub fn login(state: &AppState, username: &str, _password: &str) -> bool {
        // TODO: 练习 - 实现登录逻辑
        // 提示: 创建 UserInfo 对象，设置 username、email（可用 username@noteflow.local）、
        //       role 设为 "editor"，更新 state.current_user，并持久化到 localStorage
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
        // TODO: 练习 - 实现注册逻辑
        // 提示: 与 login 类似，但使用传入的 email 参数
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
