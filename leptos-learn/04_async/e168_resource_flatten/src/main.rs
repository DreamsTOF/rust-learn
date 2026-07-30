// ============================================================
// 练习 e168: .flatten() — 展平 Resource (resource_flatten)
//
// 核心知识点:
//   - Suspend::new(async { resource.await }) 展平 Resource
//   - 直接使用异步结果，避免嵌套 Option<Result<...>>
//   - 在视图层"展平"异步数据
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

async fn fetch_user(id: u32) -> Result<String, String> {
    match id {
        1 => Ok("Alice".to_string()),
        2 => Ok("Bob".to_string()),
        _ => Err("用户不存在".to_string()),
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (user_id, set_user_id) = signal(1u32);

    // 创建依赖 user_id 的 Resource
    let user_resource = Resource::new(
        move || user_id.get(),
        |id| async move {
            fetch_user(id).await
        },
    );

    view! {
        <div>
            <h2>"展平 Resource 示例"</h2>
            <button on:click=move |_| set_user_id.set(1)>"Alice"</button>
            <button on:click=move |_| set_user_id.set(2)>"Bob"</button>
            <button on:click=move |_| set_user_id.set(3)>"未知"</button>

            <hr/>

            // TODO: 使用 Suspend::new 和 .await 展平 Resource
            // 使用 Transition 包裹以显示加载状态
            // 提示: Suspend::new(async move { user_resource.await.map(|user| view! { <p>{user}</p> }) })
            <Transition
                fallback=|| view! { <p>"加载中..."</p> }
            >
                {move || Suspend::new(async move {
                    // .await 直接得到 Result<String, String>
                    // 直接返回视图，Resource 的 Option/Result 被"展平"
                    user_resource.await
                        .map(|user| view! { <p>"用户: " {user}</p> }.into_any())
                        .unwrap_or_else(|e| view! { <p class="error">"错误: " {e}</p> }.into_any())
                })}
            </Transition>
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
// ### 代码
// ```rust
// use leptos::prelude::*;
//
// async fn fetch_user(id: u32) -> Result<String, String> {
//     match id {
//         1 => Ok("Alice".to_string()),
//         2 => Ok("Bob".to_string()),
//         _ => Err("用户不存在".to_string()),
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (user_id, set_user_id) = signal(1u32);
//
//     let user_resource = Resource::new(
//         move || user_id.get(),
//         |id| async move { fetch_user(id).await },
//     );
//
//     view! {
//         <div>
//             <h2>"展平 Resource 示例"</h2>
//             <button on:click=move |_| set_user_id.set(1)>"Alice"</button>
//             <button on:click=move |_| set_user_id.set(2)>"Bob"</button>
//             <button on:click=move |_| set_user_id.set(3)>"未知"</button>
//             <hr/>
//             <Transition fallback=|| view! { <p>"加载中..."</p> }>
//                 {move || Suspend::new(async move {
//                     user_resource.await
//                         .map(|user| view! { <p>"用户: " {user}</p> }.into_any())
//                         .unwrap_or_else(|e| view! { <p class="error">"错误: " {e}</p> }.into_any())
//                 })}
//             </Transition>
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
// - Suspend::new(async { ... }) 在 view 中嵌入异步块
// - resource.await 会等待 Resource 完成并返回其内部值
// - 这"展平"了 Resource 的 Option/Result 嵌套，直接操作数据
// - Transition 在等待期间显示 fallback
// - Suspend 内部需要返回实现了 Render 的类型（如 AnyView）
//
// </details>
