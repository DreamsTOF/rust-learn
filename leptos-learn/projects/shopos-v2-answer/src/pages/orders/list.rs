use leptos::prelude::*;
use leptos_router::components::A;
use thaw::*;

use crate::server::orders::list_orders;

#[component]
pub fn OrderListPage() -> impl IntoView {
    let status_filter = RwSignal::new(String::new());
    let page = RwSignal::new(1);
    let page_size = 15i64;

    let params = move || {
        (
            if status_filter.get().is_empty() { None } else { Some(status_filter.get()) },
            page.get() as i64,
            page_size,
        )
    };

    let orders = Resource::new(params, |(status, p, ps)| async move { list_orders(status, p, ps, 0i64).await }); // TODO: use actual user_id

    let total_pages = move || {
        orders.get().map(|result| {
            result.map(|data| ((data.total as f64) / page_size as f64).ceil() as i64).unwrap_or(1)
        }).unwrap_or(1)
    };

    let tabs = vec![
        ("", "全部"),
        ("pending_payment", "待付款"),
        ("paid", "待发货"),
        ("shipped", "待收货"),
        ("completed", "已完成"),
        ("cancelled", "已取消"),
    ];

    let set_status = move |s: &str| {
        status_filter.set(s.to_string());
        page.set(1);
    };

    view! {
        <div class="page-container">
            <h2>"订单列表"</h2>
            <div style="margin-bottom: 16px;"><Space>
                {tabs.iter().map(|(val, label)| {
                    let val = val.to_string();
                    let label = label.to_string();
                    let val2 = val.clone();
                    let is_active = move || status_filter.get() == val;
                    view! {
                        <Button
                            appearance={if is_active() { ButtonAppearance::Primary } else { ButtonAppearance::Secondary }}
                            on_click=move |_| set_status(&val2)
                        >
                            {label}
                        </Button>
                    }
                }).collect_view()}
            </Space>
        </div>

            <Suspense fallback=move || view! { <Text>"加载中..."</Text> }>
                {move || orders.get().map(|result| {
                    match result {
                        Ok(data) => view! {
                            <table class="thaw-table">
                                <thead>
                                    <tr>
                                        <th>"订单号"</th>
                                        <th>"金额"</th>
                                        <th>"状态"</th>
                                        <th>"下单时间"</th>
                                        <th>"操作"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {data.items.iter().map(|order| {
                                        let status_color = match order.status.as_str() {
                                            "pending_payment" => "#faad14",
                                            "paid" => "#1890ff",
                                            "shipped" => "#52c41a",
                                            "completed" => "#888",
                                            "cancelled" => "#ff4d4f",
                                            _ => "#888",
                                        };
                                        let status_label = match order.status.as_str() {
                                            "pending_payment" => "待付款",
                                            "paid" => "待发货",
                                            "shipped" => "待收货",
                                            "completed" => "已完成",
                                            "cancelled" => "已取消",
                                            _ => &order.status,
                                        };
                                        view! {
                                            <tr>
                                                <td>{order.order_no.to_string()}</td>
                                                <td>"¥" {format!("{:.2}", order.actual_amount)}</td>
                                                <td>
                                                    <span style=format!("background: {}; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;", status_color)>{status_label}</span>
                                                </td>
                                                <td>{order.created_at.to_string()}</td>
                                                <td>
                                                    <A href=format!("/admin/orders/{}", order.id)>
                                                        <Button size=ButtonSize::Small>"详情"</Button>
                                                    </A>
                                                </td>
                                            </tr>
                                        }
                                    }).collect_view()}
                                </tbody>
                            </table>
                            <div style="margin-top: 16px; justify-content: center;"><Space>
                                <Button
                                    disabled=Signal::derive(move || page.get() <= 1)
                                    on_click=move |_| page.update(|p| *p -= 1)
                                >"上一页"</Button>
                                <span>"第 " {move || page.get()} " / " {move || total_pages()} " 页"</span>
                                <Button
                                    disabled=Signal::derive(move || page.get() >= total_pages())
                                    on_click=move |_| page.update(|p| *p += 1)
                                >"下一页"</Button>
                            </Space>
                        </div>
                        }.into_any(),
                        Err(e) => view! { <Text>"加载失败: " {e.to_string()}</Text> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
