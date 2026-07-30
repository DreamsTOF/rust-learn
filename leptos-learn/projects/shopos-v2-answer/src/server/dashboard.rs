use leptos::prelude::*;

use crate::{ChartPoint, DashboardStats};

#[server(GetDashboardStats)]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // Today's orders and revenue
        let today = sqlx::query(
            "SELECT COUNT(*), COALESCE(SUM(actual_amount), 0) FROM orders WHERE date(created_at) = date('now')",
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let today_orders: i64 = today.get(0);
        let today_revenue: f64 = today.get(1);

        // This week's orders and revenue (last 7 days including today)
        let week = sqlx::query(
            "SELECT COUNT(*), COALESCE(SUM(actual_amount), 0) FROM orders WHERE created_at >= datetime('now', '-7 days')",
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let week_orders: i64 = week.get(0);
        let week_revenue: f64 = week.get(1);

        // Total products
        let total_products: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM products")
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Total users
        let total_users: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM users")
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Pending returns
        let pending_returns: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM returns WHERE status = 'pending_review'")
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Chart data: last 7 days
        let chart_rows = sqlx::query(
            "SELECT date(created_at) AS day, COUNT(*) AS orders, COALESCE(SUM(actual_amount), 0) AS revenue \
             FROM orders WHERE created_at >= datetime('now', '-7 days') \
             GROUP BY date(created_at) ORDER BY day",
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let mut chart_data: Vec<ChartPoint> = chart_rows
            .iter()
            .map(|row| ChartPoint {
                day: row.get::<String, _>(0),
                orders: row.get::<i64, _>(1),
                revenue: row.get::<f64, _>(2),
            })
            .collect();

        // Fill in missing days
        let mut filled = Vec::new();
        for i in (0..7).rev() {
            let date_str = chrono::Local::now()
                .checked_sub_signed(chrono::Duration::days(i))
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();

            let existing = chart_data.iter().find(|c| c.day == date_str);
            match existing {
                Some(point) => filled.push(point.clone()),
                None => filled.push(ChartPoint {
                    day: date_str,
                    orders: 0,
                    revenue: 0.0,
                }),
            }
        }
        chart_data = filled;

        Ok(DashboardStats {
            today_orders,
            today_revenue,
            week_orders,
            week_revenue,
            total_products,
            total_users,
            pending_returns,
            chart_data,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
