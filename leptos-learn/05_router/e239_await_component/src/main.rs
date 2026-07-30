use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::path;

fn main() {
    mount_to_body(Exercise);
}

// TODO: 在 DataPage 组件中使用 <Await/> 组件
// 根据路由参数 :id 加载数据并显示

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/data/42">"Data 42"</A>
                <A href="/data/99">"Data 99"</A>
            </nav>
            <main>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=|| view! { <p>"Home"</p> }/>
                    <Route path=path!("/data/:id") view=DataPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn DataPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params().get("id").unwrap_or_default();

    view! {
        <p>"Loading data for ID: " {id}</p>
    }
}
