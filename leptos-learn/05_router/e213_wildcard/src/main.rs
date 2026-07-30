// ============================================================
// 练习 213: wildcard — 通配符匹配
//
// 目标: 使用 `*tail` 通配符匹配剩余路径并显示
//
// 难度: ⭐⭐
// 核心知识点: *tail 通配符语法
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
        <p><a href="/files">"文件列表"</a></p>
        <p><a href="/files/docs/readme.txt">"docs/readme.txt"</a></p>
        <p><a href="/files/images/photo.jpg">"images/photo.jpg"</a></p>
        <p><a href="/files/a/b/c/deep.txt">"a/b/c/deep.txt"</a></p>
    }
}

#[component]
fn Files() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 用 use_params_map() 获取通配符参数 `tail`
    // 提示: 路由声明为 /files/*tail，匹配 /files/ 之后的任意路径
    //       使用 get("tail") 获取完整匹配内容
    let params = use_params_map();
    let tail = move || params.get().get("tail");

    view! {
        <h2>"文件浏览"</h2>
        // TODO: 显示通配符捕获的路径，处理 None 情况（访问 /files 时）
        <p>
            {move || match tail() {
                Some(p) => format!("当前路径: {}", p),
                None => "文件根目录".to_string(),
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
                <a href="/files">"文件"</a>
            </nav>
            <main>
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    // TODO: 添加 /files/*tail 通配符路由
                    // 提示: *tail 捕获 /files/ 之后的所有路径段
                    <Route path=path!("/files/*tail") view=Files/>
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
// fn Files() -> impl IntoView {
//     let params = use_params_map();
//     let tail = move || params.get().get("tail");
//
//     view! {
//         <h2>"文件浏览"</h2>
//         <p>{move || match tail() {
//             Some(p) => format!("当前路径: {}", p),
//             None => "文件根目录".to_string(),
//         }}</p>
//         <p><a href="/">"返回首页"</a></p>
//     }
// }
// ```
//
// ### 知识点
// - `*tail` 通配符捕获 /files/ 之后的所有路径段
// - 通配符参数名（如 `tail`）可自定义，不限于特定名称
// - 访问 /files（无尾部）时通配符参数为 None
// - 常用于文件系统浏览、SEO 友好的多级 URL 等场景
//
// </details>
