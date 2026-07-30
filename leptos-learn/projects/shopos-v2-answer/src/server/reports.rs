use leptos::prelude::*;

#[server(ExportReport)]
pub async fn export_report(
    report_type: String,
    date_from: Option<String>,
    date_to: Option<String>,
    format: String,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let from = date_from.unwrap_or_else(|| "1970-01-01".to_string());
        let to = date_to.unwrap_or_else(|| "2099-12-31".to_string());

        match report_type.as_str() {
            "orders" => export_orders(&pool, &from, &to, &format).await,
            "products" => export_products(&pool, &format).await,
            "users" => export_users(&pool, &format).await,
            "revenue" => export_revenue(&pool, &from, &to, &format).await,
            _ => Err(ServerFnError::new(format!(
                "不支持的报表类型: {}",
                report_type
            ))),
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[cfg(feature = "ssr")]
async fn export_orders(
    pool: &sqlx::SqlitePool,
    from: &str,
    to: &str,
    format: &str,
) -> Result<String, ServerFnError> {
    use sqlx::Row;

    let rows = sqlx::query(
        "SELECT o.id, o.order_no, o.status, o.total_amount, o.discount_amount, o.actual_amount, \
                u.username, o.created_at \
         FROM orders o LEFT JOIN users u ON o.user_id = u.id \
         WHERE date(o.created_at) >= ? AND date(o.created_at) <= ? \
         ORDER BY o.id DESC",
    )
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    if format == "csv" {
        let mut csv = String::from("ID,订单号,状态,总金额,优惠金额,实付金额,用户名,创建时间\n");
        for row in &rows {
            csv.push_str(&format!(
                "{},{},{},{:.2},{:.2},{:.2},{},{}\n",
                row.get::<i64, _>(0),
                row.get::<String, _>(1),
                row.get::<String, _>(2),
                row.get::<f64, _>(3),
                row.get::<f64, _>(4),
                row.get::<f64, _>(5),
                row.get::<String, _>(6),
                row.get::<String, _>(7),
            ));
        }
        Ok(csv)
    } else {
        // JSON
        let items: Vec<serde_json::Value> = rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "id": row.get::<i64, _>(0),
                    "order_no": row.get::<String, _>(1),
                    "status": row.get::<String, _>(2),
                    "total_amount": row.get::<f64, _>(3),
                    "discount_amount": row.get::<f64, _>(4),
                    "actual_amount": row.get::<f64, _>(5),
                    "username": row.get::<String, _>(6),
                    "created_at": row.get::<String, _>(7),
                })
            })
            .collect();
        serde_json::to_string(&items).map_err(|e| ServerFnError::new(e.to_string()))
    }
}

#[cfg(feature = "ssr")]
async fn export_products(pool: &sqlx::SqlitePool, format: &str) -> Result<String, ServerFnError> {
    use sqlx::Row;

    let rows = sqlx::query(
        "SELECT p.id, p.name, p.price, p.stock, p.status, COALESCE(c.name, '') AS category_name \
         FROM products p LEFT JOIN categories c ON p.category_id = c.id ORDER BY p.id DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    if format == "csv" {
        let mut csv = String::from("ID,名称,价格,库存,状态,分类\n");
        for row in &rows {
            csv.push_str(&format!(
                "{},{},{:.2},{},{},{}\n",
                row.get::<i64, _>(0),
                row.get::<String, _>(1),
                row.get::<f64, _>(2),
                row.get::<i32, _>(3),
                row.get::<String, _>(4),
                row.get::<String, _>(5),
            ));
        }
        Ok(csv)
    } else {
        let items: Vec<serde_json::Value> = rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "id": row.get::<i64, _>(0),
                    "name": row.get::<String, _>(1),
                    "price": row.get::<f64, _>(2),
                    "stock": row.get::<i32, _>(3),
                    "status": row.get::<String, _>(4),
                    "category": row.get::<String, _>(5),
                })
            })
            .collect();
        serde_json::to_string(&items).map_err(|e| ServerFnError::new(e.to_string()))
    }
}

#[cfg(feature = "ssr")]
async fn export_users(pool: &sqlx::SqlitePool, format: &str) -> Result<String, ServerFnError> {
    use sqlx::Row;

    let rows = sqlx::query(
        "SELECT id, username, email, role, created_at FROM users ORDER BY id DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    if format == "csv" {
        let mut csv = String::from("ID,用户名,邮箱,角色,注册时间\n");
        for row in &rows {
            csv.push_str(&format!(
                "{},{},{},{},{}\n",
                row.get::<i64, _>(0),
                row.get::<String, _>(1),
                row.get::<String, _>(2),
                row.get::<String, _>(3),
                row.get::<String, _>(4),
            ));
        }
        Ok(csv)
    } else {
        let items: Vec<serde_json::Value> = rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "id": row.get::<i64, _>(0),
                    "username": row.get::<String, _>(1),
                    "email": row.get::<String, _>(2),
                    "role": row.get::<String, _>(3),
                    "created_at": row.get::<String, _>(4),
                })
            })
            .collect();
        serde_json::to_string(&items).map_err(|e| ServerFnError::new(e.to_string()))
    }
}

#[cfg(feature = "ssr")]
async fn export_revenue(
    pool: &sqlx::SqlitePool,
    from: &str,
    to: &str,
    format: &str,
) -> Result<String, ServerFnError> {
    use sqlx::Row;

    let rows = sqlx::query(
        "SELECT date(created_at) AS day, COUNT(*) AS orders, COALESCE(SUM(actual_amount), 0) AS revenue \
         FROM orders WHERE date(created_at) >= ? AND date(created_at) <= ? \
         GROUP BY date(created_at) ORDER BY day",
    )
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    if format == "csv" {
        let mut csv = String::from("日期,订单数,收入\n");
        for row in &rows {
            csv.push_str(&format!(
                "{},{},{:.2}\n",
                row.get::<String, _>(0),
                row.get::<i64, _>(1),
                row.get::<f64, _>(2),
            ));
        }
        Ok(csv)
    } else {
        let items: Vec<serde_json::Value> = rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "day": row.get::<String, _>(0),
                    "orders": row.get::<i64, _>(1),
                    "revenue": row.get::<f64, _>(2),
                })
            })
            .collect();
        serde_json::to_string(&items).map_err(|e| ServerFnError::new(e.to_string()))
    }
}
