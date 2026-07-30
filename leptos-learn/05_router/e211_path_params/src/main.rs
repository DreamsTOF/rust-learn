// ============================================================
// 练习 211: path_params — 路径参数 (:id)
//
// 目标: 从 URL 路径中读取 `:id` 参数并显示
//
// 难度: ⭐⭐
// 核心知识点: :id 路径参数语法, use_params_map()
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params_map;
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p><a href="/user/42">"查看用户 42"</a></p>
        <p><a href="/user/100">"查看用户 100"</a></p>
    }
}

#[component]
fn User() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 用 use_params_map() 获取路由参数映射表
    // 提示: use_params_map() 返回 Memo<ParamsMap>
    //       然后用 .get("id") 取得 Option<&String>
    let params = use_params_map();
    let id = move || params.get().get("id");

    view! {
        <h2>"用户信息"</h2>
        // TODO: 显示 id，处理 id 不存在的情况
        <p>
            {move || id().map(|s| format!("用户 ID: {}", s)).unwrap_or_else(|| "未指定用户 ID".to_string())}
        </p>
        <p><a href="/">"返回首页"</a></p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {
        <Router>
            <nav>
                <a href="/">"首页"</a>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    // TODO: 添加 /user/:id 路由
                    <Route path=path!("/user/:id") view=User/>
                </Routes>
            </main>
        </Router>
    });
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// #[component]
// fn User() -> impl IntoView {
//     let params = use_params_map();
//     let id = move || params.get().get("id");
//
//     view! {
//         <h2>"用户信息"</h2>
//         <p>{move || id().map(|s| format!("用户 ID: {}", s)).unwrap_or_else(|| "未指定".into())}</p>
//         <p><a href="/">"返回首页"</a></p>
//     }
// }
// ```
//
// ### 知识点
// - `use_params_map()` 返回 `Memo<ParamsMap>`，是响应式的参数映射表
// - `ParamsMap::get("key")` 返回 `Option<String>`，参数不存在时返回 None
// - 路由路径中用 `:id` 声明路径参数，参数名与 get() 的 key 对应
// - 路径参数的值会自动 URL 解码
//
// </details>
