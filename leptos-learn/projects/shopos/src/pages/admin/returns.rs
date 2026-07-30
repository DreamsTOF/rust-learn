use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::server::returns::{list_returns, review_return, process_refund};

#[component]
pub fn ReturnsPage() -> impl IntoView {
    let page = RwSignal::new(1i64);
    let page_size = 20i64;
    let status_filter = RwSignal::new(Option::<String>::None);
    let params = move || (status_filter.get(), page.get(), page_size);
    let returns = Resource::new(params, |(s, p, ps)| async move { list_returns(s, p, ps).await });
    let refresh = RwSignal::new(0i32);
    let reload = move || {
        refresh.update(|v| *v += 1);
        page.set(1);
    };

    let do_approve = move |id: i64| {
        spawn_local(async move {
            if review_return(id, true, Some("审核通过".into())).await.is_ok() {
                reload();
            }
        });
    };

    let do_reject = move |id: i64| {
        spawn_local(async move {
            if review_return(id, false, Some("审核拒绝".into())).await.is_ok() {
                reload();
            }
        });
    };

    let do_refund = move |id: i64| {
        spawn_local(async move {
            if process_refund(id).await.is_ok() {
                reload();
            }
        });
    };

    view! {
        <div class="page-container">
            <h2>"售后管理"</h2>

            <Suspense fallback=move || view! { <Text>"加载中..."</Text> }>
                {move || refresh.get(); returns.get().map(|result| {
                    match result {
                        Ok(data) => view! {
                            <table class="thaw-table">
                                <thead>
                                    <tr>
                                        <th>"ID"</th>
                                        <th>"订单ID"</th>
                                        <th>"原因"</th>
                                        <th>"金额"</th>
                                        <th>"状态"</th>
                                        <th>"操作"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {data.into_iter().map(|r| {
                                        let status_color = match r.status.as_str() {
                                            "pending_review" => "#faad14",
                                            "approved" => "#52c41a",
                                            "rejected" => "#ff4d4f",
                                            "refunded" => "#1890ff",
                                            _ => "#888",
                                        };
                                        let status_label = match r.status.as_str() {
                                            "pending_review" => "待审核".to_string(),
                                            "approved" => "已通过".to_string(),
                                            "rejected" => "已拒绝".to_string(),
                                            "refunded" => "已退款".to_string(),
                                            _ => r.status.clone(),
                                        };
                                        view! {
                                            <tr>
                                                <td>{r.id}</td>
                                                <td>{r.order_id}</td>
                                                <td>{r.reason.to_string()}</td>
                                                <td>"¥" {format!("{:.2}", r.refund_amount)}</td>
                                                <td><span style=format!("background: {}; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;", status_color)>{status_label}</span></td>
                                                <td>
                                                    <Space>
                                                        {if r.status == "pending_review" {
                                                            view! {
                                                                <Button size=ButtonSize::Small appearance=ButtonAppearance::Primary on_click=move |_| do_approve(r.id)>"通过"</Button>
                                                                <Button size=ButtonSize::Small on_click=move |_| do_reject(r.id)>"拒绝"</Button>
                                                            }.into_any()
                                                        } else if r.status == "approved" {
                                                            view! {
                                                                <Button size=ButtonSize::Small on_click=move |_| do_refund(r.id)>"退款"</Button>
                                                            }.into_any()
                                                        } else {
                                                            view! { <span>"-"</span> }.into_any()
                                                        }}
                                                    </Space>
                                                </td>
                                            </tr>
                                        }
                                    }).collect_view()}
                                </tbody>
                            </table>
                        }.into_any(),
                        Err(e) => view! { <Text>"加载失败: " {e.to_string()}</Text> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
