// ============================================================
// 练习 e128: Context Default — use_context 默认值处理
//
// 核心知识点:
//   - use_context() 返回 Option<T>
//   - unwrap_or_default / unwrap_or_else 兜底
//
// 难度: ⭐⭐ (补全关键位置)
// ============================================================

use leptos::prelude::*;

#[derive(Clone, Default)]
struct AppConfig {
    title: String,
    version: u32,
}

#[component]
fn Child() -> impl IntoView {
    // TODO: use_context 返回 Option，提供默认值兜底
    let config = use_context::<AppConfig>().unwrap_or_default();

    view! {
        <p>"标题: " {config.title}</p>
        <p>"版本: v" {config.version}</p>
    }
}

#[component]
fn WithConfig() -> impl IntoView {
    provide_context(AppConfig {
        title: String::from("Leptos App"),
        version: 1,
    });
    view! { <Child/> }
}

#[component]
fn WithoutConfig() -> impl IntoView {
    view! { <Child/> }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <h2>"use_context 默认值兜底"</h2>
        <div>
            <h3>"有 Context 提供"</h3>
            <WithConfig/>
        </div>
        <hr/>
        <div>
            <h3>"无 Context 提供 (走默认值)"</h3>
            <WithoutConfig/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案 (去除注释后的纯净版本):
//
// use leptos::prelude::*;
//
// #[derive(Clone, Default)]
// struct AppConfig {
//     title: String,
//     version: u32,
// }
//
// #[component]
// fn Child() -> impl IntoView {
//     let config = use_context::<AppConfig>().unwrap_or_default();
//     view! {
//         <p>"标题: " {config.title}</p>
//         <p>"版本: v" {config.version}</p>
//     }
// }
//
// #[component]
// fn WithConfig() -> impl IntoView {
//     provide_context(AppConfig { title: String::from("Leptos App"), version: 1 });
//     view! { <Child/> }
// }
//
// #[component]
// fn WithoutConfig() -> impl IntoView {
//     view! { <Child/> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     view! {
//         <h2>"use_context 默认值兜底"</h2>
//         <div><h3>"有 Context 提供"</h3><WithConfig/></div>
//         <hr/>
//         <div><h3>"无 Context 提供 (走默认值)"</h3><WithoutConfig/></div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - use_context::<T>() 返回 Option<T>，而非直接 Panic
// - unwrap_or_default / unwrap_or_else 可在缺失时优雅兜底
// - 适用于可选全局配置
// </details>
