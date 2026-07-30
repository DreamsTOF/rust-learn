// ============================================================
// 练习 159: resource_error — 错误处理
//
// 目标: 处理 Resource 加载错误（使用 Result<T, E> 作为资源类型）
//
// 难度: ⭐⭐
// 核心知识点: Resource 与 Result 结合的错误处理
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

/// 模拟异步获取数据，返回 Result 类型
async fn fetch_toggle(should_error: bool) -> Result<String, String> {
    if should_error {
        Err("数据加载失败！".to_string())
    } else {
        Ok("数据加载成功 ✅".to_string())
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建信号 trigger，初始为 false
    let (trigger, set_trigger) = signal(false);

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 创建 Resource<Result<String, String>>
    //   当 trigger 变化时自动重新加载
    let data = Resource::new(
        move || trigger(),
        move |val| async move { fetch_toggle(val).await },
    );

    view! {
        <div>
            <p>"练习 159: resource_error — 错误处理"</p>
            <p>"当前模式: " {move || if trigger() { "错误模式" } else { "正常模式" }}</p>
            // TODO: 分别处理 Ok 和 Err 情况
            <p>
                {move || match data.map(|r| r.clone()) {
                    Some(Ok(value)) => value.into_any(),
                    Some(Err(e)) => format!("❌ {}", e).into_any(),
                    None => "加载中...".into_any(),
                }}
            </p>
            // TODO: 添加按钮切换 trigger，触发资源重新加载
            <button on:click=move |_| set_trigger.update(|v| *v = !*v)>
                "切换模式"
            </button>
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
// async fn fetch_toggle(should_error: bool) -> Result<String, String> {
//     if should_error {
//         Err("数据加载失败！".to_string())
//     } else {
//         Ok("数据加载成功 ✅".to_string())
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (trigger, set_trigger) = signal(false);
//
//     let data = Resource::new(
//         move || trigger(),
//         move |val| async move { fetch_toggle(val).await },
//     );
//
//     view! {
//         <div>
//             <p>"练习 159: resource_error — 错误处理"</p>
//             <p>"当前模式: " {move || if trigger() { "错误模式" } else { "正常模式" }}</p>
//             <p>
//                 {move || match data.map(|r| r.clone()) {
//                     Some(Ok(value)) => value.into_any(),
//                     Some(Err(e)) => format!("❌ {}", e).into_any(),
//                     None => "加载中...".into_any(),
//                 }}
//             </p>
//             <button on:click=move |_| set_trigger.update(|v| *v = !*v)>
//                 "切换模式"
//             </button>
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
// - Resource 的类型可以是 `Result<T, E>`，天然支持错误处理
// - `.map()` 返回 `Option<Result<T, E>>`，需要嵌套匹配
// - 错误不会导致程序崩溃，可以优雅显示错误信息
// - `Resource<Result<T, E>>` 还提供了 `.and_then()` 方法简化 Ok 分支的处理
//
// </details>
