use leptos::prelude::*;
use thaw::*;

use crate::server::dashboard::get_dashboard_stats;
use crate::DashboardStats;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let stats = Resource::new(|| (), |_| async { get_dashboard_stats().await });

    view! {
        <div class="page-container">
            <h2>"仪表盘"</h2>
            <Suspense fallback=move || view! { <Text>"加载中..."</Text> }>
                {move || stats.get().map(|result| {
                    match result {
                        Ok(data) => view! {
                            <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; margin-bottom: 24px;">
                                <Card>
                                    <div style="text-align: center;">
                                        <div style="font-size: 24px; font-weight: bold; color: #1890ff;">{data.today_orders.to_string()}</div>
                                        <div style="font-size: 14px; color: #888;">"今日订单"</div>
                                    </div>
                                </Card>
                                <Card>
                                    <div style="text-align: center;">
                                        <div style="font-size: 24px; font-weight: bold; color: #52c41a;">{format!("¥{:.2}", data.today_revenue)}</div>
                                        <div style="font-size: 14px; color: #888;">"今日收入"</div>
                                    </div>
                                </Card>
                                <Card>
                                    <div style="text-align: center;">
                                        <div style="font-size: 24px; font-weight: bold; color: #722ed1;">{data.total_products.to_string()}</div>
                                        <div style="font-size: 14px; color: #888;">"商品总数"</div>
                                    </div>
                                </Card>
                                <Card>
                                    <div style="text-align: center;">
                                        <div style="font-size: 24px; font-weight: bold; color: #fa8c16;">{data.total_users.to_string()}</div>
                                        <div style="font-size: 14px; color: #888;">"用户总数"</div>
                                    </div>
                                </Card>
                                <Card>
                                    <div style="text-align: center;">
                                        <div style="font-size: 24px; font-weight: bold; color: #ff4d4f;">{data.pending_returns.to_string()}</div>
                                        <div style="font-size: 14px; color: #888;">"待处理退货"</div>
                                    </div>
                                </Card>
                            </div>
                            <Card>
                                <h3>"近7天收入趋势"</h3>
                                <table class="thaw-table">
                                    <thead>
                                        <tr>
                                            <th>"日期"</th>
                                            <th>"订单数"</th>
                                            <th>"收入"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {data.chart_data.iter().map(|cp| {
                                            view! {
                                                <tr>
                                                    <td>{cp.day.to_string()}</td>
                                                    <td>{cp.orders}</td>
                                                    <td>"¥" {format!("{:.2}", cp.revenue)}</td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            </Card>
                        }.into_any(),
                        Err(e) => view! { <Text>"加载失败: " {e.to_string()}</Text> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
