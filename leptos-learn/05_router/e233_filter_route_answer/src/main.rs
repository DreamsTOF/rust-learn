// ============================================================
// 练习 e233 — filter_route — 参考答案
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::query_signal;
use leptos_router::path;

#[component]
fn Exercise() -> impl IntoView {
    let (category, set_category) = query_signal::<String>("cat");
    let (in_stock, set_in_stock) = query_signal::<String>("stock");
    let (sort, set_sort) = query_signal::<String>("sort");

    let cat_val = move || category().unwrap_or_else(|| "all".into());
    let stock_val = move || in_stock().unwrap_or_else(|| "all".into());
    let sort_val = move || sort().unwrap_or_else(|| "name".into());

    view! {
        <Router>
            <h1>"e233: 筛选路由"</h1>
            <div style="border: 1px solid #ccc; padding: 12px; margin-bottom: 12px;">
                <h3>"筛选条件"</h3>

                <label>"分类: "
                    <select
                        prop:value=cat_val
                        on:change=move |ev| set_category.set(Some(event_target_value(&ev)))
                    >
                        <option value="all">"全部"</option>
                        <option value="electronics">"电子产品"</option>
                        <option value="clothing">"服装"</option>
                        <option value="books">"图书"</option>
                    </select>
                </label>

                <label style="margin-left: 16px;">
                    <input
                        type="checkbox"
                        prop:checked=move || stock_val() == "instock"
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_in_stock.set(if checked { Some("instock".into()) } else { Some("all".into()) });
                        }
                    />
                    "仅显示有货"
                </label>

                <br/>
                <label style="margin-top: 8px; display: inline-block;">
                    "排序: "
                    <input type="radio" name="sort" value="name"
                        prop:checked=move || sort_val() == "name"
                        on:change=move |_| set_sort.set(Some("name".into()))
                    /> "名称"
                    <input type="radio" name="sort" value="price"
                        prop:checked=move || sort_val() == "price"
                        on:change=move |_| set_sort.set(Some("price".into()))
                    /> "价格"
                    <input type="radio" name="sort" value="rating"
                        prop:checked=move || sort_val() == "rating"
                        on:change=move |_| set_sort.set(Some("rating".into()))
                    /> "评分"
                </label>
            </div>

            <div>
                <h3>"当前筛选状态"</h3>
                <p>"分类: " {cat_val}</p>
                <p>"库存: " {stock_val}</p>
                <p>"排序: " {sort_val}</p>
                <p>"(筛选条件已同步到 URL 查询参数)"</p>
            </div>

            <Routes fallback=|| "页面未找到">
                <Route path=path!("") view=|| view! { <p>"修改筛选条件，观察 URL 变化"</p> }/>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
