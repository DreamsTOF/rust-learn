use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::path;
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes};

use crate::layout::Layout;
use crate::pages::admin::coupons::CouponsPage;
use crate::pages::admin::settings::SettingsPage;
use crate::pages::admin::audit::AuditPage;
use crate::pages::admin::returns::ReturnsPage;
use crate::pages::admin::invoices::InvoicesPage;
use crate::pages::admin::reports::ReportsPage;
use crate::pages::auth::login::LoginPage;
use crate::pages::auth::register::RegisterPage;
use crate::pages::cart::CartPage;
use crate::pages::categories::CategoriesPage;
use crate::pages::checkout::CheckoutPage;
use crate::pages::dashboard::DashboardPage;
use crate::pages::home::HomePage;
use crate::pages::orders::detail::OrderDetailPage;
use crate::pages::orders::list::OrderListPage;
use crate::pages::products::detail::ProductDetailPage;
use crate::pages::products::import::ImportPage;
use crate::pages::products::list::ProductListPage;
use crate::pages::reconciliation::ReconciliationPage;
use crate::pages::user::addresses::AddressesPage;
use crate::pages::user::profile::ProfilePage;
use crate::pages::user::security::SecurityPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <ParentRoute path=path!("") view=Layout>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("login") view=LoginPage/>
                    <Route path=path!("register") view=RegisterPage/>
                    <Route path=path!("cart") view=CartPage/>
                    <Route path=path!("checkout") view=CheckoutPage/>
                    <ParentRoute path=path!("admin") view=move || view! { <Outlet/> }>
                        <Route path=path!("dashboard") view=DashboardPage/>
                        <Route path=path!("categories") view=CategoriesPage/>
                        <ParentRoute path=path!("products") view=move || view! { <Outlet/> }>
                            <Route path=path!("") view=ProductListPage/>
                            <Route path=path!("import") view=ImportPage/>
                            <Route path=path!(":id") view=ProductDetailPage/>
                        </ParentRoute>
                        <ParentRoute path=path!("orders") view=move || view! { <Outlet/> }>
                            <Route path=path!("") view=OrderListPage/>
                            <Route path=path!(":id") view=OrderDetailPage/>
                        </ParentRoute>
                        <Route path=path!("coupons") view=CouponsPage/>
                        <Route path=path!("returns") view=ReturnsPage/>
                        <Route path=path!("invoices") view=InvoicesPage/>
                        <Route path=path!("audit") view=AuditPage/>
                        <Route path=path!("settings") view=SettingsPage/>
                        <Route path=path!("reports") view=ReportsPage/>
                        <Route path=path!("reconciliation") view=ReconciliationPage/>
                    </ParentRoute>
                    <ParentRoute path=path!("user") view=move || view! { <Outlet/> }>
                        <Route path=path!("profile") view=ProfilePage/>
                        <Route path=path!("addresses") view=AddressesPage/>
                        <Route path=path!("security") view=SecurityPage/>
                    </ParentRoute>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
