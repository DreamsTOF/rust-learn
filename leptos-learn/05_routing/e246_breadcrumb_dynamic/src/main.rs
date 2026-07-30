// ============================================================
// 练习 246: breadcrumb_dynamic — 动态面包屑
//
// 目标: 根据当前路由自动生成面包屑导航
//
// 难度: ⭐⭐⭐
// 核心知识点: 动态面包屑
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_location;
use leptos_router::path;

// --- 面包屑核心逻辑 ---

/// 将路径 segment 转为可读标签
fn segment_label(segment: &str) -> String {
    if segment.is_empty() {
        return "Home".into();
    }
    segment
        .split(|c: char| c == '-' || c == '_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// 面包屑导航组件：响应式读取 pathname，生成层级链接
#[component]
fn Breadcrumbs() -> impl IntoView {
    let location = use_location();

    // 每次 pathname 变化重新计算面包屑项
    let crumbs = move || {
        let pathname = location.pathname.get();
        let segments: Vec<&str> = pathname.split('/').filter(|s| !s.is_empty()).collect();

        let mut items: Vec<(String, String)> = Vec::new();
        let mut accumulated = String::new();

        // 根路径 Home
        items.push(("Home".into(), "/".into()));

        for seg in &segments {
            accumulated.push('/');
            accumulated.push_str(seg);
            items.push((segment_label(seg), accumulated.clone()));
        }

        items
    };

    view! {
        <nav aria-label="Breadcrumb">
            <ol style="list-style: none; display: flex; gap: 0.25rem; padding: 0.5rem 0; margin: 0;">
                {move || {
                    let items = crumbs();
                    let len = items.len();
                    items
                        .into_iter()
                        .enumerate()
                        .map(move |(i, (label, path))| {
                            let is_last = i == len - 1;
                            view! {
                                <li>
                                    {if is_last {
                                        view! { <span style="font-weight: bold; color: #333;">{label}</span> }
                                            .into_any()
                                    } else {
                                        view! {
                                            <>
                                                <A href=path>{label}</A>
                                                <span style="margin: 0 0.25rem; color: #999;">/</span>
                                            </>
                                        }
                                            .into_any()
                                    }}
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ol>
        </nav>
    }
}

// --- 页面组件 ---

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h2>"Home"</h2>
            <p>"Welcome to the homepage."</p>
        </div>
    }
}

#[component]
fn ProductsPage() -> impl IntoView {
    let products = vec!["laptop", "phone", "tablet", "monitor"];
    view! {
        <div>
            <h2>"Products"</h2>
            <ul>
                {products
                    .into_iter()
                    .map(|p| {
                        view! { <li><A href=format!("/products/{}", p)>{p}</A></li> }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}

#[component]
fn ProductDetail() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();
    view! {
        <div>
            <h2>{move || format!("Product: {}", id())}</h2>
            <p>{move || format!("You are viewing product \"{}\".", id())}</p>
            <A href="/products">"← Back to products"</A>
        </div>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div>
            <h2>"About"</h2>
            <p>"This is the about page."</p>
        </div>
    }
}

/// 布局：面包屑 + 内容出口
#[component]
fn Layout() -> impl IntoView {
    view! {
        <div style="max-width: 800px; margin: 0 auto; padding: 1rem; font-family: system-ui, sans-serif;">
            <Breadcrumbs />
            <hr />
            <Outlet />
        </div>
    }
}

// --- 入口 ---

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Router>
                <Routes fallback=|| "Page not found.">
                    <ParentRoute path=path!("/") view=Layout>
                        <Route path=path!("") view=HomePage />
                        <Route path=path!("products") view=ProductsPage />
                        <Route path=path!("products/:id") view=ProductDetail />
                        <Route path=path!("about") view=AboutPage />
                    </ParentRoute>
                </Routes>
            </Router>
        }
    });
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 关键思路
// 1. `use_location().pathname` 获取当前路径字符串
// 2. 按 `"/"` 分割后逐步累加生成每级完整 URL
// 3. `segment_label()` 将路径片段（如 "product-detail"）转为可读标签（"Product Detail"）
// 4. 最后一级用 `<span>` 纯文本，其余用 `<A>` 包裹成超链接
// 5. 全部包裹在 `<nav aria-label="Breadcrumb"><ol>` 语义结构中
//
// ### 知识点
// - `use_location()` 响应式获取路由位置
// - `<ParentRoute>` + `<Outlet>` 嵌套布局
// - `<A>` 客户端导航链接
// - `.collect_view()` 将 Vec<HtmlElement> 转为 View
//
// </details>
