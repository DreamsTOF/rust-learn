// ============================================================
// Exercise 225 - Answer (active_nav_highlight)
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_location;
use leptos_router::path;

#[component]
fn Dashboard() -> impl IntoView {
    view! {
        <h2>"Dashboard"</h2>
        <p>"Welcome to the dashboard overview."</p>
    }
}

#[component]
fn Analytics() -> impl IntoView {
    view! {
        <h2>"Analytics"</h2>
        <p>"View your analytics data here."</p>
    }
}

#[component]
fn Reports() -> impl IntoView {
    view! {
        <h2>"Reports"</h2>
        <p>"Generated reports and summaries."</p>
    }
}

#[component]
fn Settings() -> impl IntoView {
    view! {
        <h2>"Settings"</h2>
        <p>"Manage your application settings."</p>
    }
}

#[component]
fn Profile() -> impl IntoView {
    view! {
        <h2>"Profile"</h2>
        <p>"Your personal profile information."</p>
    }
}

#[component]
fn SidebarLayout() -> impl IntoView {
    let location = use_location();
    let pathname = move || location.pathname.get();

    // Manual parent-route active detection:
    // highlight "Dashboard" when on any /dashboard/* route (including children)
    let dashboard_active = Signal::derive(move || pathname().starts_with("/dashboard"));

    view! {
        <div style="display:flex;gap:16px;font-family:sans-serif;">
            <aside style="width:200px;background:#f5f5f5;padding:16px;border-radius:8px;">
                <h3>"Navigation"</h3>
                <ul style="list-style:none;padding:0;">
                    <li style="margin-bottom:4px;">
                        <A href="/dashboard" class:active=dashboard_active>"📊 Dashboard"</A>
                    </li>
                    <li style="margin-left:20px;margin-bottom:2px;">
                        <A href="/dashboard/analytics" class:active>" Analytics"</A>
                    </li>
                    <li style="margin-left:20px;margin-bottom:2px;">
                        <A href="/dashboard/reports" class:active>" Reports"</A>
                    </li>
                    <li style="margin-top:12px;margin-bottom:4px;">
                        <A href="/settings" class:active>"⚙️ Settings"</A>
                    </li>
                    <li style="margin-bottom:4px;">
                        <A href="/profile" class:active>"👤 Profile"</A>
                    </li>
                </ul>
            </aside>
            <main style="flex:1;padding:16px;border:1px solid #ddd;border-radius:8px;">
                <Outlet/>
            </main>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <h1>"e225: Active Nav Highlight"</h1>
            <style>
                "
                ul a {
                    display: block;
                    padding: 6px 10px;
                    border-radius: 4px;
                    text-decoration: none;
                    color: #333;
                    transition: background-color 0.2s, color 0.2s;
                }
                ul a:hover {
                    background-color: #e8e8e8;
                }
                .active {
                    font-weight: bold !important;
                    background-color: #e3f2fd !important;
                    color: #1976d2 !important;
                    border-right: 3px solid #1976d2;
                }
                "
            </style>
            <Routes fallback=|| "Page not found">
                <ParentRoute path=path!("/") view=SidebarLayout>
                    <Route path=path!("dashboard") view=Dashboard/>
                    <Route path=path!("dashboard/analytics") view=Analytics/>
                    <Route path=path!("dashboard/reports") view=Reports/>
                    <Route path=path!("settings") view=Settings/>
                    <Route path=path!("profile") view=Profile/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
