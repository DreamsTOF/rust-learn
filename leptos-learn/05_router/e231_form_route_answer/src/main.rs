// ============================================================
// 练习 e231 — form_route — 参考答案
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_query_map;
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"搜索页面"</h2>
        <Form action="/search" method="GET">
            <label>"关键词: " <input type="text" name="q" placeholder="输入搜索词"/></label>
            <br/>
            <label>"分类: "
                <select name="cat">
                    <option value="all">"全部"</option>
                    <option value="rust">"Rust"</option>
                    <option value="web">"Web"</option>
                </select>
            </label>
            <br/>
            <button type="submit">"搜索"</button>
        </Form>
        <p>"提交后表单数据将自动编码到 URL 中"</p>
    }
}

#[component]
fn Search() -> impl IntoView {
    let query = use_query_map();
    let q = move || query().get("q").unwrap_or_default();
    let cat = move || query().get("cat").unwrap_or_default();

    view! {
        <h2>"搜索结果"</h2>
        <p>"关键词: " {q}</p>
        <p>"分类: " {cat}</p>
        <A href="/">"返回搜索"</A>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            <h1>"e231: 表单路由"</h1>
            <Routes fallback=|| "页面未找到">
                <Route path=path!("") view=Home/>
                <Route path=path!("search") view=Search/>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(Exercise);
}
