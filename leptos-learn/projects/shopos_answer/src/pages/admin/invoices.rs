use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::server::invoices::{list_invoices, approve_invoice};
use crate::Invoice;

#[component]
pub fn InvoicesPage() -> impl IntoView {
    let page = RwSignal::new(1i64);
    let page_size = 20i64;
    let status_filter = RwSignal::new(Option::<String>::None);
    let params = move || (status_filter.get(), page.get(), page_size);
    let invoices = Resource::new(params, |(s, p, ps)| async move { list_invoices(s, p, ps).await });
    let refresh = RwSignal::new(0i32);
    let msg = RwSignal::new(String::new());

    let reload = move || {
        refresh.update(|v| *v += 1);
        page.set(1);
    };

    let do_approve = move |id: i64| {
        spawn_local(async move {
            match approve_invoice(id).await {
                Ok(_) => {
                    msg.set("已审批通过".into());
                    reload();
                }
                Err(e) => msg.set(format!("审批失败: {}", e)),
            }
        });
    };

    view! {
        <div class="page-container">
            <h2>"发票管理"</h2>

            {move || {
                let m = msg.get();
                if !m.is_empty() {
                    view! { <Text style="margin-bottom: 16px;">{m}</Text> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            <Suspense fallback=move || view! { <Text>"加载中..."</Text> }>
                {move || refresh.get(); invoices.get().map(|result| {
                    match result {
                        Ok(data) => view! {
                            <table class="thaw-table">
                                <thead>
                                    <tr>
                                        <th>"ID"</th>
                                        <th>"订单ID"</th>
                                        <th>"发票抬头"</th>
                                        <th>"税号"</th>
                                        <th>"金额"</th>
                                        <th>"状态"</th>
                                        <th>"操作"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {data.into_iter().map(|inv| {
                                        let status_color = match inv.status.as_str() {
                                            "pending" => "#faad14",
                                            "approved" => "#1890ff",
                                            _ => "#888",
                                        };
                                        let status_label = match inv.status.as_str() {
                                            "pending" => "待审核".to_string(),
                                            "approved" => "已通过".to_string(),
                                            _ => inv.status.clone(),
                                        };
                                        view! {
                                            <tr>
                                                <td>{inv.id}</td>
                                                <td>{inv.order_id}</td>
                                                <td>{inv.title.to_string()}</td>
                                                <td>{inv.tax_number.clone().unwrap_or("-".to_string())}</td>
                                                <td>"¥" {format!("{:.2}", inv.amount)}</td>
                                                <td><span style=format!("background: {}; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;", status_color)>{status_label}</span></td>
                                                <td>
                                                    {if inv.status == "pending" {
                                                        view! {
                                                            <Button size=ButtonSize::Small appearance=ButtonAppearance::Primary on_click=move |_| do_approve(inv.id)>"审批通过"</Button>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span>"-"</span> }.into_any()
                                                    }}
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
