// ============================================================
// 练习 214: params_parse — use_params() 自动解析
//
// 目标: 用 use_params::<T>() 将路由参数自动解析为结构体
//
// 难度: ⭐⭐
// 核心知识点: use_params(), Params 派生
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::path;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 定义一个结构体包含要解析的参数
// 提示: 需要 #[derive(Params)] (来自 leptos_router::params)
//       字段名对应路由参数名
#[derive(Params, Debug, Clone, PartialEq)]
struct UserParams {
    id: Option<u32>,
    tab: Option<String>,
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"首页"</h2>
        <p><a href="/user/42">"用户 42"</a></p>
        <p><a href="/user/99/posts">"用户 99 的帖子"</a></p>
    }
}

#[component]
fn User() -> impl IntoView {
    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 用 use_params::<UserParams>() 自动解析参数
    // 提示: use_params() 返回 Memo<Result<T, ParamsError>>
    //       需要在 view! 中使用 .ok() 或 match 处理
    let params = use_params::<UserParams>();

    view! {
        <h2>"用户页面"</h2>
        // TODO: 显示解析后的参数
        <p>
            {move || match params.get().as_ref() {
                Ok(p) => format!("用户 ID: {:?}, 标签: {:?}", p.id, p.tab),
                Err(e) => format!("参数解析失败: {}", e),
            }}
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
                    // TODO: 添加 /user/:id/:tab? 路由
                    // 提示: 路径参数名需与结构体字段名对应
                    <Route path=path!("/user/:id/:tab?") view=User/>
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
// #[derive(Params, Debug, Clone, PartialEq)]
// struct UserParams {
//     id: Option<u32>,
//     tab: Option<String>,
// }
//
// #[component]
// fn User() -> impl IntoView {
//     let params = use_params::<UserParams>();
//     view! {
//         <h2>"用户页面"</h2>
//         <p>{move || match params.read().as_ref() {
//             Ok(p) => format!("用户 ID: {:?}, 标签: {:?}", p.id, p.tab),
//             Err(e) => format!("参数解析失败: {}", e),
//         }}</p>
//         <p><a href="/">"返回首页"</a></p>
//     }
// }
// ```
//
// ### 知识点
// - `use_params::<T>()` 基于 Params trait 自动将路径参数反序列化为结构体
// - 结构体字段最好是 `Option<T>` 类型，因为路径参数可能缺失
// - 解析失败返回 `Err(ParamsError)`，需要妥善处理
// - 使用 `use_params()` 比手动 `use_params_map()` + `.get()` 更安全、便捷
//
// </details>
