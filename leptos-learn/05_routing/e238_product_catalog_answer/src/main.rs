// ============================================================
// Exercise 238 - Answer: Product Catalog
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_query_map;
use leptos_router::path;

static PRODUCTS: &[(&u32, &str, &str, &f64)] = &[
    (&1, "笔记本电脑", "电子", &5999.0),
    (&2, "机械键盘", "外设", &399.0),
    (&3, "无线鼠标", "外设", &129.0),
    (&4, "显示器 27寸", "电子", &2499.0),
    (&5, "USB-C 扩展坞", "配件", &299.0),
    (&6, "降噪耳机", "音频", &899.0),
    (&7, "移动硬盘 2TB", "存储", &549.0),
    (&8, "智能手表", "可穿戴", &1999.0),
];

#[component]
fn CatalogLayout() -> impl IntoView {
    view! {
        <div>
            <header>
                <nav>
                    <A href="/products/">"全部"</A>
                    <A href="/products/?category=电子">"电子"</A>
                    <A href="/products/?category=外设">"外设"</A>
                    <A href="/products/?category=配件">"配件"</A>
                </nav>
            </header>
            <main><Outlet/></main>
        </div>
    }
}

#[component]
fn ProductList() -> impl IntoView {
    let query = use_query_map();

    let filtered = move || {
        let q = query.read().get("q").unwrap_or_default().to_lowercase();
        let cat = query.read().get("category");
        PRODUCTS.iter().filter(|(_, name, category, _)| {
            let cat_match = cat.as_ref().map_or(true, |c| *category == c);
            let search_match = q.is_empty()
                || name.to_lowercase().contains(&q)
                || category.to_lowercase().contains(&q);
            cat_match && search_match
        }).collect::<Vec<_>>()
    };

    let page = move || {
        query
            .read()
            .get("page")
            .and_then(|p| p.parse::<usize>().ok())
            .unwrap_or(1)
    };
    let per_page = 4;
    let total_pages = move || (filtered().len() + per_page - 1) / per_page;

    let paginated = move || {
        let p = page().max(1);
        let start = (p - 1) * per_page;
        filtered().into_iter().skip(start).take(per_page).collect::<Vec<_>>()
    };

    view! {
        <div>
            <h1>"商品目录"</h1>
            <form method="get" action="/products/">
                <input
                    type="search"
                    name="q"
                    placeholder="搜索商品..."
                    value=move || query.read().get("q").unwrap_or_default()
                />
                <button type="submit">"搜索"</button>
            </form>
            <div class="product-grid">
                {move || paginated().iter().map(|(id, name, category, price)| view! {
                    <div class="product-card">
                        <h3><A href=format!("/products/{}", id)>{*name}</A></h3>
                        <p>"分类: " {*category}</p>
                        <p>"价格: ¥" {**price}</p>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
            <div class="pagination">
                {move || {
                    let current = page();
                    let total = total_pages();
                    (1..=total).map(|i| {
                        let active = if i == current { "current" } else { "" };
                        view! {
                            <A href=format!("/products/?page={}", i) attr:class=active>
                                {i}
                            </A>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

#[component]
fn ProductDetail() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let id = move || {
        params
            .read()
            .get("id")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0)
    };
    let product = move || PRODUCTS.iter().find(|(i, _, _, _)| **i == id());

    view! {
        <div>
            {move || product().map(|(_, name, category, price)| view! {
                <div>
                    <h1>{*name}</h1>
                    <p>"分类: " {*category}</p>
                    <p>"价格: ¥" {**price}</p>
                    <A href="/products/">"← 返回目录"</A>
                </div>
            })}
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "404 - 商品未找到">
                <ParentRoute path=path!("products") view=CatalogLayout>
                    <Route path=path!("") view=ProductList/>
                    <Route path=path!(":id") view=ProductDetail/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(App);
}
