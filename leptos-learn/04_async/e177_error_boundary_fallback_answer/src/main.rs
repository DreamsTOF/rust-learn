// ============================================================
// Exercise 177 - Answer
// ============================================================

use leptos::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
struct CrashError(String);

impl fmt::Display for CrashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CrashError {}

#[component]
fn Exercise() -> impl IntoView {
    let (value, set_value) = signal(Ok("正常状态"));

    view! {
        <h2>"可展开的 ErrorBoundary"</h2>
        <button on:click=move |_| set_value.set(Ok("正常状态"))>
            "恢复正常"
        </button>
        <button on:click=move |_| {
            set_value.set(Err(CrashError("发生了一个不可恢复的错误！请检查输入数据。".into())))
        }>
            "触发错误"
        </button>

        <ErrorBoundary fallback=|errors| {
            let errors = errors.clone();
            view! {
                <div style="border: 1px solid red; padding: 8px; margin: 8px 0;">
                    <p style="color: red; font-weight: bold;">"操作失败"</p>
                    <details>
                        <summary>"点击查看错误详情"</summary>
                        <ul>
                            {move || errors.read().iter().map(|(_, e)| {
                                view! { <li>{e.to_string()}</li> }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </details>
                </div>
            }
        }>
            <p>{move || value.get()}</p>
        </ErrorBoundary>
    }
}

fn main() {
    mount_to_body(Exercise);
}
