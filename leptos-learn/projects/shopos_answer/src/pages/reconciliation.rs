use leptos::prelude::*;
use thaw::*;

#[component]
pub fn ReconciliationPage() -> impl IntoView {
    let records = RwSignal::new(vec![
        ReconRecord { id: 1, order_no: "202607280001".into(), amount: 299.00, pay_method: "微信支付".into(), pay_time: "2026-07-28 10:30".into(), status: "已匹配".into() },
        ReconRecord { id: 2, order_no: "202607280002".into(), amount: 159.00, pay_method: "支付宝".into(), pay_time: "2026-07-28 11:00".into(), status: "已匹配".into() },
        ReconRecord { id: 3, order_no: "202607280003".into(), amount: 599.00, pay_method: "微信支付".into(), pay_time: "2026-07-28 14:20".into(), status: "异常".into() },
    ]);

    let mark_anomaly = move |id: i64| {
        records.update(|r| {
            if let Some(rec) = r.iter_mut().find(|r| r.id == id) {
                rec.status = "异常".into();
            }
        });
    };

    view! {
        <div class="page-container">
            <h2>"支付对账"</h2>
            <table class="thaw-table">
                <thead>
                    <tr>
                        <th>"订单号"</th>
                        <th>"金额"</th>
                        <th>"支付方式"</th>
                        <th>"支付时间"</th>
                        <th>"状态"</th>
                        <th>"操作"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || records.get().iter().map(|rec| {
                        let rid = rec.id;
                        let status_style = if rec.status == "已匹配" { "background: #52c41a; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;" } else { "background: #ff4d4f; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;" };
                        view! {
                            <tr>
                                <td>{rec.order_no.to_string()}</td>
                                <td>"¥" {format!("{:.2}", rec.amount)}</td>
                                <td>{rec.pay_method.to_string()}</td>
                                <td>{rec.pay_time.to_string()}</td>
                                <td>
                                    <span style=status_style>{rec.status.to_string()}</span>
                                </td>
                                <td>
                                    {if rec.status == "已匹配" {
                                        view! {
                                            <Button size=ButtonSize::Small on_click=move |_| mark_anomaly(rid)>"标记异常"</Button>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }}
                                </td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
        </div>
    }
}

#[derive(Debug, Clone)]
struct ReconRecord {
    id: i64,
    order_no: String,
    amount: f64,
    pay_method: String,
    pay_time: String,
    status: String,
}
