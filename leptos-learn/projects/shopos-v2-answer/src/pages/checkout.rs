use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use thaw::*;

use crate::server::addresses::list_addresses;
use crate::server::coupons::validate_coupon;
use crate::server::orders::create_order;

#[component]
pub fn CheckoutPage() -> impl IntoView {
    let addresses = Resource::new(|| (), |_| async { list_addresses(0).await });
    let selected_address_id = RwSignal::new(Option::<i64>::None);
    let coupon_code = RwSignal::new(String::new());
    let coupon_discount = RwSignal::new(0.0);
    let coupon_msg = RwSignal::new(String::new());
    let submitting = RwSignal::new(false);
    let submit_result = RwSignal::new(Option::<String>::None);

    // Get cart items from local storage
    let cart_items = get_checkout_items();
    let cart_items_for_checkout = cart_items.clone();
    let subtotal = cart_items.iter().map(|i| i.price * i.quantity as f64).sum::<f64>();

    let apply_coupon = move |_| {
        let code = coupon_code.get();
        if code.is_empty() {
            coupon_msg.set("请输入优惠券代码".into());
            return;
        }
        spawn_local(async move {
            match validate_coupon(code.clone(), subtotal).await {
                Ok(_resp) => {
                    coupon_msg.set("优惠券已应用".into());
                }
                Err(e) => {
                    coupon_discount.set(0.0);
                    coupon_msg.set(format!("优惠券无效: {}", e));
                }
            }
        });
    };

    let pay_amount = move || (subtotal - coupon_discount.get()).max(0.0);

    let do_checkout = move |_| {
        let address_id = match selected_address_id.get() {
            Some(id) => id,
            None => {
                submit_result.set(Some("请选择收货地址".into()));
                return;
            }
        };
        if cart_items_for_checkout.is_empty() {
            submit_result.set(Some("购物车为空".into()));
            return;
        }
        submitting.set(true);
        let items_json = serde_json::to_string(&cart_items_for_checkout.iter().map(|i| {
            serde_json::json!({
                "product_id": i.product_id,
                "quantity": i.quantity,
            })
        }).collect::<Vec<_>>()).unwrap_or_default();
        let coupon = coupon_code.get();
        let coupon_opt = if coupon.is_empty() { None } else { Some(coupon) };
        submit_result.set(None);

        spawn_local(async move {
            let result = create_order(address_id, coupon_opt, items_json, 0).await;
            submitting.set(false);
            match result {
                Ok(order_id) => {
                    clear_cart_storage();
                    let _ = leptos_router::hooks::use_navigate()(
                        &format!("/admin/orders/{}", order_id),
                        Default::default(),
                    );
                }
                Err(e) => {
                    submit_result.set(Some(format!("下单失败: {}", e)));
                }
            }
        });
    };

    view! {
        <div class="page-container">
            <h2>"结算"</h2>
            <Card>
                <Suspense fallback=move || view! { <Text>"加载地址..."</Text> }>
                    {move || addresses.get().map(|result| {
                        match result {
                            Ok(addrs) => {
                                let selected = selected_address_id;
                                view! {
                                    <Space vertical=true>
                                        {addrs.into_iter().map(|addr| {
                                            let is_selected = move || selected.get() == Some(addr.id);
                                            let a = addr.clone();
                                            view! {
                                                <div
                                                    class="address-item"
                                                    style="padding:8px;border:1px solid #e8e8e8;border-radius:6px;margin-bottom:8px;cursor:pointer"
                                                    class:selected=is_selected
                                                    on:click=move |_| selected.set(Some(a.id))
                                                >
                                                    <strong>{a.receiver_name.to_string()}</strong> " " {a.phone.to_string()}
                                                    <br/>
                                                    {format!("{}{}{}{}", a.province, a.city, a.district, a.detail)}
                                                </div>
                                            }
                                        }).collect_view()}
                                    </Space>
                                }.into_any()
                            }
                            Err(e) => view! { <Text>"加载失败: " {e.to_string()}</Text> }.into_any(),
                        }
                    })}
                </Suspense>
                <A href="/user/addresses">
                    <Button>"管理地址"</Button>
                </A>
            </Card>

            <div style="margin-top: 16px;"><Card>
                {cart_items.iter().map(|item| {
                    view! {
                        <div style="display:flex;justify-content:space-between;padding:8px 0;border-bottom:1px solid #f0f0f0">
                            <span>{item.product_name.to_string()}</span>
                            <span>"¥" {format!("{:.2}", item.price)} " x " {item.quantity}</span>
                        </div>
                    }
                }).collect_view()}
            </Card>
        </div>

            <Card>
                <h3>"优惠券"</h3>
                <Space>
                    <Input
                        placeholder="输入优惠券代码"
                        value=coupon_code
                    />
                    <Button on_click=apply_coupon>"验证"</Button>
                </Space>
                <p>{move || coupon_msg.get()}</p>
            </Card>

            <div style="margin-top: 16px; text-align: right;"><Card>
                <Space vertical=true>
                    <p>"商品合计: ¥" {format!("{:.2}", subtotal)}</p>
                    <p>"优惠金额: -¥" {move || format!("{:.2}", coupon_discount.get())}</p>
                    <p><strong>"应付金额: ¥" {move || format!("{:.2}", pay_amount())}</strong></p>
                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=do_checkout
                        disabled=move || submitting.get()
                    >
                        {move || if submitting.get() { "提交中..." } else { "提交订单" }}
                    </Button>
                    {move || submit_result.get().map(|e| view! { <Text>"错误: " {e}</Text> })}
                </Space>
            </Card>
        </div>
    </div>
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CheckoutItem {
    product_id: i64,
    product_name: String,
    price: f64,
    quantity: i32,
}

fn get_checkout_items() -> Vec<CheckoutItem> {
    let storage = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten();
    if let Some(storage) = storage {
        if let Ok(Some(data)) = storage.get_item("shopos_cart") {
            if let Ok(items) = serde_json::from_str::<Vec<CheckoutItem>>(&data) {
                return items;
            }
        }
    }
    Vec::new()
}

fn clear_cart_storage() {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
        let _ = storage.remove_item("shopos_cart");
    }
}
