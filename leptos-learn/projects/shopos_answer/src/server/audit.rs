use leptos::prelude::*;

use crate::AuditLog;

#[server(ListAuditLogs)]
pub async fn list_audit_logs(
    action: Option<String>,
    user_id: Option<i64>,
    page: i64,
    page_size: i64,
) -> Result<Vec<AuditLog>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let offset = (page - 1) * page_size;

        let mut has_action = false;
        let mut has_user = false;

        let mut sql = String::from(
            "SELECT id, user_id, action, resource, resource_id, detail, ip_address, created_at FROM audit_logs WHERE 1=1",
        );

        if action.is_some() {
            sql.push_str(" AND action = ?");
            has_action = true;
        }
        if user_id.is_some() {
            sql.push_str(" AND user_id = ?");
            has_user = true;
        }

        sql.push_str(" ORDER BY id DESC LIMIT ? OFFSET ?");

        let mut query = sqlx::query(&sql);
        if let Some(ref a) = action {
            query = query.bind(a);
        }
        if let Some(ref uid) = user_id {
            query = query.bind(uid);
        }
        query = query.bind(page_size);
        query = query.bind(offset);

        let rows = query
            .fetch_all(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(rows
            .iter()
            .map(|row| AuditLog {
                id: row.get::<i64, _>(0),
                user_id: row.get::<Option<i64>, _>(1),
                action: row.get::<String, _>(2),
                resource: row.get::<String, _>(3),
                resource_id: row.get::<Option<String>, _>(4),
                detail: row.get::<Option<String>, _>(5),
                ip_address: row.get::<Option<String>, _>(6),
                created_at: row.get::<String, _>(7),
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
