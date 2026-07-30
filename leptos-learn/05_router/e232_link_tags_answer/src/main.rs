// ============================================================
// 练习 e232 — link_tags — 参考答案
// ============================================================

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

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
