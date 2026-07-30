// ============================================================
// Exercise 215 - Answer: typed_params — derive(Params) 类型安全
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::path;

#[derive(Params, Debug, Clone, PartialEq)]
struct ProductParams {
    category: String,
    product_id: String,
    section: Option<String>,
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"商店首页"</h2>
        <p><a href="/product/electronics/42">"电子产品 42"</a></p>
        <p><a href="/product/books/7/details">"图书 7 详情"</a></p>
    }
}

#[component]
fn Product() -> impl IntoView {
    let params = use_params::<ProductParams>();

    view! {
        <h2>"商品详情"</h2>
        <div>
            {move || match params.read().as_ref() {
                Ok(p) => {
                    view! {
                        <p>"分类: " {p.category.clone()}</p>
                        <p>"商品ID: " {p.product_id.clone()}</p>
                        <p>"版块: " {p.section.clone().unwrap_or("默认".to_string())}</p>
                    }.into_any()
                }
                Err(e) => view! { <p>"参数错误: " {e.to_string()}</p> }.into_any(),
            }}
        </div>
        <p><a href="/">"返回首页"</a></p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <a href="/">"商店"</a>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/product/:category/:product_id/:section?") view=Product/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
