// ============================================================
// Exercise 222 - Answer (layout_props)
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[derive(Clone)]
struct LayoutData {
    title: String,
    description: String,
}

#[derive(Clone)]
struct UserInfo {
    name: String,
    role: String,
}

#[component]
fn AppLayout(title: String) -> impl IntoView {
    let layout_data = LayoutData {
        title: title.clone(),
        description: format!("Description for: {}", title),
    };
    provide_context(layout_data);

    view! {
        <div style="border:2px solid #2196F3;padding:16px;border-radius:8px;font-family:sans-serif;">
            <h2 style="color:#2196F3;margin-top:0;">{title}</h2>
            <nav style="margin-bottom:12px;">
                <span style="margin-right:12px;"><A href="/profile">"Profile"</A></span>
                <A href="/settings">"Settings"</A>
            </nav>
            <Outlet/>
        </div>
    }
}

#[component]
fn Profile() -> impl IntoView {
    let layout = use_context::<LayoutData>().expect("LayoutData not provided");
    let user = use_context::<UserInfo>().expect("UserInfo not provided");

    view! {
        <div style="padding:12px;background:#e3f2fd;border-radius:4px;">
            <h3>"Profile"</h3>
            <p>"Layout Title: " {layout.title.clone()}</p>
            <p>"Layout Description: " {layout.description.clone()}</p>
            <hr/>
            <p>"User: " {user.name.clone()} " (" {user.role.clone()} ")"</p>
        </div>
    }
}

#[component]
fn Settings() -> impl IntoView {
    let layout = use_context::<LayoutData>().expect("LayoutData not provided");

    view! {
        <div style="padding:12px;background:#fff3e0;border-radius:4px;">
            <h3>"Settings"</h3>
            <p>"Layout Title: " {layout.title.clone()}</p>
            <p>"Layout Description: " {layout.description.clone()}</p>
            <p>"Configure your preferences here."</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let user = UserInfo {
        name: "Alice".to_string(),
        role: "Admin".to_string(),
    };
    provide_context(user);

    view! {
        <Router>
            <h1>"e222: Layout Props"</h1>
            <Routes fallback=|| "Page not found">
                <ParentRoute path=path!("/") view=move || view! { <AppLayout title="User Center".to_string()/> }>
                    <Route path=path!("profile") view=Profile/>
                    <Route path=path!("settings") view=Settings/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
