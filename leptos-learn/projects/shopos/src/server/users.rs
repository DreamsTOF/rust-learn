use leptos::prelude::*;

use crate::state::UserInfo;

#[server(GetUserProfile)]
pub async fn get_user_profile(user_id: i64) -> Result<UserInfo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Fetch user profile by ID from users table
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(UpdateUserProfile)]
pub async fn update_user_profile(
    username: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,
    user_id: i64,
) -> Result<UserInfo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Dynamically update username/email/avatar_url if Some, then return updated UserInfo
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
