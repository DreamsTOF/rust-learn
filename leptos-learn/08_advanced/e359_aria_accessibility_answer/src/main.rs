// ============================================================
// 练习 e359: ARIA 可访问性 — 在组件中添加 ARIA 属性提升可访问性
//
// 核心知识点:
//   - 使用 attr:aria-* 语法设置 ARIA 属性
//   - role 属性定义元素角色
//   - aria-live 区域实现动态内容播报
//   - 焦点管理和键盘可访问性
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

/// 无可访问性支持的按钮（对比用）
#[component]
fn BadButton() -> impl IntoView {
    view! {
        <div
            style="display: inline-block; padding: 10px 20px; background: #e0e0e0; border-radius: 4px; cursor: pointer;"
            on:click=|_| leptos::logging::log!("BadButton clicked")
        >
            "点击我"
        </div>
    }
}

/// 具有完整 ARIA 支持的按钮
#[component]
fn GoodButton() -> impl IntoView {
    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" || ev.key() == " " {
            leptos::logging::log!("GoodButton activated via keyboard");
            ev.prevent_default();
        }
    };

    view! {
        <div
            role="button"
            attr:aria-label="提交表单"
            tabindex="0"
            style="display: inline-block; padding: 10px 20px; background: #1a73e8; color: white; border-radius: 4px; cursor: pointer; outline: none;"
            on:click=|_| leptos::logging::log!("GoodButton clicked")
            on:keydown=on_keydown
        >
            "提交"
        </div>
    }
}

/// 可访问的 alert 消息组件
#[component]
fn AlertMessage(message: String, is_visible: bool) -> impl IntoView {
    if is_visible {
        view! {
            <div
                role="alert"
                attr:aria-live="polite"
                style="padding: 12px; background: #fff3cd; border: 1px solid #ffc107; border-radius: 4px; margin: 8px 0;"
            >
                {message}
            </div>
        }.into_any()
    } else {
        view! {}.into_any()
    }
}

/// 可访问的表单输入组件
#[component]
fn AccessibleInput(
    label: &'static str,
    #[prop(optional)] described_by: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div style="margin: 8px 0;">
            <label style="display: block; margin-bottom: 4px; font-weight: 500;">{label}</label>
            <input
                type="text"
                attr:aria-label={label}
                attr:aria-describedby={described_by}
                style="padding: 8px 12px; border: 1px solid #ccc; border-radius: 4px; width: 100%; max-width: 300px; box-sizing: border-box;"
            />
            <p style="font-size: 0.85rem; color: #666; margin-top: 4px;" id="input-help">
                "请输入有效内容"
            </p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (alert_msg, set_alert_msg) = signal(String::new());
    let (show_alert, set_show_alert) = signal(false);

    let trigger_alert = move |_| {
        set_alert_msg.set("动态消息：操作成功完成！".to_string());
        set_show_alert.set(true);
    };

    view! {
        <div style="max-width: 700px; margin: 2rem auto; font-family: system-ui, sans-serif; padding: 0 1rem;">
            <h1>"♿ ARIA 可访问性演示"</h1>

            <style>
                {":focus-visible { outline: 3px solid #1a73e8; outline-offset: 2px; }"}
            </style>

            <section style="margin: 2rem 0; padding: 1.5rem; border: 1px solid #e0e0e0; border-radius: 8px;">
                <h2>"❌ 无可访问性支持"</h2>
                <BadButton />
                <p style="color: #888; font-size: 0.85rem; margin-top: 0.5rem;">
                    "此按钮使用 div 模拟，没有 role、tabindex、键盘事件支持——屏幕阅读器无法识别。"
                </p>
            </section>

            <section style="margin: 2rem 0; padding: 1.5rem; border: 1px solid #e0e0e0; border-radius: 8px;">
                <h2>"✅ 具有 ARIA 支持"</h2>
                <GoodButton />
                <p style="color: #888; font-size: 0.85rem; margin-top: 0.5rem;">
                    "此按钮有 role、aria-label、tabindex 和键盘事件——屏幕阅读器与键盘均可操作。"
                </p>
            </section>

            <section style="margin: 2rem 0; padding: 1.5rem; border: 1px solid #e0e0e0; border-radius: 8px;">
                <h2>"📢 aria-live 动态消息"</h2>
                <button on:click=trigger_alert
                    style="padding: 10px 20px; background: #ff9800; color: white; border: none; border-radius: 4px; cursor: pointer;">
                    "触发消息"
                </button>
                {move || {
                    view! {
                        <AlertMessage message={alert_msg.get()} is_visible={show_alert.get()} />
                    }
                }}
            </section>

            <section style="margin: 2rem 0; padding: 1.5rem; border: 1px solid #e0e0e0; border-radius: 8px;">
                <h2>"⌨️ 可访问表单"</h2>
                <AccessibleInput label="用户名" described_by="input-help" />
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
