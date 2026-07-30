// ============================================================
// 练习 e192: 模拟服务端 Action
//
// 核心知识点:
//   - CSR 中模拟 Server Action
//   - Action 封装异步操作
//   - 模拟网络请求和处理延时
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

async fn mock_server_request(input: String) -> String {
    // 模拟服务端处理延迟
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(800));
    rx.await.unwrap();
    format!("[服务端响应] 收到数据: {} (长度: {})", input, input.len())
}

#[component]
fn Exercise() -> impl IntoView {
    let (text, set_text) = signal(String::new());

    // TODO: 创建 Action，调用 mock_server_request
    let action = Action::new(|input: &String| {
        let input = input.clone();
        async move { mock_server_request(input).await }
    });

    view! {
        <div>
            <p>"练习 192 — 模拟 Server Action (server_action)"</p>
            <input type="text" placeholder="输入数据"
                on:input=move |ev| set_text(event_target_value(&ev))
                prop:value=move || text.get()
            />
            <button on:click=move |_| { action.dispatch(text.get()); }
                disabled=move || action.pending().get()>
                {move || if action.pending().get() { "发送中..." } else { "发送到服务端" }}
            </button>
            <div>
                {move || action.value().get().map(|v| view! { <p><strong>{v}</strong></p> })}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
