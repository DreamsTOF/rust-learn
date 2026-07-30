// ============================================================
// 练习 212: optional_params — 可选路径参数
//
// 目标: 通过可选路径参数 `:name?` 实现带/不带名字的问候
//
// 难度: ⭐⭐
// 核心知识点: Option 参数, :name? 语法
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
        <p><a href="/greet">"匿名问候"</a></p>
        <p><a href="/greet/小明">"问候小明"</a></p>
        <p><a href="/greet/小红">"问候小红"</a></p>
    }
}

#[component]
fn Greet() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 用 use_params_map() 获取可选参数 `name`
    // 提示: 路由声明为 /greet/:name? 时，name 为可选
    //       若未提供，get("name") 返回 None
    let params = use_params_map();
    let name = move || params.get().get("name");

    view! {
        <h2>"问候"</h2>
        // TODO: 根据 name 是否提供显示不同的问候语
        <p>
            {move || match name() {
                Some(n) => format!("你好，{}！", n),
                None => "你好，访客！".to_string(),
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
                    // TODO: 添加 /greet/:name? 可选参数路由
                    <Route path=path!("/greet/:name?") view=Greet/>
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
// fn Greet() -> impl IntoView {
//     let params = use_params_map();
//     let name = move || params.get().get("name");
//
//     view! {
//         <h2>"问候"</h2>
//         <p>{move || match name() {
//             Some(n) => format!("你好，{}！", n),
//             None => "你好，访客！".to_string(),
//         }}</p>
//         <p><a href="/">"返回首页"</a></p>
//     }
// }
// ```
//
// ### 知识点
// - 路由路径中 `:name?` 的 `?` 后缀表示该参数可省略
// - 无参数时 `get("name")` 返回 `None`，不会导致 404
// - 可选参数通常作为路由的末尾段，避免歧义
//
// </details>
