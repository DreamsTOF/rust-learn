use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use thaw::*;

use crate::server::products::get_product_detail;

#[component]
pub fn ProductDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params
            .get()
            .get("id")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0)
    };

    let product = Resource::new(id, |id| async move { get_product_detail(id).await });

    let images = move || {
        product.get().and_then(|r| r.ok()).and_then(|p| {
            p.image_urls.as_ref().map(|urls| {
                urls.split('\n')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
            })
        }).unwrap_or_default()
    };

    let product_content = move || {
        let imgs = images();
        product.get().map(|result| {
            match result {
                Ok(data) => {
                    let cat_id = data.category_id.map(|id| id.to_string()).unwrap_or_else(|| "-".to_string());
                    let desc = data.description.clone().unwrap_or_else(|| "-".to_string());
                    let status_str = if data.status == "active" { "上架".to_string() } else { "下架".to_string() };

                    view! {
                    <div>
                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 24px;">
                            <Card>
                                <h3>"商品图片"</h3>
                                <div style="display: flex; flex-wrap: wrap; gap: 8px;">
                                    {if imgs.is_empty() {
                                        view! { <div style="width: 200px; height: 200px; background: #f0f0f0; display: flex; align-items: center; justify-content: center;">"暂无图片"</div> }.into_any()
                                    } else {
                                        view! {
                                            {imgs.iter().map(|url| {
                                                view! {
                                                    <img
                                                        src=url
                                                        alt="商品图片"
                                                        style="width: 200px; height: 200px; object-fit: cover; border-radius: 4px;"
                                                    />
                                                }
                                            }).collect_view()}
                                        }.into_any()
                                    }}
                                </div>
                            </Card>
                            <Card>
                                <h3>"商品信息"</h3>
                                <div style="display: flex; flex-direction: column; gap: 8px;">
                                    <p><strong>"名称: "</strong>{data.name.to_string()}</p>
                                    <p><strong>"描述: "</strong>{desc}</p>
                                    <p><strong>"分类ID: "</strong>{cat_id}</p>
                                    <p><strong>"价格: "</strong>"¥" {format!("{:.2}", data.price)}</p>
                                    <p><strong>"库存: "</strong>{data.stock}</p>
                                    <p>
                                        <strong>"状态: "</strong>
                                        <span style=if data.status == "active" { "background: #52c41a; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;" } else { "background: #888; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;" }>
                                            {status_str}
                                        </span>
                                    </p>
                                </div>
                            </Card>
                        </div>

                        <div style="margin-top: 24px;">
                            <Card>
                                <h3>"SKU 列表"</h3>
                                <table class="thaw-table">
                                    <thead>
                                        <tr>
                                            <th>"SKU代码"</th>
                                            <th>"规格名"</th>
                                            <th>"规格值"</th>
                                            <th>"价格"</th>
                                            <th>"库存"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {data.skus.iter().map(|sku| {
                                            let spec_name = sku.spec_name.clone().unwrap_or_else(|| "-".to_string());
                                            let spec_value = sku.spec_value.clone().unwrap_or_else(|| "-".to_string());
                                            view! {
                                                <tr>
                                                    <td>{sku.sku_code.to_string()}</td>
                                                    <td>{spec_name}</td>
                                                    <td>{spec_value}</td>
                                                    <td>"¥" {format!("{:.2}", sku.price.unwrap_or(data.price))}</td>
                                                    <td>{sku.stock}</td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            </Card>
                        </div>
                    </div>
                }.into_any()
            },
                Err(e) => view! {
                    <p>"加载失败: " {e.to_string()}</p>
                }.into_any(),
            }
        })
    };

    view! {
        <div class="page-container">
            <div style="display: flex; margin-bottom: 16px;">
                <A href="/admin/products">
                    <Button>"返回列表"</Button>
                </A>
            </div>

            <Suspense fallback=move || view! { <p>"加载中..."</p> }>
                {product_content}
            </Suspense>
        </div>
    }
}
