use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
 
fn main() {
    mount_to_body(Exercise);
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 use_is_routing() 获取路由切换状态
    // 当路由切换时，显示加载指示器
    // 结合 <Suspense/> 使用

    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/about">"About"</A>
            </nav>
            <main>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=|| view! { <p>"Home"</p> }/>
                    <Route path=path!("/about") view=|| view! { <p>"About"</p> }/>
                </Routes>
            </main>
        </Router>
    }
}
