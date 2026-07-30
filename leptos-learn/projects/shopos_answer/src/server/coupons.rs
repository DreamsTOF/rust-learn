use leptos::prelude::*;

use crate::Coupon;

#[server(ListCoupons)]
pub async fn list_coupons(page: i64, page_size: i64) -> Result<Vec<Coupon>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let offset = (page - 1) * page_size;
        let rows = sqlx::query(
            "SELECT id, code, name, discount_type, discount_value, min_amount, max_discount, \
             total_count, used_count, start_time, end_time, status \
             FROM coupons ORDER BY id DESC LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(rows
            .iter()
            .map(|row| Coupon {
                id: row.get::<i64, _>(0),
                code: row.get::<String, _>(1),
                name: row.get::<String, _>(2),
                discount_type: row.get::<String, _>(3),
                discount_value: row.get::<f64, _>(4),
                min_amount: row.get::<f64, _>(5),
                max_discount: row.get::<Option<f64>, _>(6),
                total_count: row.get::<Option<i32>, _>(7),
                used_count: row.get::<i32, _>(8),
                start_time: row.get::<String, _>(9),
                end_time: row.get::<String, _>(10),
                status: row.get::<String, _>(11),
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(CreateCoupon)]
pub async fn create_coupon(data: String) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let coupon: Coupon = serde_json::from_str(&data)
            .map_err(|e| ServerFnError::new(format!("Invalid coupon data: {}", e)))?;

        let result = sqlx::query(
            "INSERT INTO coupons (code, name, discount_type, discount_value, min_amount, max_discount, total_count, start_time, end_time, status) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&coupon.code)
        .bind(&coupon.name)
        .bind(&coupon.discount_type)
        .bind(coupon.discount_value)
        .bind(coupon.min_amount)
        .bind(coupon.max_discount)
        .bind(coupon.total_count)
        .bind(&coupon.start_time)
        .bind(&coupon.end_time)
        .bind(&coupon.status)
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

#[server(UpdateCoupon)]
pub async fn update_coupon(id: i64, data: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let coupon: Coupon = serde_json::from_str(&data)
            .map_err(|e| ServerFnError::new(format!("Invalid coupon data: {}", e)))?;

        let affected = sqlx::query(
            "UPDATE coupons SET name = ?, discount_type = ?, discount_value = ?, min_amount = ?, max_discount = ?, total_count = ?, start_time = ?, end_time = ?, status = ? WHERE id = ?",
        )
        .bind(&coupon.name)
        .bind(&coupon.discount_type)
        .bind(coupon.discount_value)
        .bind(coupon.min_amount)
        .bind(coupon.max_discount)
        .bind(coupon.total_count)
        .bind(&coupon.start_time)
        .bind(&coupon.end_time)
        .bind(&coupon.status)
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

#[server(DeleteCoupon)]
pub async fn delete_coupon(id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let affected = sqlx::query("DELETE FROM coupons WHERE id = ?")
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

#[server(ValidateCoupon)]
pub async fn validate_coupon(
    code: String,
    order_amount: f64,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let row = sqlx::query(
            "SELECT discount_type, discount_value, min_amount, max_discount, used_count, total_count \
             FROM coupons WHERE code = ? AND status = 'active' AND start_time <= datetime('now') AND end_time >= datetime('now')",
        )
        .bind(&code)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("优惠券无效或已过期"))?;

        let discount_type: String = row.get(0);
        let discount_value: f64 = row.get(1);
        let min_amount: f64 = row.get(2);
        let max_discount: Option<f64> = row.get(3);
        let used_count: i32 = row.get(4);
        let total_count: Option<i32> = row.get(5);

        if total_count.is_some() && used_count >= total_count.unwrap() {
            return Err(ServerFnError::new("优惠券已被领完"));
        }

        if order_amount < min_amount {
            return Err(ServerFnError::new(format!(
                "订单金额不足 {:.2}",
                min_amount
            )));
        }

        let discount = if discount_type == "percentage" {
            let d = order_amount * discount_value / 100.0;
            match max_discount {
                Some(max) => d.min(max),
                None => d,
            }
        } else {
            discount_value
        };

        let result = serde_json::json!({
            "valid": true,
            "discount_type": discount_type,
            "discount_value": discount_value,
            "discount": discount,
            "description": format!("优惠 {:.2} 元", discount),
        });

        Ok(result.to_string())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
