// ============================================================
// 练习 e232: 搜索表单 (search_form)
//
// 目标: 搜索表单提交时更新 URL query 参数
//
// 难度: ⭐⭐
// 核心知识点: use_navigate、event_target_value、use_query_map
// ============================================================

// TODO: 导入所需模块
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::{use_navigate, use_query_map};
use leptos_router::{path, NavigateOptions};

// TODO: 补全搜索组件
// - 输入框的值通过 use_query_map 从 URL 读取
// - 输入时通过 use_navigate 更新 URL query 参数
#[component]
fn Exercise() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    // 从 URL 读取当前搜索词（响应式）
    let current_q = move || query().get("q").unwrap_or_default();

    // 每次输入时更新 URL
    let on_input = move |ev| {
        let value = event_target_value(&ev);
        let _ = navigate(&format!("/?q={value}"), NavigateOptions::default());
    };

    view! {
        <Router>
            <h1>"e232: 搜索表单"</h1>
            <div>
                <input
                    type="text"
                    placeholder="输入搜索词..."
                    prop:value=current_q
                    on:input=on_input
                />
            </div>
            <p>"当前搜索词: " {current_q}</p>
            <Routes fallback=|| "页面未找到">
                <Route path=path!("") view=move || {
                    let q = current_q();
                    if q.is_empty() {
                        view! { <p>"请输入关键词搜索"</p> }.into_any()
                    } else {
                        view! { <p>"正在搜索 \"" {q} "\" 的结果..."</p> }.into_any()
                    }
                }/>
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
// use leptos_router::hooks::{use_navigate, use_query_map};
// use leptos_router::{path, NavigateOptions};
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let query = use_query_map();
//     let navigate = use_navigate();
//     let current_q = move || query().get("q").unwrap_or_default();
//
//     let on_input = move |ev| {
//         let value = event_target_value(&ev);
//         let _ = navigate(&format!("/?q={value}"), NavigateOptions::default());
//     };
//
//     view! {
//         <Router>
//             <h1>"e232: 搜索表单"</h1>
//             <div>
//                 <input type="text" placeholder="输入搜索词..." prop:value=current_q on:input=on_input/>
//             </div>
//             <p>"当前搜索词: " {current_q}</p>
//             <Routes fallback=|| "404">
//                 <Route path=path!("") view=move || {
//                     let q = current_q();
//                     if q.is_empty() { view!{<p>"请输入关键词搜索"</p>}.into_any() }
//                     else { view!{<p>"正在搜索 \""{q}"\" 的结果..."</p>}.into_any() }
//                 }/>
//             </Routes>
//         </Router>
//     }
// }
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - `use_navigate` 返回闭包，接收路径和 `NavigateOptions`，编程式导航
// - `use_query_map` 读取 URL 查询参数，返回值是响应式的
// - `event_target_value` 从输入事件中提取输入框的值
// - 将搜索词同步到 URL 便于分享和书签
//
// </details>
