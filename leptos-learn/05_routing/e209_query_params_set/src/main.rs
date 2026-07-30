// ============================================================
// 练习 e209: 设置查询参数 (query_params_set)
//
// 目标: 结合 use_navigate() 和 use_query_map()，
//       实现通过输入框自定义搜索词并同步到 URL 查询参数。
//
// 难度: ⭐⭐⭐
// 核心知识点: use_navigate 设置查询参数, 表单与 URL 同步
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::Router;

// 提示: 需要时添加 Routes, Route 等组件导入
// 提示: use_query_map, use_navigate 来自 leptos_router::hooks
// 提示: signal, event_target_value 来自 leptos::prelude

// TODO: 定义 SearchPage 组件（与 e208 类似）
//       1. 使用 use_query_map() 获取当前查询参数
//       2. 读取 "q" 参数值并显示

#[component]
fn SearchPage() -> impl IntoView {
    view! {
        <h2>"搜索页面"</h2>
        // TODO: 显示当前搜索词
    }
}

// TODO: 定义 SearchForm 组件
//       1. 使用 use_navigate() 获取导航函数
//       2. 创建 input 输入框和 "搜索" 按钮
//       3. 点击按钮时用输入内容构造 URL: /search?q=输入内容
//       4. 再创建一个快速搜索 "Leptos" 的按钮

#[component]
fn SearchForm() -> impl IntoView {
    view! {
        <nav>
            // TODO: 输入框 + 搜索按钮
            // TODO: 快速搜索 "Leptos" 按钮
        </nav>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 在 <Router> 中放置 <SearchForm/> 和 <Routes>
    // TODO: 定义 path="/search" 的路由对应 SearchPage

    view! {
        <Router>
            <SearchForm/>
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
// fn SearchForm() -> impl IntoView {
//     let navigate = use_navigate();
//     let (input, set_input) = signal(String::new());
//
//     view! {
//         <nav>
//             <input
//                 type="text"
//                 placeholder="输入搜索关键词"
//                 on:input=move |ev| {
//                     set_input(event_target_value(&ev));
//                 }
//                 prop:value=input
//             />
//             <button on:click=move |_| {
//                 let q = input.get();
//                 if !q.is_empty() {
//                     navigate(&format!("/search?q={}", q), Default::default());
//                 }
//             }>"搜索"</button>
//             " "
//             <button on:click=move |_| navigate("/search?q=leptos", Default::default())>
//                 "搜索 Leptos"
//             </button>
//         </nav>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <Router>
//             <SearchForm/>
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
// - 通过 use_navigate() 在 URL 中设置查询参数
// - signal() 管理输入框状态，event_target_value 获取输入值
// - 动态构造 URL 字符串实现自定义搜索
// - 查询参数同步到 URL 后支持浏览器前进/后退导航
//
// </details>
