use leptos::prelude::*;
use thaw::*;

#[derive(Debug, Clone)]
pub struct ProductFormData {
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub price: f64,
    pub image_urls: Option<String>,
    pub status: String,
}

#[component]
pub fn ProductForm(
    #[prop(optional)] initial_data: Option<ProductFormData>,
    #[prop(optional)] on_submit: Option<Box<dyn Fn(ProductFormData) + Send + Sync>>,
    #[prop(optional)] submit_label: Option<String>,
) -> impl IntoView {
    let name = RwSignal::new(
        initial_data
            .as_ref()
            .map(|d| d.name.clone())
            .unwrap_or_default(),
    );
    let description = RwSignal::new(
        initial_data
            .as_ref()
            .and_then(|d| d.description.clone())
            .unwrap_or_default(),
    );
    let category_id = RwSignal::new(initial_data.as_ref().and_then(|d| d.category_id));
    let price = RwSignal::new(initial_data.as_ref().map(|d| d.price).unwrap_or(0.0));
    let image_urls = RwSignal::new(
        initial_data
            .as_ref()
            .and_then(|d| d.image_urls.clone())
            .unwrap_or_default(),
    );
    let status = RwSignal::new(
        initial_data
            .as_ref()
            .map(|d| d.status.clone())
            .unwrap_or("draft".into()),
    );

    let name_error = RwSignal::new(Option::<String>::None);
    let price_error = RwSignal::new(Option::<String>::None);

    let selected_category = RwSignal::new(
        initial_data
            .as_ref()
            .and_then(|d| d.category_id)
            .map(|id| id.to_string())
            .unwrap_or_default(),
    );

    let submit = move |_| {
        let mut valid = true;

        if name.get().trim().is_empty() {
            name_error.set(Some("商品名称不能为空".into()));
            valid = false;
        } else {
            name_error.set(None);
        }

        if price.get() <= 0.0 {
            price_error.set(Some("价格必须大于0".into()));
            valid = false;
        } else {
            price_error.set(None);
        }

        if !valid {
            return;
        }

        if let Some(ref cb) = on_submit {
            let cat_id = selected_category
                .get()
                .parse::<i64>()
                .ok()
                .filter(|&id| id > 0);

            cb(ProductFormData {
                name: name.get().trim().to_string(),
                description: {
                    let d = description.get();
                    if d.trim().is_empty() {
                        None
                    } else {
                        Some(d.trim().to_string())
                    }
                },
                category_id: cat_id,
                price: price.get(),
                image_urls: {
                    let u = image_urls.get();
                    if u.trim().is_empty() {
                        None
                    } else {
                        Some(u.trim().to_string())
                    }
                },
                status: status.get(),
            });
        }
    };

    view! {
        <div class="product-form">
            <div class="product-form-field">
                <label>"商品名称"</label>
                <Input
                    value=name
                />
                {move || {
                    name_error.get().map(|e| view! {
                        <div class="form-field-error">{e}</div>
                    })
                }}
            </div>

            <div class="product-form-field">
                <label>"商品分类"</label>
                <select
                    prop:value=move || selected_category.get()
                    on:change=move |ev| {
                        let v = event_target_value(&ev);
                        selected_category.set(v.clone());
                        category_id.set(v.parse::<i64>().ok());
                    }
                    style="padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                >
                    <option value="">"请选择分类"</option>
                    <option value="1">"电子产品"</option>
                    <option value="2">"服装鞋帽"</option>
                    <option value="3">"食品饮料"</option>
                    <option value="4">"家居用品"</option>
                    <option value="5">"图书音像"</option>
                    <option value="6">"运动户外"</option>
                    <option value="7">"美妆个护"</option>
                    <option value="8">"母婴用品"</option>
                </select>
            </div>

            <div class="product-form-field">
                <label>"商品描述"</label>
                <textarea
                    prop:value=move || description.get()
                    on:input=move |ev| description.set(event_target_value(&ev))
                    style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px; min-height: 80px;"
                ></textarea>
            </div>

            <div class="product-form-field">
                <label>"价格"</label>
                <input
                    type="number"
                    value=move || price.get().to_string()
                    on:input=move |ev| price.set(event_target_value(&ev).parse::<f64>().unwrap_or(0.0))
                    style="padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                />
                {move || {
                    price_error.get().map(|e| view! {
                        <div class="form-field-error">{e}</div>
                    })
                }}
            </div>

            <div class="product-form-field">
                <label>"图片URL（每行一张）"</label>
                <textarea
                    prop:value=move || image_urls.get()
                    on:input=move |ev| image_urls.set(event_target_value(&ev))
                    style="width: 100%; padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px; min-height: 80px;"
                ></textarea>
            </div>

            <div class="product-form-field">
                <label>"状态"</label>
                <select
                    prop:value=move || status.get()
                    on:change=move |ev| status.set(event_target_value(&ev))
                    style="padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                >
                    <option value="draft">"草稿"</option>
                    <option value="published">"已上架"</option>
                    <option value="archived">"已归档"</option>
                </select>
            </div>

            <div class="product-form-actions">
                <Button on_click=submit>
                    {move || submit_label.clone().unwrap_or_else(|| "保存".into())}
                </Button>
            </div>
        </div>
    }
}
