use leptos::prelude::*;
use leptos_router::components::A;
use thaw::*;

use crate::server::products::list_products;

#[component]
pub fn ProductListPage() -> impl IntoView {
    let keyword = RwSignal::new(String::new());
    let category_id = RwSignal::new(Option::<i64>::None);
    let price_min = RwSignal::new(Option::<f64>::None);
    let price_max = RwSignal::new(Option::<f64>::None);
    let status = RwSignal::new(Option::<String>::None);
    let page = RwSignal::new(1i64);
    let page_size = 20i64;

    let params = move || {
        (page.get(), page_size, category_id.get(), keyword.get(), status.get(), price_min.get(), price_max.get())
    };

    let products = Resource::new(params, |(p, ps, cid, kw, st, minp, maxp)| async move {
        list_products(p, ps, cid, if kw.is_empty() { None } else { Some(kw) }, st, minp, maxp).await
    });

    let total_pages = move || {
        products.get().map(|result| {
            result.map(|data| {
                let total = data.total;
                (total as f64 / page_size as f64).ceil() as i64
            }).unwrap_or(1)
        }).unwrap_or(1)
    };

    let do_search = move |_| {
        page.set(1);
    };

    view! {
        <div class="page-container">
            <div style="margin-bottom: 16px;">
                <Space align=SpaceAlign::Center>
                    <h2>"商品列表"</h2>
                    <A href="/admin/products/import">
                        <Button>"批量导入"</Button>
                    </A>
                </Space>
            </div>
            <div style="margin-bottom: 16px;">
                <Card>
                    <Space>
                        <Input
                            placeholder="搜索商品名称"
                            value=keyword
                        />
                        <Input
                            placeholder="最低价"
                            prop:value=Signal::derive(move || price_min.get().map(|v| v.to_string()).unwrap_or_default())
                        />
                        <Input
                            placeholder="最高价"
                            prop:value=Signal::derive(move || price_max.get().map(|v| v.to_string()).unwrap_or_default())
                        />
                        <select
                            prop:value=move || status.get().unwrap_or_default()
                            on:change=move |ev| {
                                let v = event_target_value(&ev);
                                status.set(if v.is_empty() { None } else { Some(v) });
                            }
                            style="padding: 4px 8px; border: 1px solid #d9d9d9; border-radius: 4px;"
                        >
                            <option value="">"全部"</option>
                            <option value="active">"上架"</option>
                            <option value="inactive">"下架"</option>
                        </select>
                        <Button appearance=ButtonAppearance::Primary on_click=do_search>"搜索"</Button>
                    </Space>
                </Card>
            </div>
            <Suspense fallback=move || view! { <Text>"加载中..."</Text> }>
                {move || products.get().map(|result| {
                    match result {
                        Ok(data) => view! {
                            <table class="thaw-table">
                                <thead>
                                    <tr>
                                        <th>"商品名称"</th>
                                        <th>"分类"</th>
                                        <th>"价格"</th>
                                        <th>"库存"</th>
                                        <th>"状态"</th>
                                        <th>"操作"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {data.items.iter().map(|item| {
                                        let id = item.id;
                                        let status_style = if item.status == "active" { "background: #52c41a; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;" } else { "background: #888; color: white; padding: 2px 8px; border-radius: 4px; font-size: 12px;" };
                                        view! {
                                            <tr>
                                                <td>{item.name.to_string()}</td>
                                                <td>{item.category_name.as_deref().unwrap_or("-")}</td>
                                                <td>"¥" {format!("{:.2}", item.price)}</td>
                                                <td>{item.stock}</td>
                                                <td>
                                                    <span style=status_style>
                                                        {if item.status == "active" { "上架" } else { "下架" }}
                                                    </span>
                                                </td>
                                                <td>
                                                    <A href=format!("/admin/products/{}", id)>
                                                        <Button size=ButtonSize::Small>"详情"</Button>
                                                    </A>
                                                </td>
                                            </tr>
                                        }
                                    }).collect_view()}
                                </tbody>
                            </table>
                            <div style="margin-top: 16px; justify-content: center;">
                                <Space>
                                    <Button
                                        disabled=Signal::derive(move || page.get() <= 1)
                                        on_click=move |_| { page.update(|p| *p -= 1); }
                                    >"上一页"</Button>
                                    <span>"第 " {move || page.get()} " / " {move || total_pages()} " 页"</span>
                                    <Button
                                        disabled=Signal::derive(move || page.get() >= total_pages())
                                        on_click=move |_| { page.update(|p| *p += 1); }
                                    >"下一页"</Button>
                                </Space>
                            </div>
                        }.into_any(),
                        Err(e) => view! { <Text>"加载失败: " {e.to_string()}</Text> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
