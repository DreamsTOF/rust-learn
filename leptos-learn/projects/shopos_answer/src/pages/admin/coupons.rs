use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::server::coupons::{list_coupons, create_coupon, update_coupon, delete_coupon};
use crate::Coupon;

#[component]
pub fn CouponsPage() -> impl IntoView {
    let page = RwSignal::new(1i64);
    let page_size = 20i64;
    let params = move || (page.get(), page_size);
    let coupons = Resource::new(params, |(p, ps)| async move { list_coupons(p, ps).await });
    let show_modal = RwSignal::new(false);
    let editing_id = RwSignal::new(Option::<i64>::None);
    let form_code = RwSignal::new(String::new());
    let form_name = RwSignal::new(String::new());
    let form_discount_type = RwSignal::new(String::from("fixed"));
    let form_discount_value = RwSignal::new(0.0);
    let form_min_amount = RwSignal::new(0.0);
    let form_max_discount = RwSignal::new(Option::<f64>::None);
    let form_total_count = RwSignal::new(100);
    let form_start_time = RwSignal::new(String::new());
    let form_end_time = RwSignal::new(String::new());
    let form_status = RwSignal::new(String::from("active"));
    let saving = RwSignal::new(false);
    let msg = RwSignal::new(String::new());
    let refresh = RwSignal::new(0i32);

    let reload = move || {
        refresh.update(|v| *v += 1);
        page.set(1);
    };

    let open_create = move |_| {
        editing_id.set(None);
        form_code.set(String::new());
        form_name.set(String::new());
        form_discount_type.set("fixed".into());
        form_discount_value.set(0.0);
        form_min_amount.set(0.0);
        form_max_discount.set(None);
        form_total_count.set(100);
        form_start_time.set(String::new());
        form_end_time.set(String::new());
        form_status.set("active".into());
        msg.set(String::new());
        show_modal.set(true);
    };

    let open_edit = move |c: Coupon| {
        editing_id.set(Some(c.id));
        form_code.set(c.code.clone());
        form_name.set(c.name.clone());
        form_discount_type.set(c.discount_type.clone());
        form_discount_value.set(c.discount_value);
        form_min_amount.set(c.min_amount);
        form_max_discount.set(c.max_discount);
        form_total_count.set(c.total_count.unwrap_or(0));
        form_start_time.set(c.start_time.clone());
        form_end_time.set(c.end_time.clone());
        form_status.set(c.status.clone());
        msg.set(String::new());
        show_modal.set(true);
    };

    let save = move |_| {
        saving.set(true);
        msg.set(String::new());

        let coupon = Coupon {
            id: editing_id.get().unwrap_or(0),
            code: form_code.get(),
            name: form_name.get(),
            discount_type: form_discount_type.get(),
            discount_value: form_discount_value.get(),
            min_amount: form_min_amount.get(),
            max_discount: form_max_discount.get(),
            total_count: Some(form_total_count.get()),
            used_count: 0,
            start_time: form_start_time.get(),
            end_time: form_end_time.get(),
            status: form_status.get(),
        };
        let data = serde_json::to_string(&coupon).unwrap_or_default();
        let edit_id = editing_id.get();

        spawn_local(async move {
            let result = if let Some(id) = edit_id {
                update_coupon(id, data).await.map(|_| ())
            } else {
                create_coupon(data).await.map(|_| ())
            };
            saving.set(false);
            match result {
                Ok(()) => {
                    show_modal.set(false);
                    reload();
                }
                Err(e) => msg.set(format!("保存失败: {}", e)),
            }
        });
    };

    let do_delete = move |id: i64| {
        spawn_local(async move {
            if delete_coupon(id).await.is_ok() {
                reload();
            }
        });
    };

    let status_label = |status: &str| match status {
        "active" => "启用",
        _ => "停用",
    };

    view! {
        <div class="page-container">
            <div style="margin-bottom: 16px;"><Space align=SpaceAlign::Center>
                <h2>"优惠券管理"</h2>
                <Button appearance=ButtonAppearance::Primary on_click=open_create>"新增优惠券"</Button>
            </Space>
        </div>

            <Suspense fallback=move || view! { <Text>"加载中..."</Text> }>
                {move || refresh.get(); coupons.get().map(|result| {
                    match result {
                        Ok(data) => view! {
                            <table class="thaw-table">
                                <thead>
                                    <tr>
                                        <th>"代码"</th>
                                        <th>"名称"</th>
                                        <th>"类型"</th>
                                        <th>"面值"</th>
                                        <th>"最低消费"</th>
                                        <th>"使用次数"</th>
                                        <th>"有效期"</th>
                                        <th>"状态"</th>
                                        <th>"操作"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {data.into_iter().map(|c| {
                                        let cc = c.clone();
                                        view! {
                                            <tr>
                                                <td>{c.code.to_string()}</td>
                                                <td>{c.name.to_string()}</td>
                                                <td>{if c.discount_type == "percentage" { "百分比" } else { "固定金额" }}</td>
                                                <td>{if c.discount_type == "percentage" { format!("{}%", c.discount_value) } else { format!("¥{:.2}", c.discount_value) }}</td>
                                                <td>"¥" {format!("{:.2}", c.min_amount)}</td>
                                                <td>{c.used_count} " / " {c.total_count.map(|n| n.to_string()).unwrap_or_else(|| "不限".into())}</td>
                                                <td>{c.start_time.to_string()} " ~ " {c.end_time.to_string()}</td>
                                                <td>
                                                    <span style=format!("background: {}; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;", if c.status == "active" { "#52c41a" } else { "#888" })>
                                                        {status_label(&c.status)}
                                                    </span>
                                                </td>
                                                <td>
                                                    <Space>
                                                        <Button size=ButtonSize::Small on_click=move |_| open_edit(cc.clone())>"编辑"</Button>
                                                        <Button size=ButtonSize::Small on_click=move |_| do_delete(c.id)>"删除"</Button>
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

            <Dialog open=show_modal>
                <DialogTitle>{move || if editing_id.get().is_some() { "编辑优惠券" } else { "新增优惠券" }}</DialogTitle>
                <Space vertical=true>
                    <p>"代码"</p>
                    <input type="text" value=move || form_code.get() on:input=move |ev| form_code.set(event_target_value(&ev)) style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;" />
                    <p>"名称"</p>
                    <input type="text" value=move || form_name.get() on:input=move |ev| form_name.set(event_target_value(&ev)) style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;" />
                    <p>"类型"</p>
                    <select prop:value=move || form_discount_type.get() on:change=move |ev| form_discount_type.set(event_target_value(&ev)) style="padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;">
                        <option value="fixed">"固定金额"</option>
                        <option value="percentage">"百分比"</option>
                    </select>
                    <p>"面值"</p>
                    <input type="number" value=move || form_discount_value.get().to_string() on:input=move |ev| { if let Ok(n) = event_target_value(&ev).parse::<f64>() { form_discount_value.set(n); } } style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;" />
                    <p>"最低消费"</p>
                    <input type="number" value=move || form_min_amount.get().to_string() on:input=move |ev| { if let Ok(n) = event_target_value(&ev).parse::<f64>() { form_min_amount.set(n); } } style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;" />
                    <p>"最大使用次数"</p>
                    <input type="number" value=move || form_total_count.get().to_string() on:input=move |ev| { if let Ok(n) = event_target_value(&ev).parse::<i32>() { form_total_count.set(n); } } style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;" />
                    <p>"开始日期"</p>
                    <input type="text" placeholder="YYYY-MM-DD" value=move || form_start_time.get() on:input=move |ev| form_start_time.set(event_target_value(&ev)) style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;" />
                    <p>"结束日期"</p>
                    <input type="text" placeholder="YYYY-MM-DD" value=move || form_end_time.get() on:input=move |ev| form_end_time.set(event_target_value(&ev)) style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;" />
                    {move || { let m = msg.get(); if !m.is_empty() { view! { <Text>{m}</Text> }.into_any() } else { view! { <span></span> }.into_any() } }}
                    <Button appearance=ButtonAppearance::Primary on_click=save disabled=move || saving.get()>
                        {move || if saving.get() { "保存中..." } else { "保存" }}
                    </Button>
                </Space>
            </Dialog>
        </div>
    }
}
