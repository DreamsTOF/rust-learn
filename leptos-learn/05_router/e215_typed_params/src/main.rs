// ============================================================
// 练习 215: typed_params — derive(Params) 类型安全路由参数
//
// 目标: 使用 #[derive(Params)] 实现类型安全的多参数路由
//
// 难度: ⭐⭐⭐
// 核心知识点: derive(Params), 类型安全路由参数
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::path;

// === 步骤 1 ——————————————————————————————————————————
// TODO: 用 #[derive(Params)] 定义类型安全参数结构体
// 提示: Params 是 leptos_router 提供的 trait
//       会自动为结构体生成参数解析逻辑
#[derive(Params, Debug, Clone, PartialEq)]
struct ProductParams {
    category: String,
    product_id: String,
    section: Option<String>,
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h2>"商店首页"</h2>
        <p><a href="/product/electronics/42">"电子产品 42"</a></p>
        <p><a href="/product/books/7/details">"图书 7 详情"</a></p>
    }
}

#[component]
fn Product() -> impl IntoView {
    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 用 use_params::<ProductParams>() 获取类型安全参数
    // 提示: 返回 Memo<Result<ProductParams, ParamsError>>
    //       使用 .get() 获取当前值
    let params = use_params::<ProductParams>();

    view! {
        <h2>"商品详情"</h2>
        // TODO: 显示各个参数的值
        <div>
            {move || match params.read().as_ref() {
                Ok(p) => {
                    view! {
                        <p>"分类: " {p.category.clone()}</p>
                        <p>"商品ID: " {p.product_id.clone()}</p>
                        <p>"版块: " {p.section.clone().unwrap_or("默认".to_string())}</p>
                    }.into_any()
                }
                Err(e) => view! { <p>"参数错误: " {e.to_string()}</p> }.into_any(),
            }}
        </div>
        <p><a href="/">"返回首页"</a></p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {
        <Router>
            <nav>
                <a href="/">"商店"</a>
            </nav>
            <main>
                // TODO: 定义路由
                // 提示: 路径参数名 (:category, :product_id, :section?)
                //       需与结构体字段名对应
                <Routes fallback=|| "页面未找到">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/product/:category/:product_id/:section?") view=Product/>
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
// struct ProductParams {
//     category: String,
//     product_id: String,
//     section: Option<String>,
// }
//
// #[component]
// fn Product() -> impl IntoView {
//     let params = use_params::<ProductParams>();
//     view! {
//         <h2>"商品详情"</h2>
//         <div>{move || match params.read().as_ref() {
//             Ok(p) => view! {
//                 <p>"分类: " {&p.category}</p>
//                 <p>"商品ID: " {&p.product_id}</p>
//                 <p>"版块: " {p.section.as_deref().unwrap_or("默认")}</p>
//             }.into_any(),
//             Err(e) => view! { <p>"参数错误: " {e.to_string()}</p> }.into_any(),
//         }}</div>
//         <p><a href="/">"返回首页"</a></p>
//     }
// }
// ```
//
// ### 知识点
// - `#[derive(Params)]` 提供编译期类型安全的路由参数解析
// - 字段类型可以是 String, u32 等标准类型，或 Option<T> 可选参数
// - 字段名与路径参数名自动映射
// - 相比手动解析，Params 提供更强的类型保证和更简洁的代码
// - 解析失败时返回 ParamsError，可在 UI 中展示错误信息
//
// </details>
