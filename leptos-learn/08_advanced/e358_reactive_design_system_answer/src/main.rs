// ============================================================
// 练习 e358: 响应式设计系统 — 用 CSS 变量 + Signal 构建响应式设计系统
//
// 核心知识点:
//   - CSS 自定义属性（css var）绑定
//   - 使用 Signal 控制主题切换（亮色/暗色）
//   - 动态更新文档根元素的 CSS 变量
//   - 组件使用设计 Token 渲染
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

/// 亮色主题的 CSS 变量
const LIGHT_THEME_VARS: &str = "\
--bg-primary: #ffffff;\
--bg-secondary: #f5f5f5;\
--text-primary: #1a1a2e;\
--text-secondary: #555;\
--accent: #1a73e8;\
--accent-hover: #1557b0;\
--card-bg: #ffffff;\
--card-shadow: 0 2px 8px rgba(0,0,0,0.1);\
--border-color: #e0e0e0;\
--success: #2e7d32;\
--danger: #c62828;\
--spacing-unit: 8px;\
";

/// 暗色主题的 CSS 变量
const DARK_THEME_VARS: &str = "\
--bg-primary: #121212;\
--bg-secondary: #1e1e1e;\
--text-primary: #e0e0e0;\
--text-secondary: #aaa;\
--accent: #64b5f6;\
--accent-hover: #90caf9;\
--card-bg: #1e1e1e;\
--card-shadow: 0 2px 8px rgba(0,0,0,0.4);\
--border-color: #333;\
--success: #66bb6a;\
--danger: #ef5350;\
--spacing-unit: 8px;\
";

/// 应用 CSS 变量到文档根元素
fn apply_theme(css_vars: &str) {
    let doc = document();
    if let Some(html_elem) = doc.document_element() {
        let _ = html_elem.set_attribute("style", css_vars);
    }
}

#[component]
fn Card(children: Children) -> impl IntoView {
    view! {
        <div style="background: var(--card-bg); box-shadow: var(--card-shadow); border-radius: 12px; padding: calc(var(--spacing-unit) * 3); border: 1px solid var(--border-color);">
            {children()}
        </div>
    }
}

#[component]
fn Button(
    label: &'static str,
    on_click: Option<impl Fn() + 'static>,
) -> impl IntoView {
    view! {
        <button
            on:click=move |_| { if let Some(ref cb) = on_click { cb() } }
            style="padding: calc(var(--spacing-unit) * 2) calc(var(--spacing-unit) * 3); background: var(--accent); color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 1rem; font-weight: 500;"
        >
            {label}
        </button>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (theme, set_theme) = signal("light");

    // 初始化亮色主题
    apply_theme(LIGHT_THEME_VARS);

    Effect::new(move || {
        let current = theme.get();
        if current == "light" {
            apply_theme(LIGHT_THEME_VARS);
        } else {
            apply_theme(DARK_THEME_VARS);
        }
    });

    view! {
        <div style="font-family: system-ui, sans-serif; min-height: 100vh; background: var(--bg-primary); color: var(--text-primary); transition: all 0.3s ease; padding: calc(var(--spacing-unit) * 4);">
            <h1>"🎨 响应式设计系统"</h1>

            <div style="margin: 1rem 0; display: flex; gap: 1rem;">
                <button
                    on:click=move |_| set_theme.set("light")
                    style="padding: 0.5rem 1.5rem; font-size: 1rem; cursor: pointer; border: 2px solid var(--accent); border-radius: 6px; background: var(--bg-secondary); color: var(--text-primary);"
                >
                    "☀️ 亮色主题"
                </button>
                <button
                    on:click=move |_| set_theme.set("dark")
                    style="padding: 0.5rem 1.5rem; font-size: 1rem; cursor: pointer; border: 2px solid var(--accent); border-radius: 6px; background: var(--bg-secondary); color: var(--text-primary);"
                >
                    "🌙 暗色主题"
                </button>
            </div>

            <div style="display: grid; gap: 1.5rem; margin-top: 2rem;">
                <Card>
                    <h2>"设计 Token 展示"</h2>
                    <p>"背景色: var(--bg-primary)"</p>
                    <p>"强调色: var(--accent)"</p>
                    <p>"间距单位: var(--spacing-unit)"</p>
                </Card>

                <Card>
                    <h2>"按钮演示"</h2>
                    <div style="display: flex; gap: 1rem; flex-wrap: wrap;">
                        <Button label="主要按钮" on_click={Some(|| {})} />
                        <button style="padding: 16px 24px; background: var(--success); color: white; border: none; border-radius: 6px; cursor: pointer; font-weight: 500;">
                            "成功按钮"
                        </button>
                        <button style="padding: 16px 24px; background: var(--danger); color: white; border: none; border-radius: 6px; cursor: pointer; font-weight: 500;">
                            "危险按钮"
                        </button>
                    </div>
                </Card>

                <Card>
                    <h2>"排版示例"</h2>
                    <p style="color: var(--text-secondary);">
                        "这是次要文字颜色 (var(--text-secondary)) 的段落。"
                    </p>
                    <hr style="border-color: var(--border-color); margin: 1rem 0;" />
                    <p>"当前边框颜色来自 var(--border-color)。"</p>
                </Card>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
