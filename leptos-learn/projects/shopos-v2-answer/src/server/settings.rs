use leptos::prelude::*;

use crate::Setting;

#[server(GetAllSettings)]
pub async fn get_all_settings() -> Result<Vec<Setting>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let rows = sqlx::query("SELECT key, value, description FROM settings ORDER BY key")
            .fetch_all(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(rows
            .iter()
            .map(|row| Setting {
                key: row.get::<String, _>(0),
                value: row.get::<String, _>(1),
                description: row.get::<Option<String>, _>(2),
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(UpdateSetting)]
pub async fn update_setting(key: String, value: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let affected = sqlx::query(
            "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, CURRENT_TIMESTAMP) \
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = CURRENT_TIMESTAMP",
        )
        .bind(&key)
        .bind(&value)
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
