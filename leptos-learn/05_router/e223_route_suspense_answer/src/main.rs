// ============================================================
// Exercise 223 - Answer (route_suspense)
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

async fn fetch_user_profile() -> String {
    "Alice".to_string()
}

async fn fetch_posts() -> Vec<String> {
    vec![
        "Post 1: Hello World".to_string(),
        "Post 2: Leptos is great".to_string(),
        "Post 3: Routing with Suspense".to_string(),
    ]
}

#[component]
fn Home() -> impl IntoView {
    let username = LocalResource::new(|| async { fetch_user_profile().await });
    let posts = LocalResource::new(|| async { fetch_posts().await });

    view! {
        <h2>"Home"</h2>
        <Suspense fallback=|| view! { <p style="color:#888;">"Loading user profile..."</p> }>
            <div style="padding:12px;background:#e8f5e9;border-radius:4px;margin-bottom:16px;">
                <h3>"Welcome back, " {move || username.map(|u| u.clone())} "!"</h3>
            </div>
        </Suspense>
        <Suspense fallback=|| view! { <p style="color:#888;">"Loading posts..."</p> }>
            <div>
                <h3>"Recent Posts"</h3>
                <ul>
                    {move || posts.map(|p| {
                        p.clone().into_iter().map(|post| view! { <li>{post}</li> }).collect::<Vec<_>>()
                    })}
                </ul>
            </div>
        </Suspense>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h2>"About"</h2>
        <p>"This page loads instantly without Suspense."</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav style="margin-bottom:16px;">
                <span style="margin-right:12px;"><A href="/">"Home"</A></span>
                <A href="/about">"About"</A>
            </nav>
            <main>
                <Routes fallback=|| "Page not found">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/about") view=About/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
