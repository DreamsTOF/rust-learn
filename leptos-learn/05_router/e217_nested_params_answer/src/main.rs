// ============================================================
// Exercise 217 - Answer
// nested_params — 嵌套路由中的路径参数
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params_map;
use leptos_router::path;

#[component]
fn SectionLayout() -> impl IntoView {
    let params = use_params_map();
    let section = move || params.read().get("section").unwrap_or_default();

    view! {
        <nav>
            <a href={move || format!("/{}", section())}>"版块首页"</a>
            <a href={move || format!("/{}/1", section())}>"第 1 项"</a>
            <a href={move || format!("/{}/2", section())}>"第 2 项"</a>
            <a href={move || format!("/{}/3", section())}>"第 3 项"</a>
        </nav>
        <h2>{move || format!("版块: {}", section())}</h2>
        <Outlet/>
    }
}

#[component]
fn SectionHome() -> impl IntoView {
    let params = use_params_map();
    let section = move || params.read().get("section").unwrap_or_default();
    view! { <p>"欢迎来到 " {move || section()} " 版块"</p> }
}

#[component]
fn ItemDetail() -> impl IntoView {
    let params = use_params_map();
    let section = move || params.read().get("section").unwrap_or_default();
    let id = move || params.read().get("id").unwrap_or_default();
    view! { <p>"正在查看 " {move || section()} " 版块的第 " {move || id()} " 项"</p> }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "页面未找到">
                    <ParentRoute path=path!("/:section") view=SectionLayout>
                        <Route path=path!("/:section") view=SectionHome/>
                        <Route path=path!("/:section/:id") view=ItemDetail/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
