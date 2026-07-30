use leptos::prelude::*;

use crate::Invoice;

#[server(ApplyInvoice)]
pub async fn apply_invoice(
    order_id: i64,
    invoice_type: String,
    title: String,
    tax_number: Option<String>,
    user_id: i64,
) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // Get order total
        let order = sqlx::query("SELECT actual_amount FROM orders WHERE id = ? AND user_id = ?")
            .bind(order_id)
            .bind(user_id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .ok_or_else(|| ServerFnError::new("订单不存在"))?;

        let amount: f64 = order.get(0);

        // Check if invoice already exists for this order
        let existing: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM invoices WHERE order_id = ?")
                .bind(order_id)
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;

        if existing > 0 {
            return Err(ServerFnError::new("该订单已申请过发票"));
        }

        let result = sqlx::query(
            "INSERT INTO invoices (order_id, user_id, invoice_type, title, tax_number, amount, status) VALUES (?, ?, ?, ?, ?, ?, 'pending')",
        )
        .bind(order_id)
        .bind(user_id)
        .bind(&invoice_type)
        .bind(&title)
        .bind(&tax_number)
        .bind(amount)
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

#[server(ListInvoices)]
pub async fn list_invoices(
    status: Option<String>,
    page: i64,
    page_size: i64,
) -> Result<Vec<Invoice>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let offset = (page - 1) * page_size;

        let rows = if let Some(ref st) = status {
            sqlx::query(
                "SELECT id, order_id, user_id, invoice_type, title, tax_number, amount, status \
                 FROM invoices WHERE status = ? ORDER BY id DESC LIMIT ? OFFSET ?",
            )
            .bind(st)
            .bind(page_size)
            .bind(offset)
            .fetch_all(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
        } else {
            sqlx::query(
                "SELECT id, order_id, user_id, invoice_type, title, tax_number, amount, status \
                 FROM invoices ORDER BY id DESC LIMIT ? OFFSET ?",
            )
            .bind(page_size)
            .bind(offset)
            .fetch_all(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
        };

        Ok(rows
            .iter()
            .map(|row| Invoice {
                id: row.get::<i64, _>(0),
                order_id: row.get::<i64, _>(1),
                user_id: row.get::<i64, _>(2),
                invoice_type: row.get::<String, _>(3),
                title: row.get::<String, _>(4),
                tax_number: row.get::<Option<String>, _>(5),
                amount: row.get::<f64, _>(6),
                status: row.get::<String, _>(7),
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(ApproveInvoice)]
pub async fn approve_invoice(id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let affected = sqlx::query(
            "UPDATE invoices SET status = 'approved' WHERE id = ? AND status = 'pending'",
        )
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
