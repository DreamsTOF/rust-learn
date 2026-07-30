// ============================================================
// Exercise 218 - Answer
// nested_breadcrumb — 基于路由树的面包屑导航
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_location;
use leptos_router::path;

#[component]
fn BreadcrumbLayout() -> impl IntoView {
    let location = use_location();

    let crumbs = move || {
        let path = location.pathname.get();
        let segments: Vec<&str> = path
            .trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let mut items: Vec<AnyView> = Vec::new();
        items.push(view! { <a href="/">"首页"</a> }.into_any());

        let mut cumulative = String::new();
        for seg in &segments {
            cumulative.push('/');
            cumulative.push_str(seg);
            items.push(view! { <span>" > "</span> }.into_any());
            let link = cumulative.clone();
            items.push(
                view! { <a href={link}>{seg.to_string()}</a> }.into_any(),
            );
        }
        items
    };

    view! {
        <nav>{move || crumbs()}</nav>
        <Outlet/>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <ul>
            <li><a href="/products">"产品"</a></li>
            <li><a href="/services">"服务"</a></li>
            <li><a href="/about">"关于"</a></li>
        </ul>
    }
}

#[component]
fn Products() -> impl IntoView {
    view! {
        <h2>"产品"</h2>
        <ul>
            <li><a href="/products/software">"软件"</a></li>
            <li><a href="/products/hardware">"硬件"</a></li>
        </ul>
    }
}

#[component]
fn ProductCategory() -> impl IntoView {
    let location = use_location();
    let seg = move || {
        location.pathname
            .get()
            .trim_matches('/')
            .split('/')
            .last()
            .unwrap_or("")
            .to_string()
    };
    view! { <p>{move || format!("{} 产品列表", seg())}</p> }
}

#[component]
fn Services() -> impl IntoView {
    view! { <p>"我们的服务包括咨询、开发和培训。"</p> }
}

#[component]
fn About() -> impl IntoView {
    view! { <p>"关于我们公司"</p> }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "页面未找到">
                    <ParentRoute path=path!("/") view=BreadcrumbLayout>
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/products/:category") view=ProductCategory/>
                        <Route path=path!("/products") view=Products/>
                        <Route path=path!("/services") view=Services/>
                        <Route path=path!("/about") view=About/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
