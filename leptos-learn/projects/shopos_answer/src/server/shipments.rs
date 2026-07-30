use leptos::prelude::*;

use crate::Shipment;

#[server(ShipOrder)]
pub async fn ship_order(
    order_id: i64,
    tracking_number: String,
    carrier: String,
) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // Verify order exists and is in 'paid' status (ready to ship)
        let row = sqlx::query("SELECT status FROM orders WHERE id = ?")
            .bind(order_id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .ok_or_else(|| ServerFnError::new("订单不存在"))?;

        let status: String = row.get(0);
        if status != "paid" {
            return Err(ServerFnError::new("订单状态不允许发货"));
        }

        // Update order status to shipped
        sqlx::query("UPDATE orders SET status = 'shipped', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(order_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let result = sqlx::query(
            "INSERT INTO shipments (order_id, tracking_number, carrier, status) VALUES (?, ?, ?, 'pending')",
        )
        .bind(order_id)
        .bind(&tracking_number)
        .bind(&carrier)
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

#[server(GetShippingInfo)]
pub async fn get_shipping_info(order_id: i64) -> Result<Option<Shipment>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let row = sqlx::query(
            "SELECT id, order_id, tracking_number, carrier, status FROM shipments WHERE order_id = ?",
        )
        .bind(order_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(row.map(|r| Shipment {
            id: r.get::<i64, _>(0),
            order_id: r.get::<i64, _>(1),
            tracking_number: r.get::<String, _>(2),
            carrier: r.get::<String, _>(3),
            status: r.get::<String, _>(4),
        }))
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
