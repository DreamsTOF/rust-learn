use leptos::prelude::*;
use leptos_meta::Body;
use leptos_router::components::*;
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <>
            <Body {..} class="home-page" style="background: #f5f5f5"/>
            <h1>"首页"</h1>
            <p>"当前 body 通过 <Body> 组件设置了 'home-page' class 和浅灰色背景"</p>
            <A href="/about">"关于页面"</A>
        </>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <>
            <Body {..} class="about-page" style="background: #e3f2fd"/>
            <h1>"关于页面"</h1>
            <p>"当前 body 通过 <Body> 组件设置了 'about-page' class 和浅蓝色背景"</p>
            <A href="/">"返回首页"</A>
        </>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "页面未找到">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/about") view=About/>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
