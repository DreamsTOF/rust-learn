use leptos::prelude::*;

use crate::Address;

#[server(ListAddresses)]
pub async fn list_addresses(user_id: i64) -> Result<Vec<Address>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let rows = sqlx::query(
            "SELECT id, user_id, receiver_name, phone, province, city, district, detail, is_default FROM addresses WHERE user_id = ? ORDER BY is_default DESC, id DESC",
        )
        .bind(user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(rows
            .iter()
            .map(|row| Address {
                id: row.get(0),
                user_id: row.get(1),
                receiver_name: row.get(2),
                phone: row.get(3),
                province: row.get(4),
                city: row.get(5),
                district: row.get(6),
                detail: row.get(7),
                is_default: row.get::<i64, _>(8) != 0,
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(CreateAddress)]
pub async fn create_address(
    receiver_name: String,
    phone: String,
    province: String,
    city: String,
    district: String,
    detail: String,
    is_default: bool,
    user_id: i64,
) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // If setting as default, unset existing default first
        if is_default {
            sqlx::query("UPDATE addresses SET is_default = 0 WHERE user_id = ?")
                .bind(user_id)
                .execute(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }

        let result = sqlx::query(
            "INSERT INTO addresses (user_id, receiver_name, phone, province, city, district, detail, is_default) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(user_id)
        .bind(&receiver_name)
        .bind(&phone)
        .bind(&province)
        .bind(&city)
        .bind(&district)
        .bind(&detail)
        .bind(if is_default { 1i64 } else { 0i64 })
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(result.last_insert_rowid())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(UpdateAddress)]
pub async fn update_address(
    id: i64,
    data: String,
    user_id: i64,
) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let addr: Address = serde_json::from_str(&data)
            .map_err(|e| ServerFnError::new(format!("Invalid address data: {}", e)))?;

        if addr.is_default {
            sqlx::query("UPDATE addresses SET is_default = 0 WHERE user_id = ? AND id != ?")
                .bind(user_id)
                .bind(id)
                .execute(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }

        let affected = sqlx::query(
            "UPDATE addresses SET receiver_name = ?, phone = ?, province = ?, city = ?, district = ?, detail = ?, is_default = ? WHERE id = ? AND user_id = ?",
        )
        .bind(&addr.receiver_name)
        .bind(&addr.phone)
        .bind(&addr.province)
        .bind(&addr.city)
        .bind(&addr.district)
        .bind(&addr.detail)
        .bind(if addr.is_default { 1i64 } else { 0i64 })
        .bind(id)
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .rows_affected();

        Ok(affected > 0)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(DeleteAddress)]
pub async fn delete_address(id: i64, user_id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let affected = sqlx::query("DELETE FROM addresses WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .rows_affected();

        Ok(affected > 0)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(SetDefaultAddress)]
pub async fn set_default_address(id: i64, user_id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // Unset all defaults for this user
        sqlx::query("UPDATE addresses SET is_default = 0 WHERE user_id = ?")
            .bind(user_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Set the new default
        let affected = sqlx::query("UPDATE addresses SET is_default = 1 WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .rows_affected();

        Ok(affected > 0)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
