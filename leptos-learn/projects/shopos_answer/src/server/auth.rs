use leptos::prelude::*;

use crate::state::UserInfo;

#[server(RegisterUser)]
pub async fn register_user(
    username: String,
    email: String,
    password: String,
) -> Result<UserInfo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // Check if username or email already exists
        let existing: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE username = ? OR email = ?",
        )
        .bind(&username)
        .bind(&email)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        if existing > 0 {
            return Err(ServerFnError::new("用户名或邮箱已被注册"));
        }

        let password_hash = hash_password(&password)?;

        let result = sqlx::query(
            "INSERT INTO users (username, email, password_hash, role) VALUES (?, ?, ?, 'user')",
        )
        .bind(&username)
        .bind(&email)
        .bind(&password_hash)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let user_id = result.last_insert_rowid();

        Ok(UserInfo {
            id: user_id,
            username,
            email,
            role: "user".to_string(),
            avatar_url: None,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(LoginUser)]
pub async fn login_user(
    username: String,
    password: String,
) -> Result<UserInfo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let row = sqlx::query(
            "SELECT id, username, email, password_hash, role, avatar_url FROM users WHERE username = ?",
        )
        .bind(&username)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("用户名或密码错误"))?;

        let password_hash: String = row.get(3);
        verify_password(&password, &password_hash)?;

        let user = UserInfo {
            id: row.get(0),
            username: row.get(1),
            email: row.get(2),
            role: row.get(4),
            avatar_url: row.get(5),
        };

        Ok(user)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(LogoutUser)]
pub async fn logout_user() -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        Ok(true)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(GetCurrentUser)]
pub async fn get_current_user(user_id: i64) -> Result<Option<UserInfo>, ServerFnError> {
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
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(UserInfo {
                id: r.get(0),
                username: r.get(1),
                email: r.get(2),
                role: r.get(3),
                avatar_url: r.get(4),
            })),
            None => Ok(None),
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(ChangePassword)]
pub async fn change_password(
    old_password: String,
    new_password: String,
    user_id: i64,
) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let row = sqlx::query("SELECT password_hash FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .ok_or_else(|| ServerFnError::new("用户不存在"))?;

        let current_hash: String = row.get(0);
        verify_password(&old_password, &current_hash)?;

        let new_hash = hash_password(&new_password)?;

        sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
            .bind(&new_hash)
            .bind(user_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(true)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[cfg(feature = "ssr")]
fn hash_password(password: &str) -> Result<String, ServerFnError> {
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    use argon2::{Argon2, PasswordHasher};

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(hash.to_string())
}

#[cfg(feature = "ssr")]
fn verify_password(password: &str, hash: &str) -> Result<(), ServerFnError> {
    use argon2::password_hash::{PasswordHash, PasswordVerifier};
    use argon2::Argon2;

    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| ServerFnError::new(e.to_string()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ServerFnError::new("用户名或密码错误"))
}
