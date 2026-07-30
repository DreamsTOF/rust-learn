// ============================================================
// 练习 e146: hook_di — Hook 依赖注入
//
// 目标: 通过 use_theme() hook 从 context 中读取主题配置
//
// 难度: ⭐⭐
// 核心知识点: use_context, provide_context, Hook 封装
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

/// 主题配置
#[derive(Clone, Debug, PartialEq)]
struct Theme {
    primary: String,
    background: String,
    text: String,
}

/// Hook: 从 context 获取主题配置
fn use_theme() -> Theme {
    use_context::<Theme>().expect("use_theme: Theme not found. Call provide_context first.")
}

/// 使用主题的子组件
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
    // === 步骤 1 — 提供 Theme context ===
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
// struct Theme { primary: String, background: String, text: String }
//
// fn use_theme() -> Theme {
//     use_context::<Theme>().expect("Theme not provided")
// }
//
// #[component]
// fn ThemedBox() -> impl IntoView {
//     let t = use_theme();
//     view! {
//         <div style=format!("background:{};color:{};border:2px solid {}", t.background, t.text, t.primary)>
//             <p>"主色: " {t.primary}</p>
//             <p>"背景: " {t.background}</p>
//             <p>"文字: " {t.text}</p>
//         </div>
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     provide_context(Theme { primary: "#4f46e5".into(), background: "#fff".into(), text: "#1e293b".into() });
//     view! { <ThemedBox /> }
// }
//
// fn main() { mount_to_body(Exercise); }
// ```
//
// ### 知识点
// - `provide_context` 将值注入组件树，子组件及子孙组件可通过 `use_context` 获取
// - 将 `use_context` 封装为 `use_theme()` 就是最简单的 Hook DI 模式
// - 利用 Rust 类型系统保证类型安全——取错类型会在编译期报错
// - `expect` 可在缺失 context 时给出清晰的运行时错误信息
//
// </details>
