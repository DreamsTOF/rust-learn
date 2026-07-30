// ============================================================
// 练习 e234: 分页路由 (pagination_route)
//
// 目标: 分页页码同步到 URL 查询参数，支持浏览器前进/后退
//
// 难度: ⭐⭐
// 核心知识点: query_signal 数值类型、分页导航
// ============================================================

// TODO: 导入所需模块
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::query_signal;
use leptos_router::path;

const TOTAL_PAGES: i32 = 10;

// TODO: 模拟数据 — 根据页码生成内容
fn page_content(page: i32) -> Vec<String> {
    let start = (page - 1) * 3 + 1;
    (start..start + 3).map(|i| format!("项目 {}", i)).collect()
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 query_signal 将页码同步到 URL 的 page 参数
    let (page, set_page) = query_signal::<i32>("page");

    let current = move || {
        let p = page().unwrap_or(1);
        p.clamp(1, TOTAL_PAGES)
    };

    let items = move || page_content(current());

    let go_to = move |p: i32| {
        let p = p.clamp(1, TOTAL_PAGES);
        set_page.set(Some(p));
    };

    view! {
        <Router>
            <h1>"e234: 分页路由"</h1>

            <div>
                // TODO: 渲染当前页的项目列表
                <ul>
                    {move || items().into_iter().map(|item| view! { <li>{item}</li> }).collect_view()}
                </ul>
            </div>

            // TODO: 分页控件 — 上一页、页码、下一页
            <div style="margin-top: 12px;">
                <button on:click=move |_| go_to(current() - 1) disabled=move || current() == 1>
                    "上一页"
                </button>

                {move || {
                    let cur = current();
                    (1..=TOTAL_PAGES).map(|p| {
                        let style = if p == cur { "font-weight: bold; margin: 0 4px;" } else { "margin: 0 4px;" };
                        view! {
                            <button style=style on:click=move |_| go_to(p)>
                                {p}
                            </button>
                        }
                    }).collect_view()
                }}

                <button on:click=move |_| go_to(current() + 1) disabled=move || current() == TOTAL_PAGES>
                    "下一页"
                </button>
            </div>

            <p>"当前页: " {move || current()} " / " {TOTAL_PAGES}</p>
            <p>"(页码已同步到 URL: ?page=N)"</p>

            <Routes fallback=|| "页面未找到">
                <Route path=path!("") view=|| view! { <p>"使用分页控件浏览"</p> }/>
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
// use leptos_router::hooks::query_signal;
// use leptos_router::path;
//
// const TOTAL_PAGES: i32 = 10;
//
// fn page_content(page: i32) -> Vec<String> {
//     let start = (page - 1) * 3 + 1;
//     (start..start + 3).map(|i| format!("项目 {}", i)).collect()
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (page, set_page) = query_signal::<i32>("page");
//     let current = move || { let p = page().unwrap_or(1); p.clamp(1, TOTAL_PAGES) };
//     let items = move || page_content(current());
//     let go_to = move |p: i32| { let p = p.clamp(1, TOTAL_PAGES); set_page.set(Some(p)); };
//
//     view! {
//         <Router>
//             <h1>"e234: 分页路由"</h1>
//             <div>
//                 <ul>{move || items().into_iter().map(|item| view!{<li>{item}</li>}).collect_view()}</ul>
//             </div>
//             <div style="margin-top:12px;">
//                 <button on:click=move|_| go_to(current()-1) disabled=move||current()==1>"上一页"</button>
//                 {move || { let cur = current(); (1..=TOTAL_PAGES).map(|p| {
//                     let style = if p == cur { "font-weight:bold;margin:0 4px;" } else { "margin:0 4px;" };
//                     view!{<button style=style on:click=move|_| go_to(p)>{p}</button>}
//                 }).collect_view() }}
//                 <button on:click=move|_| go_to(current()+1) disabled=move||current()==TOTAL_PAGES>"下一页"</button>
//             </div>
//             <p>"当前页: " {move || current()} " / " {TOTAL_PAGES}</p>
//             <Routes fallback=||"404">
//                 <Route path=path!("") view=||view!{<p>"使用分页控件浏览"</p>}/>
//             </Routes>
//         </Router>
//     }
// }
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - `query_signal::<i32>("page")` 将数值类型同步到 URL
// - 修改 set_page 会自动更新 URL，浏览器导航也会更新 signal
// - 通过 `clamp` 确保页码在有效范围内
// - 分页同步到 URL 可使分享的链接包含特定页码
//
// </details>
