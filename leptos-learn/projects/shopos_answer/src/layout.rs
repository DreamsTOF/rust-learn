use leptos::prelude::*;
use leptos_router::components::Outlet;
use thaw::ConfigProvider;
use crate::components::nav::sidebar::Sidebar;
use crate::components::nav::topbar::Topbar;

#[component]
pub fn Layout() -> impl IntoView {
    let collapsed = RwSignal::new(false);

    view! {
        <ConfigProvider>
            <div class="layout-container">
                <Sidebar collapsed />
                <div class="layout-main">
                    <Topbar collapsed />
                    <main class="layout-content">
                        <Outlet />
                    </main>
                </div>
            </div>
        </ConfigProvider>
    }
}
