// ============================================================
// Exercise 236 - Answer: Blog Routes
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

static POSTS: &[(&u32, &str, &str)] = &[
    (&1, "Rust 入门指南", "学习 Rust 语言的基础知识，从变量到所有权。"),
    (&2, "Leptos 框架介绍", "探索 Leptos 响应式 Web 框架的核心概念。"),
    (&3, "响应式编程基础", "理解信号、效果和派生状态的工作原理。"),
];

static CATEGORIES: &[(&str, &str)] = &[("rust", "Rust"), ("web", "Web 开发"), ("tutorial", "教程")];

#[component]
fn BlogLayout() -> impl IntoView {
    view! {
        <div class="blog-layout">
            <header>
                <nav>
                    <A href="/">"首页"</A>
                    <A href="/posts">"文章"</A>
                    <A href="/categories">"分类"</A>
                </nav>
            </header>
            <main><Outlet/></main>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <h1>"欢迎来到博客"</h1>
            <p>"最新文章和技术分享汇聚于此。"</p>
            <A href="/posts">"浏览文章 →"</A>
        </div>
    }
}

#[component]
fn PostList() -> impl IntoView {
    view! {
        <div>
            <h1>"文章列表"</h1>
            <ul>
                {POSTS.iter().map(|(id, title, _)| view! {
                    <li><A href=format!("/posts/{}", id)>{*title}</A></li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[component]
fn PostDetail() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();
    let post = move || POSTS.iter().find(|(i, _, _)| format!("{}", i) == id());

    view! {
        <div>
            {move || post().map(|(_, title, summary)| view! {
                <div>
                    <h1>{*title}</h1>
                    <p>{*summary}</p>
                </div>
            })}
            <A href="/posts">"← 返回列表"</A>
        </div>
    }
}

#[component]
fn CategoryList() -> impl IntoView {
    view! {
        <div>
            <h1>"分类"</h1>
            <ul>
                {CATEGORIES.iter().map(|(slug, name)| view! {
                    <li><A href=format!("/categories/{}", slug)>{*name}</A></li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[component]
fn CategoryDetail() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();
    view! {
        <div>
            <h1>"分类: " {slug}</h1>
            <p>"该分类下的文章列表..."</p>
            <A href="/categories">"← 返回分类"</A>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "404 - 页面未找到">
                <ParentRoute path=path!("") view=BlogLayout>
                    <Route path=path!("") view=Home/>
                    <ParentRoute path=path!("posts") view=|| view! { <Outlet/> }>
                        <Route path=path!("") view=PostList/>
                        <Route path=path!(":id") view=PostDetail/>
                    </ParentRoute>
                    <ParentRoute path=path!("categories") view=|| view! { <Outlet/> }>
                        <Route path=path!("") view=CategoryList/>
                        <Route path=path!(":slug") view=CategoryDetail/>
                    </ParentRoute>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(App);
}
