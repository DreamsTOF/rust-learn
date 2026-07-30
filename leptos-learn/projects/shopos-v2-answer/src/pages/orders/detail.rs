use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use thaw::*;

use crate::server::orders::{cancel_order, get_order_detail, update_order_status};
use crate::server::shipments::ship_order;

#[component]
pub fn OrderDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params
            .get()
            .get("id")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0)
    };

    let order = Resource::new(id, |id| async move { get_order_detail(id, 0i64).await });
    let refresh = RwSignal::new(0i32);
    let msg = RwSignal::new(String::new());

    let reload = move || {
        refresh.update(|v| *v += 1);
    };

    let do_ship = move |_| {
        let oid = id();
        spawn_local(async move {
            match ship_order(oid, "SF123456789".into(), "快递".into()).await {
                Ok(_) => {
                    msg.set("已发货".into());
                    reload();
                }
                Err(e) => msg.set(format!("发货失败: {}", e)),
            }
        });
    };

    let do_complete = move |_| {
        let oid = id();
        spawn_local(async move {
            match update_order_status(oid, "completed".into(), 0i64).await {
                Ok(_) => {
                    msg.set("已完成".into());
                    reload();
                }
                Err(e) => msg.set(format!("操作失败: {}", e)),
            }
        });
    };

    let do_cancel = move |_| {
        let oid = id();
        spawn_local(async move {
            match cancel_order(oid, 0i64).await {
                Ok(_) => {
                    msg.set("已取消".into());
                    reload();
                }
                Err(e) => msg.set(format!("取消失败: {}", e)),
            }
        });
    };

    let msg_display = move || {
        let m = msg.get();
        if !m.is_empty() {
            view! { <p>"消息: " {m}</p> }.into_any()
        } else {
            view! { <span></span> }.into_any()
        }
    };

    let order_content = move || {
        order.get().map(|result| {
            match result {
                Ok(data) => {
                    let status = data.status.clone();
                    let status_label = match status.as_str() {
                        "pending_payment" => "待付款",
                        "paid" => "待发货",
                        "shipped" => "待收货",
                        "received" => "已收货",
                        "completed" => "已完成",
                        "cancelled" => "已取消",
                        _ => &status,
                    };
                    let status_color = match status.as_str() {
                        "pending_payment" => "#faad14",
                        "paid" => "#1890ff",
                        "shipped" => "#52c41a",
                        "completed" => "#888",
                        "cancelled" => "#ff4d4f",
                        _ => "#888",
                    };

                    view! {
                        <div>
                            <div style="border: 1px solid #e8e8e8; border-radius: 8px; padding: 16px; margin-bottom: 16px;">
                                <h3>"订单信息"</h3>
                                <div style="display: flex; flex-direction: column; gap: 8px;">
                                    <p><strong>"订单号: "</strong>{data.order_no.to_string()}</p>
                                    <p><strong>"状态: "</strong><span style=format!("background: {}; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;", status_color)>{status_label}</span></p>
                                    <p><strong>"商品总额: "</strong>"¥" {format!("{:.2}", data.total_amount)}</p>
                                    <p><strong>"优惠金额: "</strong>"¥" {format!("{:.2}", data.discount_amount)}</p>
                                    <p><strong>"实付金额: "</strong>"¥" {format!("{:.2}", data.actual_amount)}</p>
                                    <p><strong>"下单时间: "</strong>{data.created_at.to_string()}</p>
                                </div>
                            </div>

                            <div style="border: 1px solid #e8e8e8; border-radius: 8px; padding: 16px; margin-bottom: 16px;">
                                <h3>"商品列表"</h3>
                                <table class="thaw-table">
                                    <thead>
                                        <tr>
                                            <th>"商品"</th>
                                            <th>"SKU"</th>
                                            <th>"单价"</th>
                                            <th>"数量"</th>
                                            <th>"小计"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {data.items.iter().map(|item| {
                                            view! {
                                                <tr>
                                                    <td>{item.product_name.to_string()}</td>
                                                    <td>{item.sku_code.as_deref().unwrap_or("-")}</td>
                                                    <td>"¥" {format!("{:.2}", item.price)}</td>
                                                    <td>{item.quantity}</td>
                                                    <td>"¥" {format!("{:.2}", item.price * item.quantity as f64)}</td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            </div>

                            {data.receiver_name.as_ref().map(|_| {
                                view! {
                                    <div style="border: 1px solid #e8e8e8; border-radius: 8px; padding: 16px; margin-bottom: 16px;">
                                        <h3>"收货地址"</h3>
                                        <p>{data.receiver_name.as_deref().unwrap_or("")} " " {data.phone.as_deref().unwrap_or("")}</p>
                                        <p>{format!("{}{}{}{}", data.province.as_deref().unwrap_or(""), data.city.as_deref().unwrap_or(""), data.district.as_deref().unwrap_or(""), data.detail.as_deref().unwrap_or(""))}</p>
                                    </div>
                                }
                            })}

                            <div style="border: 1px solid #e8e8e8; border-radius: 8px; padding: 16px;">
                                <h3>"操作"</h3>
                                <div style="display: flex; gap: 8px;">
                                    {if status == "paid" {
                                        view! { <button on:click=do_ship style="padding: 4px 12px; border: 1px solid #1890ff; border-radius: 4px; background: #1890ff; color: white; cursor: pointer;">"发货"</button> }.into_any()
                                    } else if status == "shipped" {
                                        view! { <button on:click=do_complete style="padding: 4px 12px; border: 1px solid #1890ff; border-radius: 4px; background: #1890ff; color: white; cursor: pointer;">"确认收货"</button> }.into_any()
                                    } else { view! { <span></span> }.into_any() }}

                                    {if status == "pending_payment" || status == "paid" {
                                        view! { <button on:click=do_cancel style="padding: 4px 12px; border: 1px solid #ff4d4f; border-radius: 4px; color: #ff4d4f; cursor: pointer;">"取消订单"</button> }.into_any()
                                    } else { view! { <span></span> }.into_any() }}
                                </div>
                            </div>
                        </div>
                    }.into_any()
                },
                Err(e) => view! {
                    <div><p>"加载失败: " {e.to_string()}</p></div>
                }.into_any(),
            }
        })
    };

    view! {
        <div class="page-container">
            <div style="display: flex; margin-bottom: 16px;">
                <A href="/admin/orders">
                    <Button>"返回列表"</Button>
                </A>
            </div>

            <div>{msg_display}</div>

            <Suspense fallback=move || view! { <p>"加载中..."</p> }>
                {order_content}
            </Suspense>
        </div>
    }
}
