// ============================================================
// 练习 e194: Action 异步验证
//
// 核心知识点:
//   - 使用 Action 进行异步表单验证
//   - 验证结果表明成功或失败
//   - 异步验证后更新 UI
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

async fn validate_username(name: String) -> Result<String, String> {
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(500));
    rx.await.unwrap();
    if name.len() < 3 {
        Err("用户名至少需要 3 个字符".to_string())
    } else if name == "admin" {
        Err("用户名 'admin' 已被占用".to_string())
    } else {
        Ok(format!("用户名 '{}' 可用！", name))
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (username, set_username) = signal(String::new());

    // TODO: 创建 Action，调用 validate_username
    // 提示: Action::new(|input: &String| { ... })
    let action = Action::new(|input: &String| {
        let input = input.clone();
        async move { validate_username(input).await }
    });

    view! {
        <div>
            <p>"练习 194 — Action 异步验证 (action_async_validation)"</p>
            <input type="text" placeholder="输入用户名"
                on:input=move |ev| set_username(event_target_value(&ev))
                prop:value=move || username.get()
            />
            <button on:click=move |_| { action.dispatch(username.get()); }
                disabled=move || action.pending().get()>
                {move || if action.pending().get() { "验证中..." } else { "验证用户名" }}
            </button>
            <div>
                {move || match action.value().get() {
                    None => view! { <p>"等待验证"</p> }.into_any(),
                    Some(Ok(msg)) => view! { <p style="color: green;">{msg}</p> }.into_any(),
                    Some(Err(err)) => view! { <p style="color: red;">{err}</p> }.into_any(),
                }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
