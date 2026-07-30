use leptos::prelude::*;

use crate::state::UserInfo;

#[server(RegisterUser)]
pub async fn register_user(
    username: String,
    email: String,
    password: String,
) -> Result<UserInfo, ServerFnError> {
    // TODO: Implement this server function
    // Hint: check if the username or email already exists; if not, hash the password
    //       with hash_password(), insert the new user, and return UserInfo
    unimplemented!()
}

#[server(LoginUser)]
pub async fn login_user(
    username: String,
    password: String,
) -> Result<UserInfo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // ⭐⭐ TODO (A-12): 实现用户登录
        // 1. 获取数据库连接池: let pool = use_context::<sqlx::SqlitePool>()...;
        // 2. 根据用户名查询用户记录，使用 verify_password() 验证密码
        // 3. 返回 UserInfo
        todo!("implement login_user");
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
    // TODO: Implement this server function
    // Hint: query the user with the given user_id from the database
    //       and return it as Option<UserInfo>
    unimplemented!()
}

#[server(ChangePassword)]
pub async fn change_password(
    old_password: String,
    new_password: String,
    user_id: i64,
) -> Result<bool, ServerFnError> {
    // TODO: Implement this server function
    // Hint: query the user, verify the old password with verify_password(),
    //       then hash the new password with hash_password() and update the database.
    //       Return true on success
    unimplemented!()
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
    // TODO: Implement this helper function
    // Hint: parse the hash and verify the password against it using argon2.
    //       Return Ok(()) if valid, or an error with "用户名或密码错误" if invalid
    unimplemented!()
}
