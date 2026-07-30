use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::server::audit::list_audit_logs;

#[component]
pub fn AuditPage() -> impl IntoView {
    let action_filter = RwSignal::new(Option::<String>::None);
    let user_id_filter = RwSignal::new(Option::<i64>::None);
    let page = RwSignal::new(1i64);
    let page_size = 20i64;

    let params = move || (action_filter.get(), user_id_filter.get(), page.get(), page_size);
    let logs = Resource::new(params, |(a, u, p, ps)| async move { list_audit_logs(a, u, p, ps).await });

    let total_count = RwSignal::new(0i64);
    let total_pages = move || ((total_count.get() as f64 / page_size as f64).ceil() as i64).max(1);

    view! {
        <div class="page-container">
            <h2>"审计日志"</h2>
            <div style="margin-bottom: 16px;">
                <Card>
                    <Space>
                        <div>
                            <p>"操作类型"</p>
                            <input
                                type="text"
                                placeholder="操作类型"
                                prop:value=move || action_filter.get().unwrap_or_default()
                                on:input=move |ev| {
                                    let v = event_target_value(&ev);
                                    action_filter.set(if v.is_empty() { None } else { Some(v) });
                                }
                            />
                        </div>
                    </Space>
                </Card>
            </div>

            <Suspense fallback=move || view! { <p>"加载中..."</p> }>
                {move || logs.get().map(|result| {
                    match result {
                        Ok(data) => {
                            total_count.set(data.len() as i64);
                            view! {
                                <table class="thaw-table">
                                    <thead>
                                        <tr>
                                            <th>"ID"</th>
                                            <th>"用户"</th>
                                            <th>"操作"</th>
                                            <th>"资源"</th>
                                            <th>"详情"</th>
                                            <th>"IP"</th>
                                            <th>"时间"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {data.iter().map(|log| {
                                            view! {
                                                <tr>
                                                    <td>{log.id.to_string()}</td>
                                                    <td>{log.user_id.map(|id| id.to_string()).unwrap_or_default()}</td>
                                                    <td><span style="background: #888; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;">{log.action.to_string()}</span></td>
                                                    <td>{log.resource.to_string()}</td>
                                                    <td>{log.detail.as_deref().unwrap_or("-")}</td>
                                                    <td>{log.ip_address.as_deref().unwrap_or("-")}</td>
                                                    <td>{log.created_at.to_string()}</td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                                <div style="margin-top: 16px; display: flex; gap: 8px; justify-content: center; align-items: center;">
                                    <button
                                        disabled=move || page.get() <= 1
                                        on:click=move |_| page.update(|p| *p -= 1)
                                    >"上一页"</button>
                                    <span>"第 " {move || page.get()} " 页，共 " {move || total_pages()} " 页"</span>
                                    <button
                                        disabled=move || page.get() >= total_pages()
                                        on:click=move |_| page.update(|p| *p += 1)
                                    >"下一页"</button>
                                </div>
                            }.into_any()
                        },
                        Err(e) => view! { <p>"加载失败: " {e.to_string()}</p> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
