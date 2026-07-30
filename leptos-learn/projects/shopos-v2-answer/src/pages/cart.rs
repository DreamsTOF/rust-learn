use leptos::prelude::*;
use leptos_router::components::A;
use thaw::*;

#[component]
pub fn CartPage() -> impl IntoView {
    let items = RwSignal::new(
        get_cart_items()
    );
    let total = Memo::new(move |_| {
        items.with(|items| items.iter().map(|i| i.price * i.quantity as f64).sum::<f64>())
    });

    let remove_item = move |sku_id: i64| {
        items.update(|items| items.retain(|i| i.sku_id != sku_id));
        save_cart_items(items.get_untracked());
    };

    let update_qty = move |sku_id: i64, qty: i32| {
        items.update(|items| {
            if let Some(item) = items.iter_mut().find(|i| i.sku_id == sku_id) {
                item.quantity = qty.max(1);
            }
        });
        save_cart_items(items.get_untracked());
    };

    let clear_cart = move |_| {
        items.set(Vec::new());
        set_cart_storage(Vec::new());
    };

    view! {
        <div class="page-container">
            <h2>"购物车"</h2>
            <Table>
                <thead>
                    <tr>
                        <th>"商品"</th>
                        <th>"单价"</th>
                        <th>"数量"</th>
                        <th>"小计"</th>
                        <th>"操作"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || items.get().into_iter().map(|item| {
                        let sku_id = item.sku_id;
                        view! {
                            <tr>
                                <td>{item.product_name} " - " {item.sku_name}</td>
                                <td>"¥" {format!("{:.2}", item.price)}</td>
                                <td>
                                    <Space>
                                        <Button size=ButtonSize::Small on_click=move |_| update_qty(sku_id, items.get().iter().find(|i| i.sku_id == sku_id).map_or(1, |i| i.quantity - 1))>"-"</Button>
                                        <span>{item.quantity}</span>
                                        <Button size=ButtonSize::Small on_click=move |_| update_qty(sku_id, items.get().iter().find(|i| i.sku_id == sku_id).map_or(1, |i| i.quantity + 1))>"+"</Button>
                                    </Space>
                                </td>
                                <td>"¥" {format!("{:.2}", item.price * item.quantity as f64)}</td>
                                <td>
                                    <Button appearance=ButtonAppearance::Primary size=ButtonSize::Small on_click=move |_| remove_item(sku_id)>"删除"</Button>
                                </td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </Table>
            <div style="margin-top: 16px; text-align: right;">
                <Space>
                    <span>"合计: ¥" {move || format!("{:.2}", total.get())}</span>
                    <Button on_click=clear_cart>"清空购物车"</Button>
                    <A href="/checkout">
                        <Button appearance=ButtonAppearance::Primary>"去结算"</Button>
                    </A>
                </Space>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CartItem {
    sku_id: i64,
    product_id: i64,
    product_name: String,
    sku_name: String,
    price: f64,
    quantity: i32,
    image_url: Option<String>,
}

fn get_cart_items() -> Vec<CartItem> {
    let storage = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten();
    if let Some(storage) = storage {
        if let Ok(Some(data)) = storage.get_item("shopos_cart") {
            if let Ok(items) = serde_json::from_str::<Vec<CartItem>>(&data) {
                return items;
            }
        }
    }
    Vec::new()
}

fn save_cart_items(items: Vec<CartItem>) {
    let storage = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten();
    if let Some(storage) = storage {
        if let Ok(data) = serde_json::to_string(&items) {
            let _ = storage.set_item("shopos_cart", &data);
        }
    }
}

fn set_cart_storage(items: Vec<CartItem>) {
    let storage = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten();
    if let Some(storage) = storage {
        let _ = storage.remove_item("shopos_cart");
    }
}
