// ============================================================
// 练习 e236: blog_routes — 博客路由设计
//
// 目标: 实现博客的完整路由系统，包括文章列表、详情和分类
//
// 难度: ⭐⭐⭐
// 核心知识点: Router, Routes, ParentRoute, Route, Outlet, A
//
// TODO: 阅读代码理解路由结构，尝试添加"关于"页面路由
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

// --- 数据 ---
static POSTS: &[(&u32, &str, &str)] = &[
    (&1, "Rust 入门指南", "学习 Rust 语言的基础知识，从变量到所有权。"),
    (&2, "Leptos 框架介绍", "探索 Leptos 响应式 Web 框架的核心概念。"),
    (&3, "响应式编程基础", "理解信号、效果和派生状态的工作原理。"),
];

static CATEGORIES: &[(&str, &str)] = &[("rust", "Rust"), ("web", "Web 开发"), ("tutorial", "教程")];

// === 步骤 1 — BlogLayout: 共享布局组件 ———————————————
// TODO: 研究 BlogLayout 如何通过 <Outlet/> 渲染子路由
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

// === 步骤 2 — PostList: 动态渲染文章列表 ————————————
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

// === 步骤 3 — PostDetail: 路径参数读取 ——————————————————
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

// === 步骤 4 — CategoryDetail: 分类参数读取 ————————————
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

// === 步骤 5 — App: 路由配置入口 ———————————————————————
// TODO: 尝试添加一个"关于"页面
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

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 添加"关于"页面
// ```rust
// #[component]
// fn About() -> impl IntoView {
//     view! {
//         <div>
//             <h1>"关于本站"</h1>
//             <p>"这是一个使用 Leptos 构建的博客。"</p>
//         </div>
//     }
// }
// // 在 App 的 <ParentRoute> 中添加:
// // <Route path=path!("about") view=About/>
// ```
//
// ### 知识点
// - `ParentRoute` 提供共享布局，子路由通过 `Outlet` 渲染
// - 嵌套 `ParentRoute` 实现多级路径（如 /posts/:id）
// - `use_params_map` 在组件内读取 URL 参数
// - `Routes fallback` 处理未匹配路径
//
// </details>
