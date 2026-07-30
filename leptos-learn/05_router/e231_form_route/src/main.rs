// ============================================================
// 练习 e231: 表单路由 (form_route)
//
// 目标: 表单数据作为路由参数，提交时自动编码到 URL query
//
// 难度: ⭐⭐
// 核心知识点: Form(GET) 组件、use_query_map
// ============================================================

// TODO: 导入 leptos 和 leptos_router
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_query_map;
use leptos_router::path;

// TODO: 首页 — 包含一个搜索表单
// 使用 <Form method="GET" action="/search">
// 包含 name="q" 的 input 和 name="cat" 的 select
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
        <p>"提交后表单数据将自动编码到 URL 中，如 ?q=xxx&cat=rust"</p>
    }
}

// TODO: 搜索结果页 — 从 URL query 参数读取搜索条件
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

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// use leptos::prelude::*;
// use leptos_router::components::*;
// use leptos_router::hooks::use_query_map;
// use leptos_router::path;
//
// #[component]
// fn Home() -> impl IntoView {
//     view! {
//         <h2>"搜索页面"</h2>
//         <Form action="/search" method="GET">
//             <label>"关键词: " <input type="text" name="q" placeholder="输入搜索词"/></label>
//             <br/>
//             <label>"分类: "
//                 <select name="cat">
//                     <option value="all">"全部"</option>
//                     <option value="rust">"Rust"</option>
//                     <option value="web">"Web"</option>
//                 </select>
//             </label>
//             <br/>
//             <button type="submit">"搜索"</button>
//         </Form>
//     }
// }
//
// #[component]
// fn Search() -> impl IntoView {
//     let query = use_query_map();
//     let q = move || query().get("q").unwrap_or_default();
//     let cat = move || query().get("cat").unwrap_or_default();
//     view! {
//         <h2>"搜索结果"</h2>
//         <p>"关键词: " {q}</p>
//         <p>"分类: " {cat}</p>
//         <A href="/">"返回"</A>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Router>
//             <h1>"e231: 表单路由"</h1>
//             <Routes fallback=|| "页面未找到">
//                 <Route path=path!("") view=Home/>
//                 <Route path=path!("search") view=Search/>
//             </Routes>
//         </Router>
//     }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - `<Form method="GET">` 将表单字段序列化为 URL query 参数
// - `use_query_map` 返回响应式的 `Memo<ParamsMap>`，自动追踪 URL 变化
// - 这是标准的搜索页面模式：表单提交 → URL 更新 → 页面响应
//
// </details>
