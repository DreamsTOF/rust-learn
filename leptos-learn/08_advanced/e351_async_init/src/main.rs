// ============================================================
// 练习 e351: 异步初始化 — 用 LocalResource 模拟异步加载用户数据
//
// 核心知识点:
//   - LocalResource::new(fetcher) 创建纯客户端异步资源
//   - .get() 同步读取 Option<T>，None=加载中，Some=已完成
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// 使用 web_sys::setTimeout 实现异步延迟，模拟网络请求
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
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 使用 LocalResource::new 创建异步资源
    //   - 在异步闭包中调用 delay_ms(2000) 模拟 2 秒延迟
    //   - 返回用户数据字符串（如 "用户名: Alice\n邮箱: alice@example.com"）
    let user_data = LocalResource::new(|| async move {
        delay_ms(2000).await;
        format!("用户名: Alice\n邮箱: alice@example.com")
    });

    view! {
        <div>
            <h2>"练习 e351: 异步初始化 (LocalResource)"</h2>
            // TODO: 使用 user_data.get() 读取资源值
            //   - None → 显示 "⏳ 加载用户数据中..."
            //   - Some(data) → 显示用户数据
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

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 完整代码
// ```rust
// use leptos::prelude::*;
// use wasm_bindgen::closure::Closure;
// use wasm_bindgen::JsCast;
//
// async fn delay_ms(ms: i32) {
//     let (sender, receiver) = futures::channel::oneshot::channel::<()>();
//     let window = leptos::web_sys::window().unwrap();
//     let closure = Closure::once(move || {
//         let _ = sender.send(());
//     });
//     window
//         .set_timeout_with_callback_and_timeout_and_arguments_0(
//             closure.as_ref().unchecked_ref(),
//             ms,
//         )
//         .expect("setTimeout failed");
//     closure.forget();
//     receiver.await.expect("timer cancelled");
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let user_data = LocalResource::new(|| async move {
//         delay_ms(2000).await;
//         format!("用户名: Alice\n邮箱: alice@example.com")
//     });
//
//     view! {
//         <div>
//             <h2>"练习 e351: 异步初始化 (LocalResource)"</h2>
//             {move || match user_data.get() {
//                 None => view! { <p>"⏳ 加载用户数据中..."</p> }.into_any(),
//                 Some(data) => view! {
//                     <div>
//                         <p>"✅ 用户数据加载完成:"</p>
//                         <pre>{data}</pre>
//                     </div>
//                 }.into_any(),
//             }}
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - `LocalResource::new(fetcher)` 只需一个 fetcher 闭包，无需 source
// - `.get()` 返回 `Option<T>`，None=加载中 / Some=就绪
// - 通过 `setTimeout` + `Closure::once` + `oneshot` 实现 WASM 异步延迟
// - `Closure::forget()` 防止回调被过早 drop
//
// </details>
