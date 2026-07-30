use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::server::reports::export_report;

#[component]
pub fn ReportsPage() -> impl IntoView {
    let report_type = RwSignal::new(String::from("orders"));
    let date_from = RwSignal::new(String::new());
    let date_to = RwSignal::new(String::new());
    let export_format = RwSignal::new(String::from("csv"));
    let exporting = RwSignal::new(false);
    let msg = RwSignal::new(String::new());
    let download_url = RwSignal::new(String::new());

    let do_export = move |_| {
        if date_from.get().is_empty() || date_to.get().is_empty() {
            msg.set("请选择日期范围".into());
            return;
        }
        exporting.set(true);
        msg.set(String::new());
        download_url.set(String::new());
        let rt = report_type.get();
        let df = date_from.get();
        let dt = date_to.get();
        let fmt = export_format.get();

        spawn_local(async move {
            match export_report(rt, Some(df), Some(dt), fmt).await {
                Ok(url) => {
                    download_url.set(url);
                    msg.set("导出成功".into());
                }
                Err(e) => msg.set(format!("导出失败: {}", e)),
            }
            exporting.set(false);
        });
    };

    view! {
        <div class="page-container">
            <h2>"数据报表"</h2>
            <Card>
                <Space vertical=true>
                    <p>"报表类型"</p>
                    <select
                        prop:value=move || report_type.get()
                        on:change=move |ev| report_type.set(event_target_value(&ev))
                        style="padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                    >
                        <option value="orders">"订单报表"</option>
                        <option value="products">"商品报表"</option>
                        <option value="users">"用户报表"</option>
                        <option value="revenue">"财务报表"</option>
                    </select>
                    <Space>
                        <Input
                            placeholder="开始日期 YYYY-MM-DD"
                            value=date_from
                        />
                        <Input
                            placeholder="结束日期 YYYY-MM-DD"
                            value=date_to
                        />
                    </Space>
                    <p>"导出格式"</p>
                    <select
                        prop:value=move || export_format.get()
                        on:change=move |ev| export_format.set(event_target_value(&ev))
                        style="padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                    >
                        <option value="csv">"CSV"</option>
                        <option value="json">"JSON"</option>
                    </select>

                    {move || {
                        let m = msg.get();
                        if !m.is_empty() {
                            view! { <Text>{m}</Text> }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}

                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=do_export
                        disabled=move || exporting.get()
                    >
                        {move || if exporting.get() { "导出中..." } else { "导出报表" }}
                    </Button>

                    {move || {
                        let url = download_url.get();
                        if url.is_empty() {
                            view! { <span></span> }.into_any()
                        } else {
                            view! {
                                <a href=url target="_blank">
                                    <Button>"下载文件"</Button>
                                </a>
                            }.into_any()
                        }
                    }}
                </Space>
            </Card>
        </div>
    }
}
