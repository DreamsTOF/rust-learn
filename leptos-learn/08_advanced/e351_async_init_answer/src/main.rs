// ============================================================
// 答案 e351: 异步初始化 — LocalResource 模拟异步加载
//
// 完整可编译实现，不含 TODO。
// 展示 LocalResource::new + delay_ms + loading/loaded 状态切换
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// 使用 setTimeout 实现异步延迟
async fn delay_ms(ms: i32) {
    let (sender, receiver) = futures::channel::oneshot::channel::<()>();
    let window = leptos::web_sys::window().unwrap();
    let closure = Closure::once(move || {
        let _ = sender.send(());
    });
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            ms,
        )
        .expect("setTimeout failed");
    closure.forget();
    receiver.await.expect("timer cancelled");
}

#[component]
fn Exercise() -> impl IntoView {
    // 创建 LocalResource，使用 delay_ms 模拟 2 秒网络延迟
    let user_data = LocalResource::new(|| async move {
        delay_ms(2000).await;
        format!("用户名: Alice\n邮箱: alice@example.com")
    });

    view! {
        <div>
            <h2>"答案 e351: 异步初始化 (LocalResource)"</h2>
            {move || match user_data.get() {
                None => view! { <p>"⏳ 加载用户数据中..."</p> }.into_any(),
                Some(data) => view! {
                    <div>
                        <p>"✅ 用户数据加载完成:"</p>
                        <pre>{data}</pre>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
