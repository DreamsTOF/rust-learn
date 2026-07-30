// ============================================================
// 练习 e170: 组合多个 Resource (resource_combine)
//
// 核心知识点:
//   - 多个 Resource 各自独立加载数据
//   - 使用 derived 信号或 Memo 组合多个资源结果
//   - 处理不同加载时间的数据合并
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

async fn fetch_user_name(user_id: u32) -> String {
    match user_id {
        1 => "Alice".to_string(),
        2 => "Bob".to_string(),
        _ => "访客".to_string(),
    }
}

async fn fetch_user_score(user_id: u32) -> u32 {
    match user_id {
        1 => 95,
        2 => 87,
        _ => 0,
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (user_id, set_user_id) = signal(1u32);

    // TODO: 创建第一个 Resource 获取用户名
    // 提示: Resource::new(move || user_id.get(), |id| async move { fetch_user_name(id).await })
    let name_resource = Resource::new(
        move || user_id.get(),
        |id| async move { fetch_user_name(id).await },
    );

    // TODO: 创建第二个 Resource 获取用户分数
    let score_resource = Resource::new(
        move || user_id.get(),
        |id| async move { fetch_user_score(id).await },
    );

    // TODO: 使用 derived signal 组合两个资源的结果
    // 提示: 创建一个 Memo 或闭包，在 name_resource 和 score_resource 都有值时组合
    let combined = move || {
        match (name_resource.get(), score_resource.get()) {
            (Some(name), Some(score)) => {
                format!("用户: {} | 分数: {}", name, score)
            }
            _ => "加载中...".to_string(),
        }
    };

    view! {
        <div>
            <h2>"组合多个 Resource"</h2>
            <button on:click=move |_| set_user_id.set(1)>"Alice"</button>
            <button on:click=move |_| set_user_id.set(2)>"Bob"</button>
            <button on:click=move |_| set_user_id.set(3)>"访客"</button>

            <p>{combined}</p>
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
// async fn fetch_user_name(user_id: u32) -> String {
//     match user_id {
//         1 => "Alice".to_string(),
//         2 => "Bob".to_string(),
//         _ => "访客".to_string(),
//     }
// }
//
// async fn fetch_user_score(user_id: u32) -> u32 {
//     match user_id {
//         1 => 95,
//         2 => 87,
//         _ => 0,
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (user_id, set_user_id) = signal(1u32);
//
//     let name_resource = Resource::new(
//         move || user_id.get(),
//         |id| async move { fetch_user_name(id).await },
//     );
//
//     let score_resource = Resource::new(
//         move || user_id.get(),
//         |id| async move { fetch_user_score(id).await },
//     );
//
//     let combined = move || {
//         match (name_resource.get(), score_resource.get()) {
//             (Some(name), Some(score)) => {
//                 format!("用户: {} | 分数: {}", name, score)
//             }
//             _ => "加载中...".to_string(),
//         }
//     };
//
//     view! {
//         <div>
//             <h2>"组合多个 Resource"</h2>
//             <button on:click=move |_| set_user_id.set(1)>"Alice"</button>
//             <button on:click=move |_| set_user_id.set(2)>"Bob"</button>
//             <button on:click=move |_| set_user_id.set(3)>"访客"</button>
//             <p>{combined}</p>
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
// - 多个 Resource 可以共享相同的依赖信号
// - 每个 Resource 独立管理自己的加载状态
// - 使用 derived signal（闭包）组合多个 .get() 结果
// - 当所有资源都加载完成时组合显示，否则显示加载状态
// - 为保持响应式，闭包中需调用 .get() 或 .track()
//
// </details>
