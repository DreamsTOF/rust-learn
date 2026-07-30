use leptos::prelude::*;

use crate::{ChartPoint, DashboardStats};

#[server(GetDashboardStats)]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    // TODO: Implement this server function
    // Hint: Query today's and weekly order counts/revenue, total products/users, pending returns, and chart data for the last 7 days
    unimplemented!()
}
