use leptos::prelude::*;

use crate::Notification;

#[server(GetNotifications)]
pub async fn get_notifications(user_id: i64) -> Result<Vec<Notification>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let rows = sqlx::query(
            "SELECT id, user_id, title, content, is_read, created_at FROM notifications WHERE user_id = ? ORDER BY id DESC LIMIT 50",
        )
        .bind(user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(rows
            .iter()
            .map(|row| Notification {
                id: row.get::<i64, _>(0),
                user_id: row.get::<i64, _>(1),
                title: row.get::<String, _>(2),
                content: row.get::<Option<String>, _>(3),
                is_read: row.get::<i64, _>(4) != 0,
                created_at: row.get::<String, _>(5),
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(MarkNotificationRead)]
pub async fn mark_notification_read(id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let affected = sqlx::query("UPDATE notifications SET is_read = 1 WHERE id = ?")
            .bind(id)
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
