use crate::state::AppState;
use leptos::prelude::GetUntracked;

pub enum PermissionLevel {
    Owner,
    Admin,
    Editor,
    Viewer,
}

pub struct Permissions {
    pub level: PermissionLevel,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_share: bool,
    pub can_manage_members: bool,
}

pub fn use_permissions(state: &AppState, _doc_id: &str) -> Permissions {
    let user = state.current_user.get_untracked();
    let level = match user.as_ref().map(|u| u.role.as_str()) {
        Some("owner") => PermissionLevel::Owner,
        Some("admin") => PermissionLevel::Admin,
        Some("editor") => PermissionLevel::Editor,
        _ => PermissionLevel::Viewer,
    };
    Permissions {
        can_edit: matches!(level, PermissionLevel::Owner | PermissionLevel::Admin | PermissionLevel::Editor),
        can_delete: matches!(level, PermissionLevel::Owner | PermissionLevel::Admin),
        can_share: matches!(level, PermissionLevel::Owner | PermissionLevel::Admin | PermissionLevel::Editor),
        can_manage_members: matches!(level, PermissionLevel::Owner | PermissionLevel::Admin),
        level,
    }
}
