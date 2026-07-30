// ============================================================
// Exercise 125 - Answer: Context vs Prop Drilling
// ============================================================

use leptos::prelude::*;

#[derive(Clone)]
struct AppConfig {
    language: String,
    theme: String,
}

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
