use leptos::prelude::*;

use crate::state::UserInfo;

#[server(GetUserProfile)]
pub async fn get_user_profile(user_id: i64) -> Result<UserInfo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let row = sqlx::query(
            "SELECT id, username, email, role, avatar_url FROM users WHERE id = ?",
        )
        .bind(user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("用户不存在"))?;

        Ok(UserInfo {
            id: row.get(0),
            username: row.get(1),
            email: row.get(2),
            role: row.get(3),
            avatar_url: row.get(4),
        })
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
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        if let Some(ref name) = username {
            sqlx::query("UPDATE users SET username = ? WHERE id = ?")
                .bind(name)
                .bind(user_id)
                .execute(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }

        if let Some(ref mail) = email {
            sqlx::query("UPDATE users SET email = ? WHERE id = ?")
                .bind(mail)
                .bind(user_id)
                .execute(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }

        if let Some(ref url) = avatar_url {
            sqlx::query("UPDATE users SET avatar_url = ? WHERE id = ?")
                .bind(url)
                .bind(user_id)
                .execute(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }

        let row = sqlx::query(
            "SELECT id, username, email, role, avatar_url FROM users WHERE id = ?",
        )
        .bind(user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("用户不存在"))?;

        Ok(UserInfo {
            id: row.get(0),
            username: row.get(1),
            email: row.get(2),
            role: row.get(3),
            avatar_url: row.get(4),
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
