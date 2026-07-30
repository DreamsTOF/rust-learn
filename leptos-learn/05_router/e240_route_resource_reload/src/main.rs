// ============================================================
// Exercise 240 - Route Resource Reload
// ============================================================
 
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::path;

fn main() {
    mount_to_body(Exercise);
}

// TODO: 在 UserPage 中使用 Resource::new
// 以 use_params_map() 作为 source，使得路由参数变化时自动重新加载数据

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/user/1">"User 1"</A>
                <A href="/user/2">"User 2"</A>
                <A href="/user/3">"User 3"</A>
            </nav>
            <main>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=|| view! { <p>"Home"</p> }/>
                    <Route path=path!("/user/:id") view=UserPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn UserPage() -> impl IntoView {
    let params = use_params_map();
    let user_data = Resource::new(
        move || params().get("id").unwrap_or_default(),
        |id| async move {
            format!("User data loaded for ID: {}", id)
        },
    );

    view! {
        <p>{move || user_data.map(|d| d.clone()).unwrap_or_default()}</p>
    }
}
