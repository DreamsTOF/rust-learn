// ============================================================
// Exercise 179 - Answer
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;
use std::fmt;

#[derive(Debug, Clone)]
struct FetchError {
    context: &'static str,
    detail: &'static str,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.context, self.detail)
    }
}

impl std::error::Error for FetchError {}

async fn fetch_data(should_fail: bool) -> Result<String, FetchError> {
    if should_fail {
        Err(FetchError {
            context: "用户模块",
            detail: "获取用户信息失败：网络超时",
        })
    } else {
        Ok("用户数据加载成功".to_string())
    }
}

#[component]
fn DataDisplay(
    data: ReadSignal<Option<Result<String, FetchError>>>,
) -> impl IntoView {
    view! {
        <div style="border: 1px solid #ccc; padding: 12px; margin: 8px 0;">
            <h3>"数据展示区"</h3>
            <ErrorBoundary fallback=move |errors| {
                view! {
                    <div style="border: 1px solid #d32f2f; padding: 12px; background: #ffebee; border-radius: 4px;">
                        <p style="color: #d32f2f; font-weight: bold;">"异步操作失败"</p>
                        <ul>
                            {move || errors.get().iter().map(|(_, e)| {
                                view! { <li>{e.to_string()}</li> }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                }
            }>
                <p>{move || data().and_then(|r| r.ok())}</p>
            </ErrorBoundary>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (should_fail, set_should_fail) = signal(false);
    let (data, set_data) = signal::<Option<Result<String, FetchError>>>(None);

    let fetch = move || {
        set_data.set(None);
        let fail = should_fail.get();
        spawn_local(async move {
            let result = fetch_data(fail).await;
            set_data.set(Some(result));
        });
    };

    fetch();

    view! {
        <h2>"异步错误上下文处理"</h2>
        <p>"当前模式: " {move || if should_fail() { "错误模式" } else { "正常模式" }}</p>
        <button on:click=move |_| { set_should_fail.update(|v| *v = !*v); fetch(); }>
            {move || if should_fail() { "切换到正常" } else { "切换到错误" }}
        </button>
        <DataDisplay data />
    }
}

fn main() {
    mount_to_body(Exercise);
}
