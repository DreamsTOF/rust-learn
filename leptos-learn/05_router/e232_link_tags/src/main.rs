// ============================================================
// 练习 e232: Link 标签 (link_tags)
//
// 目标: 使用 leptos::meta::Link 组件设置页面链接标签
//
// 难度: ⭐⭐
// 核心知识点: Link 组件、favicon、样式表、预加载
// ============================================================

// TODO: 导入 leptos、leptos_meta 和 leptos_router
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

// TODO: 使用 <Link> 组件设置页面链接
// <Link rel="icon" href="/favicon.ico"/> 设置 favicon
// <Link rel="stylesheet" href="/style.css"/> 加载样式表
// <Link rel="preload" href="/hero.png" as_="image"/> 预加载资源
#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Link rel="icon" href="/favicon.ico"/>
        <Link rel="stylesheet" href="/style.css"/>
        <Link rel="preload" href="/hero.png" as_="image"/>

        <Router>
            <h1>"e232: Link 标签"</h1>
            <p>"查看 <head> 中的 link 标签 (打开 DevTools)"</p>
            <Routes fallback=|| "页面未找到">
                <Route path=path!("") view=|| view! { <p>"首页内容"</p> }/>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
