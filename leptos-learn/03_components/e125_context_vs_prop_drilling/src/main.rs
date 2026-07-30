// ============================================================
// 练习 e125: Context 替代 Prop Drilling (context_vs_prop_drilling)
//
// 核心知识点:
//   - Context 跨多层组件传递数据，避免逐层传递 props
//   - 中间组件无需感知数据，降低耦合
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

// TODO: 定义 AppConfig，包含多语言和主题配置
#[derive(Clone)]
struct AppConfig {
    language: String,
    theme: String,
}

// TODO: 最深层的组件直接通过 use_context 获取配置
#[component]
fn DeepChild() -> impl IntoView {
    let config = use_context::<AppConfig>()
        .expect("AppConfig should be provided by root");

    view! {
        <div style="border: 1px solid orange; padding: 8px; margin: 8px 0;">
            <h4>"DeepChild (最深层)"</h4>
            <p>"Language: " {config.language.clone()}</p>
            <p>"Theme: " {config.theme.clone()}</p>
        </div>
    }
}

// 中间组件无需接收和传递 AppConfig props
#[component]
fn MiddleChild() -> impl IntoView {
    view! {
        <div style="border: 1px solid blue; padding: 8px; margin: 8px 0;">
            <h3>"MiddleChild (中间层)"</h3>
            <p>"中间层不需要接触 AppConfig"</p>
            <DeepChild/>
        </div>
    }
}

#[component]
fn InnerChild() -> impl IntoView {
    view! {
        <div style="border: 1px solid purple; padding: 8px; margin: 8px 0;">
            <h3>"InnerChild (中间层)"</h3>
            <p>"中间层不需要接触 AppConfig"</p>
            <MiddleChild/>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 在根组件提供 AppConfig，让深层组件直接消费
    provide_context(AppConfig {
        language: "zh-CN".to_string(),
        theme: "dark".to_string(),
    });

    view! {
        <div style="border: 1px solid gray; padding: 8px;">
            <h2>"Context vs Prop Drilling Demo"</h2>
            <p>"AppConfig 在根组件提供"</p>
            <p>"InnerChild → MiddleChild → DeepChild 三层嵌套"</p>
            <p>"中间组件无需传递 props，DeepChild 直接消费"</p>
            <InnerChild/>
        </div>
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
//
// #[derive(Clone)]
// struct AppConfig { language: String, theme: String }
//
// #[component]
// fn DeepChild() -> impl IntoView {
//     let config = use_context::<AppConfig>().expect("...");
//     view! {
//         <div>
//             <p>"Language: " {config.language.clone()}</p>
//             <p>"Theme: " {config.theme.clone()}</p>
//         </div>
//     }
// }
//
// #[component]
// fn MiddleChild() -> impl IntoView {
//     view! { <div><DeepChild/></div> }
// }
//
// #[component]
// fn InnerChild() -> impl IntoView {
//     view! { <div><MiddleChild/></div> }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     provide_context(AppConfig { language: "zh-CN".into(), theme: "dark".into() });
//     view! { <div><InnerChild/></div> }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - Prop drilling：数据经过的每一层都要声明和透传 props
// - Context 方案：根组件 provide_context，任意后代 use_context
// - 中间组件零耦合，不感知数据存在
// - 适合全局或跨多层共享的配置、主题、用户信息等
//
// </details>
