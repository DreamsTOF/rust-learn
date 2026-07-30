// ============================================================
// Exercise 178 - Error Throw
// ============================================================

use leptos::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
struct ValidationError(&'static str);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "验证失败: {}", self.0)
    }
}

impl std::error::Error for ValidationError {}

#[component]
fn DeepChild(
    value: ReadSignal<Result<&'static str, ValidationError>>,
    set_value: WriteSignal<Result<&'static str, ValidationError>>,
) -> impl IntoView {
    view! {
        <div style="border: 1px solid gray; padding: 8px; margin: 4px;">
            <p>"深层子组件"</p>
            <p>"当前值: " {move || value.get().ok()}</p>
            <button on:click=move |_| {
                set_value.set(Err(ValidationError("深层组件触发的错误！")));
            }>"触发错误"</button>
            <button on:click=move |_| {
                set_value.set(Ok("已恢复"));
            }>"清除错误"</button>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (value, set_value) = signal(Ok("初始状态"));

    view! {
        <h2>"错误传播 — 任意层级触发"</h2>
        <ErrorBoundary fallback=|errors| {
            view! {
                <div style="border: 2px solid red; padding: 12px; margin: 8px 0; background: #fff0f0;">
                    <p style="color: red; font-weight: bold;">"⚠ 错误被 ErrorBoundary 捕获"</p>
                    <ul>
                        {move || errors.get().iter().map(|(_, e)| view! { <li>{e.to_string()}</li> }).collect::<Vec<_>>()}
                    </ul>
                    <button on:click=move |_| set_value.set(Ok("已手动恢复"))>"恢复"</button>
                </div>
            }
        }>
            <p>"外层组件"</p>
            <DeepChild value set_value />
        </ErrorBoundary>
    }
}

fn main() {
    mount_to_body(Exercise);
}
