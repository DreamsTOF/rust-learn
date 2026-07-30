use leptos::prelude::*;

use crate::{Order, OrderDetail, OrderItem, OrderListResponse};

#[server(CreateOrder)]
pub async fn create_order(
    address_id: i64,
    coupon_code: Option<String>,
    cart_items: String,
    user_id: i64,
) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // Parse cart items: JSON array of {product_id, sku_code?, quantity}
        #[derive(serde::Deserialize)]
        struct CartItem {
            product_id: i64,
            #[serde(default)]
            sku_code: Option<String>,
            quantity: i32,
        }

        let items: Vec<CartItem> = serde_json::from_str(&cart_items)
            .map_err(|e| ServerFnError::new(format!("Invalid cart data: {}", e)))?;

        if items.is_empty() {
            return Err(ServerFnError::new("购物车为空"));
        }

        // Use a transaction
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Calculate total amount and validate stock
        let mut total_amount = 0.0f64;
        for item in &items {
            // Check product exists and get price
            let prod = sqlx::query("SELECT price, stock, name FROM products WHERE id = ?")
                .bind(item.product_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?
                .ok_or_else(|| ServerFnError::new(format!("商品 {} 不存在", item.product_id)))?;

            let price: f64 = prod.get(0);
            let stock: i32 = prod.get(1);
            let _name: String = prod.get(2);

            if stock < item.quantity {
                return Err(ServerFnError::new(format!(
                    "商品 {} 库存不足 (库存: {}, 需要: {})",
                    item.product_id, stock, item.quantity
                )));
            }

            total_amount += price * item.quantity as f64;

            // Deduct stock
            let updated = sqlx::query("UPDATE products SET stock = stock - ? WHERE id = ? AND stock >= ?")
                .bind(item.quantity)
                .bind(item.product_id)
                .bind(item.quantity)
                .execute(&mut *tx)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?
                .rows_affected();

            if updated == 0 {
                return Err(ServerFnError::new(format!(
                    "商品 {} 库存不足",
                    item.product_id
                )));
            }
        }

        // Apply coupon if provided
        let mut discount_amount = 0.0f64;
        if let Some(ref code) = coupon_code {
            let coupon_row = sqlx::query(
                "SELECT discount_type, discount_value, min_amount, max_discount, used_count, total_count \
                 FROM coupons WHERE code = ? AND status = 'active' AND start_time <= datetime('now') AND end_time >= datetime('now')",
            )
            .bind(code)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .ok_or_else(|| ServerFnError::new("优惠券无效或已过期"))?;

            let discount_type: String = coupon_row.get(0);
            let discount_value: f64 = coupon_row.get(1);
            let min_amount: f64 = coupon_row.get(2);
            let max_discount: Option<f64> = coupon_row.get(3);
            let used_count: i32 = coupon_row.get(4);
            let total_count: Option<i32> = coupon_row.get(5);

            if total_count.is_some() && used_count >= total_count.unwrap() {
                return Err(ServerFnError::new("优惠券已被领完"));
            }

            if total_amount < min_amount {
                return Err(ServerFnError::new(format!(
                    "订单金额不足 {:.2}，无法使用该优惠券",
                    min_amount
                )));
            }

            discount_amount = if discount_type == "percentage" {
                let d = total_amount * discount_value / 100.0;
                match max_discount {
                    Some(max) => d.min(max),
                    None => d,
                }
            } else {
                // fixed
                discount_value
            };

            // Increment used_count
            sqlx::query("UPDATE coupons SET used_count = used_count + 1 WHERE code = ?")
                .bind(code)
                .execute(&mut *tx)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }

        let actual_amount = (total_amount - discount_amount).max(0.0);

        // Generate order_no
        let order_no = format!(
            "ORD{}{}",
            chrono::Local::now().format("%Y%m%d%H%M%S"),
            &uuid::Uuid::new_v4().to_string()[..6].to_uppercase()
        );

        // Insert order
        let order_result = sqlx::query(
            "INSERT INTO orders (user_id, order_no, status, total_amount, discount_amount, actual_amount, address_id) \
             VALUES (?, ?, 'pending_payment', ?, ?, ?, ?)",
        )
        .bind(user_id)
        .bind(&order_no)
        .bind(total_amount)
        .bind(discount_amount)
        .bind(actual_amount)
        .bind(address_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let order_id = order_result.last_insert_rowid();

        // Insert order items
        for item in &items {
            let prod = sqlx::query("SELECT name, price FROM products WHERE id = ?")
                .bind(item.product_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?
                .ok_or_else(|| ServerFnError::new("商品不存在"))?;

            let product_name: String = prod.get(0);
            let price: f64 = prod.get(1);

            sqlx::query(
                "INSERT INTO order_items (order_id, product_id, sku_code, product_name, price, quantity) VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(order_id)
            .bind(item.product_id)
            .bind(&item.sku_code)
            .bind(&product_name)
            .bind(price)
            .bind(item.quantity)
            .execute(&mut *tx)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        }

        tx.commit()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(order_id)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(ListOrders)]
pub async fn list_orders(
    status: Option<String>,
    page: i64,
    page_size: i64,
    user_id: i64,
) -> Result<OrderListResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let (count_sql, data_sql, bind_status) = if let Some(ref st) = status {
            (
                "SELECT COUNT(*) FROM orders WHERE user_id = ? AND status = ?",
                "SELECT id, user_id, order_no, status, total_amount, discount_amount, actual_amount, created_at \
                 FROM orders WHERE user_id = ? AND status = ? ORDER BY id DESC LIMIT ? OFFSET ?",
                true,
            )
        } else {
            (
                "SELECT COUNT(*) FROM orders WHERE user_id = ?",
                "SELECT id, user_id, order_no, status, total_amount, discount_amount, actual_amount, created_at \
                 FROM orders WHERE user_id = ? ORDER BY id DESC LIMIT ? OFFSET ?",
                false,
            )
        };

        let total: i64 = if bind_status {
            sqlx::query_scalar(count_sql)
                .bind(user_id)
                .bind(status.as_ref().unwrap())
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?
        } else {
            sqlx::query_scalar(count_sql)
                .bind(user_id)
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?
        };

        let offset = (page - 1) * page_size;
        let rows = if bind_status {
            sqlx::query(data_sql)
                .bind(user_id)
                .bind(status.as_ref().unwrap())
                .bind(page_size)
                .bind(offset)
                .fetch_all(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?
        } else {
            sqlx::query(data_sql)
                .bind(user_id)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?
        };

        let items: Vec<Order> = rows
            .iter()
            .map(|row| Order {
                id: row.get::<i64, _>(0),
                user_id: row.get::<i64, _>(1),
                order_no: row.get::<String, _>(2),
                status: row.get::<String, _>(3),
                total_amount: row.get::<f64, _>(4),
                discount_amount: row.get::<f64, _>(5),
                actual_amount: row.get::<f64, _>(6),
                created_at: row.get::<String, _>(7),
            })
            .collect();

        Ok(OrderListResponse { items, total })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(GetOrderDetail)]
pub async fn get_order_detail(
    id: i64,
    user_id: i64,
) -> Result<OrderDetail, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let row = sqlx::query(
            "SELECT o.id, o.user_id, o.order_no, o.status, o.total_amount, o.discount_amount, o.actual_amount, o.created_at, \
                    a.receiver_name, a.phone, a.province, a.city, a.district, a.detail \
             FROM orders o LEFT JOIN addresses a ON o.address_id = a.id \
             WHERE o.id = ? AND o.user_id = ?",
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("订单不存在"))?;

        let item_rows = sqlx::query(
            "SELECT id, order_id, product_id, sku_code, product_name, price, quantity FROM order_items WHERE order_id = ?",
        )
        .bind(id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let items: Vec<OrderItem> = item_rows
            .iter()
            .map(|r| OrderItem {
                id: r.get::<i64, _>(0),
                order_id: r.get::<i64, _>(1),
                product_id: r.get::<i64, _>(2),
                sku_code: r.get::<Option<String>, _>(3),
                product_name: r.get::<String, _>(4),
                price: r.get::<f64, _>(5),
                quantity: r.get::<i32, _>(6),
            })
            .collect();

        Ok(OrderDetail {
            id: row.get::<i64, _>(0),
            user_id: row.get::<i64, _>(1),
            order_no: row.get::<String, _>(2),
            status: row.get::<String, _>(3),
            total_amount: row.get::<f64, _>(4),
            discount_amount: row.get::<f64, _>(5),
            actual_amount: row.get::<f64, _>(6),
            created_at: row.get::<String, _>(7),
            receiver_name: row.get::<Option<String>, _>(8),
            phone: row.get::<Option<String>, _>(9),
            province: row.get::<Option<String>, _>(10),
            city: row.get::<Option<String>, _>(11),
            district: row.get::<Option<String>, _>(12),
            detail: row.get::<Option<String>, _>(13),
            items,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(UpdateOrderStatus)]
pub async fn update_order_status(
    id: i64,
    new_status: String,
    user_id: i64,
) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let row = sqlx::query("SELECT status, user_id FROM orders WHERE id = ?")
            .bind(id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .ok_or_else(|| ServerFnError::new("订单不存在"))?;

        let current_status: String = row.get(0);
        let order_user_id: i64 = row.get(1);

        // Only the order owner can update
        if order_user_id != user_id {
            return Err(ServerFnError::new("无权操作此订单"));
        }

        // State machine validation
        let valid_transition = match (current_status.as_str(), new_status.as_str()) {
            ("pending_payment", "paid") => true,
            ("pending_payment", "cancelled") => true,
            ("paid", "shipped") => true,
            ("shipped", "received") => true,
            ("received", "completed") => true,
            ("received", "reviewed") => true,
            _ => false,
        };

        if !valid_transition {
            return Err(ServerFnError::new(format!(
                "不允许从 '{}' 转换到 '{}'",
                current_status, new_status
            )));
        }

        let affected = sqlx::query("UPDATE orders SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(&new_status)
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

#[server(CancelOrder)]
pub async fn cancel_order(id: i64, user_id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let row = sqlx::query("SELECT status, user_id FROM orders WHERE id = ?")
            .bind(id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .ok_or_else(|| ServerFnError::new("订单不存在"))?;

        let current_status: String = row.get(0);
        let order_user_id: i64 = row.get(1);

        if order_user_id != user_id {
            return Err(ServerFnError::new("无权操作此订单"));
        }

        // Only pending_payment orders can be cancelled
        if current_status != "pending_payment" {
            return Err(ServerFnError::new("当前订单状态不允许取消"));
        }

        // Use transaction to restore stock
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Get order items to restore stock
        let items = sqlx::query("SELECT product_id, quantity FROM order_items WHERE order_id = ?")
            .bind(id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        for item in &items {
            let product_id: i64 = item.get(0);
            let quantity: i32 = item.get(1);
            sqlx::query("UPDATE products SET stock = stock + ? WHERE id = ?")
                .bind(quantity)
                .bind(product_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }

        sqlx::query("UPDATE orders SET status = 'cancelled', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(true)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
