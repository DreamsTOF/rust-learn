use leptos::prelude::*;

use crate::AuditLog;

#[server(ListAuditLogs)]
pub async fn list_audit_logs(
    action: Option<String>,
    user_id: Option<i64>,
    page: i64,
    page_size: i64,
) -> Result<Vec<AuditLog>, ServerFnError> {
    // TODO: Implement this server function
    // Hint: query audit_logs from the database with optional filtering by action and/or user_id,
    //       paginated by page and page_size. Return Vec<AuditLog> ordered by id DESC
    unimplemented!()
}
