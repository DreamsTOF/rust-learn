// ============================================================
// 练习 e233: 动态 Meta (dynamic_meta)
//
// 目标: 根据当前路由动态设置 <Meta/> 标签（title、description）
//
// 难度: ⭐⭐⭐
// 核心知识点: MetaProvider, Meta, Title, use_location
// ============================================================

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

// 首页组件
#[component]
fn Home() -> impl IntoView {
    view! {
        <>
            <Title text="首页 - 动态 Meta 示例"/>
            <Meta name="description" content="这是首页的描述信息"/>
            <h1>"首页"</h1>
            <p>"欢迎来到动态 Meta 示例"</p>
            <A href="/about">"关于我们"</A>
            <br/>
            <A href="/contact">"联系我们"</A>
        </>
    }
}

// 关于页面组件
#[component]
fn About() -> impl IntoView {
    view! {
        <>
            <Title text="关于我们 - 动态 Meta 示例"/>
            <Meta name="description" content="了解我们的故事和团队信息"/>
            <h1>"关于我们"</h1>
            <p>"这是关于页面，展示了动态 Meta 标签的用法"</p>
            <A href="/">"返回首页"</A>
        </>
    }
}

// 联系页面组件
#[component]
fn Contact() -> impl IntoView {
    view! {
        <>
            <Title text="联系我们 - 动态 Meta 示例"/>
            <Meta name="description" content="联系我们获取更多支持和信息"/>
            <h1>"联系我们"</h1>
            <p>"这是联系页面"</p>
            <A href="/">"返回首页"</A>
        </>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| "页面未找到">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/about") view=About/>
                <Route path=path!("/contact") view=Contact/>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
