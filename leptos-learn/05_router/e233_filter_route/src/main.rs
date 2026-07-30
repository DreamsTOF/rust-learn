// ============================================================
// 练习 e233: 筛选路由 (filter_route)
//
// 目标: 多条件筛选器与 URL 查询参数双向同步
//
// 难度: ⭐⭐
// 核心知识点: query_signal、表单控件与 URL 同步
// ============================================================

// TODO: 导入所需模块
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::query_signal;
use leptos_router::path;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 query_signal 创建三个筛选条件，与 URL 双向同步
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

                // 分类筛选
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

                // 库存筛选
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

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// use leptos::prelude::*;
// use leptos_router::components::*;
// use leptos_router::hooks::query_signal;
// use leptos_router::path;
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (category, set_category) = query_signal::<String>("cat");
//     let (in_stock, set_in_stock) = query_signal::<String>("stock");
//     let (sort, set_sort) = query_signal::<String>("sort");
//
//     let cat_val = move || category().unwrap_or_else(|| "all".into());
//     let stock_val = move || in_stock().unwrap_or_else(|| "all".into());
//     let sort_val = move || sort().unwrap_or_else(|| "name".into());
//
//     view! {
//         <Router>
//             <h1>"e233: 筛选路由"</h1>
//             <div style="border:1px solid #ccc;padding:12px;margin-bottom:12px;">
//                 <h3>"筛选条件"</h3>
//                 <label>"分类: "
//                     <select prop:value=cat_val on:change=move|ev| set_category.set(Some(event_target_value(&ev)))>
//                         <option value="all">"全部"</option>
//                         <option value="electronics">"电子产品"</option>
//                         <option value="clothing">"服装"</option>
//                         <option value="books">"图书"</option>
//                     </select>
//                 </label>
//                 <label style="margin-left:16px;">
//                     <input type="checkbox" prop:checked=move||stock_val()=="instock"
//                         on:change=move|ev| { let checked = event_target_checked(&ev); set_in_stock.set(if checked { Some("instock".into()) } else { Some("all".into()) }); }/>
//                     "仅显示有货"
//                 </label>
//                 <br/>
//                 <label style="margin-top:8px;display:inline-block;">
//                     "排序: "
//                     <input type="radio" name="sort" value="name" prop:checked=move||sort_val()=="name" on:change=move|_| set_sort.set(Some("name".into()))/> "名称"
//                     <input type="radio" name="sort" value="price" prop:checked=move||sort_val()=="price" on:change=move|_| set_sort.set(Some("price".into()))/> "价格"
//                     <input type="radio" name="sort" value="rating" prop:checked=move||sort_val()=="rating" on:change=move|_| set_sort.set(Some("rating".into()))/> "评分"
//                 </label>
//             </div>
//             <div>
//                 <h3>"当前筛选状态"</h3>
//                 <p>"分类: " {cat_val}</p>
//                 <p>"库存: " {stock_val}</p>
//                 <p>"排序: " {sort_val}</p>
//             </div>
//             <Routes fallback=||"404">
//                 <Route path=path!("") view=||view!{<p>"修改筛选条件，观察 URL 变化"</p>}/>
//             </Routes>
//         </Router>
//     }
// }
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - `query_signal::<T>("key")` 提供与 URL query 的双向绑定
// - 任何 `T: FromStr + ToString` 类型都可作为 query_signal 的类型参数
// - 修改 signal 自动更新 URL；浏览器前进/后退自动更新 signal
// - 适合分类、排序、筛选等需要持久化到 URL 的 UI 状态
//
// </details>
