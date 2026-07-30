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
    let (value, set_value) = signal(Ok("正常显示"));

    view! {
        <h2>"自定义 Fallback"</h2>
        <button on:click=move |_| set_value.set(Ok("正常状态"))>
            "恢复正常"
        </button>
        <button on:click=move |_| set_value.set(Err(CrashError("组件崩溃！".into())))>
            "触发错误"
        </button>

        <ErrorBoundary fallback=|_| {
            view! { <p style="color: red; font-weight: bold;">"出错了"</p> }
        }>
            <p>{move || value.get()}</p>
        </ErrorBoundary>
    }
}

fn main() {
    mount_to_body(Exercise);
}
