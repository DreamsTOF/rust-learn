use leptos::prelude::*;
use leptos::task::spawn_local;
use thaw::*;

use crate::server::addresses::{list_addresses, create_address, update_address, delete_address, set_default_address};
use crate::Address;

#[component]
pub fn AddressesPage() -> impl IntoView {
    let addresses = Resource::new(|| (), |_| async { list_addresses(0i64).await }); // TODO: use actual user_id
    let show_modal = RwSignal::new(false);
    let editing_id = RwSignal::new(Option::<i64>::None);
    let form_name = RwSignal::new(String::new());
    let form_phone = RwSignal::new(String::new());
    let form_province = RwSignal::new(String::new());
    let form_city = RwSignal::new(String::new());
    let form_district = RwSignal::new(String::new());
    let form_detail = RwSignal::new(String::new());
    let form_default = RwSignal::new(false);
    let saving = RwSignal::new(false);
    let msg = RwSignal::new(String::new());

    let refresh = RwSignal::new(0i32);
    let reload = move || {
        refresh.update(|v| *v += 1);
    };

    let open_add = move |_| {
        editing_id.set(None);
        form_name.set(String::new());
        form_phone.set(String::new());
        form_province.set(String::new());
        form_city.set(String::new());
        form_district.set(String::new());
        form_detail.set(String::new());
        form_default.set(false);
        msg.set(String::new());
        show_modal.set(true);
    };

    let open_edit = move |addr: Address| {
        editing_id.set(Some(addr.id));
        form_name.set(addr.receiver_name);
        form_phone.set(addr.phone);
        form_province.set(addr.province);
        form_city.set(addr.city);
        form_district.set(addr.district);
        form_detail.set(addr.detail);
        form_default.set(addr.is_default);
        msg.set(String::new());
        show_modal.set(true);
    };

    let save_addr = move |_| {
        if form_name.get().is_empty() || form_phone.get().is_empty() {
            msg.set("请填写必要字段".into());
            return;
        }
        saving.set(true);
        msg.set(String::new());
        let name = form_name.get();
        let phone = form_phone.get();
        let province = form_province.get();
        let city = form_city.get();
        let district = form_district.get();
        let detail = form_detail.get();
        let default = form_default.get();
        let edit_id = editing_id.get();

        spawn_local(async move {
            let result = if let Some(id) = edit_id {
                let addr_data = serde_json::json!({
                    "receiver_name": name,
                    "phone": phone,
                    "province": province,
                    "city": city,
                    "district": district,
                    "detail": detail,
                    "is_default": default,
                });
                update_address(id, addr_data.to_string(), 0i64).await // TODO: use actual user_id
            } else {
                create_address(name, phone, province, city, district, detail, default, 0i64) // TODO: use actual user_id
                    .await
                    .map(|_| true)
            };
            saving.set(false);
            match result {
                Ok(_) => {
                    show_modal.set(false);
                    reload();
                }
                Err(e) => msg.set(format!("保存失败: {}", e)),
            }
        });
    };

    let do_delete = move |id: i64| {
        spawn_local(async move {
            if delete_address(id, 0i64).await.is_ok() {
                reload();
            }
        });
    };

    let do_set_default = move |id: i64| {
        spawn_local(async move {
            if set_default_address(id, 0i64).await.is_ok() { // TODO: use actual user_id
                reload();
            }
        });
    };

    view! {
        <div class="page-container">
            <div style="margin-bottom: 16px;"><Space align=SpaceAlign::Center>
                <h2>"地址管理"</h2>
                <Button appearance=ButtonAppearance::Primary on_click=open_add>"新增地址"</Button>
            </Space>
        </div>

            <Suspense fallback=move || view! { <Text>"加载中..."</Text> }>
                {move || refresh.get(); addresses.get().map(|result| {
                    match result {
                        Ok(addrs) => view! {
                            <Space vertical=true>
                                {addrs.into_iter().map(|addr| {
                                    let a = addr.clone();
                                    view! {
                                        <Card>
                                            <Space vertical=true>
                                                <Space align=SpaceAlign::Center>
                                                    <strong>{addr.receiver_name.to_string()}</strong>
                                                    <span>{addr.phone.to_string()}</span>
                                                    {if addr.is_default {
                                                        view! { <span style="background: #52c41a; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;">"默认"</span> }.into_any()
                                                    } else {
                                                        view! { <span></span> }.into_any()
                                                    }}
                                                </Space>
                                                <p>{format!("{}{}{}{}", addr.province, addr.city, addr.district, addr.detail)}</p>
                                                <Space>
                                                    <Button size=ButtonSize::Small on_click=move |_| open_edit(a.clone())>"编辑"</Button>
                                                    {if !addr.is_default {
                                                        view! {
                                                            <Button size=ButtonSize::Small on_click=move |_| do_set_default(addr.id)>"设为默认"</Button>
                                                            <Button size=ButtonSize::Small on_click=move |_| do_delete(addr.id)>"删除"</Button>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span></span> }.into_any()
                                                    }}
                                                </Space>
                                            </Space>
                                        </Card>
                                    }
                                }).collect_view()}
                            </Space>
                        }.into_any(),
                        Err(e) => view! { <Text>"加载失败: " {e.to_string()}</Text> }.into_any(),
                    }
                })}
            </Suspense>

            <Dialog open=show_modal>
                <DialogTitle>
                    {move || if editing_id.get().is_some() { "编辑地址" } else { "新增地址" }}
                </DialogTitle>
                <Space vertical=true>
                    <p>"姓名"</p>
                    <input
                        type="text"
                        value=move || form_name.get()
                        on:input=move |ev| form_name.set(event_target_value(&ev))
                        style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                    />
                    <p>"电话"</p>
                    <input
                        type="text"
                        value=move || form_phone.get()
                        on:input=move |ev| form_phone.set(event_target_value(&ev))
                        style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                    />
                    <Space>
                        <div>
                            <p>"省份"</p>
                            <input
                                type="text"
                                value=move || form_province.get()
                                on:input=move |ev| form_province.set(event_target_value(&ev))
                                style="padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                            />
                        </div>
                        <div>
                            <p>"城市"</p>
                            <input
                                type="text"
                                value=move || form_city.get()
                                on:input=move |ev| form_city.set(event_target_value(&ev))
                                style="padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                            />
                        </div>
                        <div>
                            <p>"区/县"</p>
                            <input
                                type="text"
                                value=move || form_district.get()
                                on:input=move |ev| form_district.set(event_target_value(&ev))
                                style="padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                            />
                        </div>
                    </Space>
                    <p>"详细地址"</p>
                    <input
                        type="text"
                        value=move || form_detail.get()
                        on:input=move |ev| form_detail.set(event_target_value(&ev))
                        style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                    />
                    <label>
                        <input
                            type="checkbox"
                            prop:checked=move || form_default.get()
                            on:change=move |_| form_default.update(|v| *v = !*v)
                        />
                        " 设为默认地址"
                    </label>
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
                        on_click=save_addr
                        disabled=move || saving.get()
                    >
                        {move || if saving.get() { "保存中..." } else { "保存" }}
                    </Button>
                </Space>
            </Dialog>
        </div>
    }
}
