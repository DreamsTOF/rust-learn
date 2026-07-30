use leptos::prelude::*;

use crate::Return;

#[server(RequestRefund)]
pub async fn request_refund(
    order_id: i64,
    reason: String,
    amount: f64,
    user_id: i64,
) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // Check order exists and belongs to user
        let order = sqlx::query("SELECT id, user_id, status FROM orders WHERE id = ?")
            .bind(order_id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .ok_or_else(|| ServerFnError::new("订单不存在"))?;

        let order_user_id: i64 = order.get(1);
        if order_user_id != user_id {
            return Err(ServerFnError::new("无权操作此订单"));
        }

        // Check if a return already exists for this order
        let existing: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM returns WHERE order_id = ?")
                .bind(order_id)
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;

        if existing > 0 {
            return Err(ServerFnError::new("该订单已申请过退款"));
        }

        let result = sqlx::query(
            "INSERT INTO returns (order_id, user_id, reason, status, refund_amount) VALUES (?, ?, ?, 'pending_review', ?)",
        )
        .bind(order_id)
        .bind(user_id)
        .bind(&reason)
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

#[server(ListReturns)]
pub async fn list_returns(
    status: Option<String>,
    page: i64,
    page_size: i64,
) -> Result<Vec<Return>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let offset = (page - 1) * page_size;

        let rows = if let Some(ref st) = status {
            sqlx::query(
                "SELECT id, order_id, user_id, reason, status, refund_amount, admin_remark \
                 FROM returns WHERE status = ? ORDER BY id DESC LIMIT ? OFFSET ?",
            )
            .bind(st)
            .bind(page_size)
            .bind(offset)
            .fetch_all(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
        } else {
            sqlx::query(
                "SELECT id, order_id, user_id, reason, status, refund_amount, admin_remark \
                 FROM returns ORDER BY id DESC LIMIT ? OFFSET ?",
            )
            .bind(page_size)
            .bind(offset)
            .fetch_all(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
        };

        Ok(rows
            .iter()
            .map(|row| Return {
                id: row.get::<i64, _>(0),
                order_id: row.get::<i64, _>(1),
                user_id: row.get::<i64, _>(2),
                reason: row.get::<String, _>(3),
                status: row.get::<String, _>(4),
                refund_amount: row.get::<f64, _>(5),
                admin_remark: row.get::<Option<String>, _>(6),
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(ReviewReturn)]
pub async fn review_return(
    id: i64,
    approved: bool,
    remark: Option<String>,
) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let new_status = if approved { "approved" } else { "rejected" };

        let affected = sqlx::query(
            "UPDATE returns SET status = ?, admin_remark = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(new_status)
        .bind(&remark)
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

#[server(ProcessRefund)]
pub async fn process_refund(id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let affected = sqlx::query(
            "UPDATE returns SET status = 'refunded', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND status = 'approved'",
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
