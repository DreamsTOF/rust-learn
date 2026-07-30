// ============================================================
// Exercise 146 - Answer: Hook 依赖注入 (use_theme)
// ============================================================

use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Theme {
    primary: String,
    background: String,
    text: String,
}

fn use_theme() -> Theme {
    use_context::<Theme>().expect("use_theme: Theme not found. Call provide_context first.")
}

#[component]
fn ThemedBox() -> impl IntoView {
    let theme = use_theme();

    view! {
        <div
            style=format!(
                "background-color:{}; color:{}; border:2px solid {}; padding:20px; border-radius:8px",
                theme.background, theme.text, theme.primary
            )
        >
            <h3>"主题展示"</h3>
            <p>"主色: " {theme.primary.clone()}</p>
            <p>"背景: " {theme.background.clone()}</p>
            <p>"文字: " {theme.text.clone()}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    provide_context(Theme {
        primary: "#4f46e5".into(),
        background: "#ffffff".into(),
        text: "#1e293b".into(),
    });

    view! {
        <div>
            <h2>"e146: Hook 依赖注入"</h2>
            <ThemedBox />
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
