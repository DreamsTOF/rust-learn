// ============================================================
// 练习 e208: 读取查询参数 (query_params)
//
// 目标: 使用 use_query_map() 钩子读取 URL 中的查询参数，
//       实现搜索词在 URL 中同步显示的效果。
//
// 难度: ⭐⭐
// 核心知识点: use_query_map, 查询参数读取
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::Router;

// 提示: 需要时添加 Routes, Route 等组件导入
// 提示: use_query_map, use_navigate 来自 leptos_router::hooks

// TODO: 定义 SearchPage 组件
//       1. 使用 use_query_map() 获取当前查询参数
//       2. 从查询参数中读取 "q" 的值
//       3. 显示当前搜索词

#[component]
fn SearchPage() -> impl IntoView {
    view! {
        <h2>"搜索页面"</h2>
        // TODO: 显示当前搜索词（从查询参数中读取）
        // 提示: query().get("q").cloned().unwrap_or_default()
    }
}

// TODO: 定义 SearchButtons 组件
//       1. 使用 use_navigate() 获取导航函数
//       2. 创建两个按钮，导航到不同搜索词
//       例如: /search?q=leptos, /search?q=rust

#[component]
fn SearchButtons() -> impl IntoView {
    view! {
        <nav>
            // TODO: "搜索 Leptos" 按钮 -> /search?q=leptos
            // TODO: "搜索 Rust" 按钮 -> /search?q=rust
        </nav>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 在 <Router> 中放置 <SearchButtons/> 和 <Routes>
    // TODO: 定义 path="/search" 的路由对应 SearchPage

    view! {
        <Router>
            <SearchButtons/>
            <main>
                // TODO: 添加 <Routes> 和 <Route path="/search" view=SearchPage/>
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
// use leptos_router::hooks::*;
// use leptos_router::path;
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
//     let navigate_rust = navigate.clone();
//
//     view! {
//         <nav>
//             <button on:click=move |_| navigate("/search?q=leptos", Default::default())>
//                 "搜索 Leptos"
//             </button>
//             " "
//             <button on:click=move |_| navigate_rust("/search?q=rust", Default::default())>
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
//                     <Route path=path!("/search") view=SearchPage/>
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
// - use_query_map() 返回响应式的查询参数映射
// - 每次导航到带查询参数的 URL 时 SearchPage 自动更新
// - 查询参数在 URL 中通过 "?key=value" 格式传递
// - use_query_map() 的返回值是 Memo<ParamsMap>，需要调用 () 获取当前值
//
// </details>
