use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: i64,
    pub sku_code: String,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    pub image_url: String,
}

pub enum CartAction {
    Add(CartItem),
    Remove(String),
    UpdateQuantity(String, i32),
    Clear,
}

const STORAGE_KEY: &str = "shopos_cart";

fn load_cart() -> Vec<CartItem> {
    let storage = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten();
    if let Some(storage) = storage {
        if let Ok(Some(data)) = storage.get_item(STORAGE_KEY) {
            if let Ok(items) = serde_json::from_str::<Vec<CartItem>>(&data) {
                return items;
            }
        }
    }
    Vec::new()
}

fn save_cart(items: &[CartItem]) {
    let storage = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten();
    if let Some(storage) = storage {
        if let Ok(data) = serde_json::to_string(items) {
            let _ = storage.set_item(STORAGE_KEY, &data);
        }
    }
}

pub fn use_cart() -> (
    ReadSignal<Vec<CartItem>>,
    impl Fn(CartAction) + Clone,
    Memo<i32>,
    Memo<f64>,
) {
    let (cart, set_cart) = signal(load_cart());

    let total_count = Memo::new(move |_| {
        cart.get().iter().map(|item| item.quantity).sum::<i32>()
    });

    let total_price = Memo::new(move |_| {
        cart.get()
            .iter()
            .map(|item| item.price * item.quantity as f64)
            .sum::<f64>()
    });

    let dispatch = move |action: CartAction| match action {
        CartAction::Add(item) => {
            set_cart.update(|items| {
                if let Some(existing) = items
                    .iter_mut()
                    .find(|i: &&mut CartItem| i.sku_code == item.sku_code)
                {
                    existing.quantity += item.quantity;
                } else {
                    items.push(item);
                }
            });
            save_cart(&cart.get_untracked());
        }
        CartAction::Remove(sku_code) => {
            set_cart
                .update(|items| items.retain(|i: &CartItem| i.sku_code != sku_code));
            save_cart(&cart.get_untracked());
        }
        CartAction::UpdateQuantity(sku_code, quantity) => {
            if quantity <= 0 {
                set_cart
                    .update(|items| items.retain(|i: &CartItem| i.sku_code != sku_code));
            } else {
                set_cart.update(|items| {
                    if let Some(item) = items
                        .iter_mut()
                        .find(|i: &&mut CartItem| i.sku_code == sku_code)
                    {
                        item.quantity = quantity;
                    }
                });
            }
            save_cart(&cart.get_untracked());
        }
        CartAction::Clear => {
            set_cart.set(Vec::new());
            let storage = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten();
            if let Some(storage) = storage {
                let _ = storage.remove_item(STORAGE_KEY);
            }
        }
    };

    (cart, dispatch, total_count, total_price)
}
