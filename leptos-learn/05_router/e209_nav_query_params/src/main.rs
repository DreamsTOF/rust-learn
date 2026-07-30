// ============================================================
// 练习 e209: 查询参数导航 (nav_query_params)
//
// 目标: 使用 use_navigate() 传递查询参数，
//       并用 use_query_map() 读取查询参数。
//
// 难度: ⭐⭐
// 核心知识点: use_navigate() + 查询参数, use_query_map()
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;

// ============================================================
// 步骤 1 — 创建搜索页面组件
// ============================================================

#[component]
fn SearchPage() -> impl IntoView {
    // TODO: 调用 use_query_map() 获取查询参数映射
    // let query = use_query_map();
    //
    // 使用 move 闭包提取 "q" 参数:
    // let search_term = move || {
    //     query().get("q").cloned().unwrap_or_default()
    // };

    view! {
        <h2>"搜索页面"</h2>
        // TODO: 显示当前搜索词
        // <p>"当前搜索词: " {search_term}</p>
    }
}

// ============================================================
// 步骤 2 — 创建搜索按钮组件
// ============================================================

#[component]
fn SearchButtons() -> impl IntoView {
    // TODO: 调用 use_navigate()
    // let navigate = use_navigate();
    //
    // 使用 navigate 导航到查询 URL:
    // navigate("/search?q=leptos", Default::default())
    // navigate("/search?q=rust", Default::default())

    view! {
        <nav>
            // TODO: 添加两个按钮，分别搜索 "leptos" 和 "rust"
        </nav>
    }
}

// ============================================================
// 步骤 3 — 组装根组件
// ============================================================

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <Router>
            // TODO: 放置 <SearchButtons/>
            <main>
                <Routes fallback=|| "页面未找到">
                    // TODO: 添加 /search 路由，渲染 SearchPage
                </Routes>
            </main>
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
use leptos_router::hooks::*;
//
// #[component]
// fn SearchPage() -> impl IntoView {
//     let query = use_query_map();
//     let search_term = move || {
//         query()
//             .get("q")
//             .cloned()
//             .unwrap_or_default()
//     };
//
//     view! {
//         <h2>"搜索页面"</h2>
//         <p>"当前搜索词: " {search_term}</p>
//     }
// }
//
// #[component]
// fn SearchButtons() -> impl IntoView {
//     let navigate = use_navigate();
//     view! {
//         <nav>
//             <button on:click=move |_| navigate("/search?q=leptos", Default::default())>
//                 "搜索 Leptos"
//             </button>
//             " "
//             <button on:click=move |_| navigate("/search?q=rust", Default::default())>
//                 "搜索 Rust"
//             </button>
//         </nav>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Router>
//             <SearchButtons/>
//             <main>
//                 <Routes fallback=|| "页面未找到">
//                     <Route path="/search" view=SearchPage/>
//                 </Routes>
//             </main>
//         </Router>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - use_navigate() 的路径中可以包含查询字符串（?key=value）
// - use_query_map() 返回当前 URL 的查询参数映射
// - query().get("key") 获取指定查询参数的值
// - 查询参数变化时，组件自动响应式更新
//
// </details>
