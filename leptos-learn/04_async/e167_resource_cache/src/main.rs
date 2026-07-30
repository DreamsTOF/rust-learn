// ============================================================
// 练习 e167: 缓存 Resource (resource_cache)
//
// 核心知识点:
//   - 使用 RwSignal 缓存 Resource 结果
//   - 避免在依赖未变化时重复请求
//   - 手动控制缓存刷新
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;

// 模拟耗时 API 调用（带计数方便观察）
static mut FETCH_COUNT: i32 = 0;

async fn fetch_data() -> Result<String, String> {
    unsafe { FETCH_COUNT += 1 };
    let count = unsafe { FETCH_COUNT };
    Ok(format!("第 {} 次请求的数据", count))
}

#[component]
fn Exercise() -> impl IntoView {
    // 缓存信号 — 存储已获取的数据
    // TODO: 创建一个 RwSignal<String> 作为缓存，初始值为 String::new()
    let cache: RwSignal<String> = RwSignal::new(String::new());

    // 触发刷新的信号
    let (refresh_trigger, set_refresh) = signal(0u64);

    // TODO: 创建一个 Resource 依赖 refresh_trigger 来获取数据
    // 提示: Resource::new(|| refresh_trigger.get(), |_| async move { ... })
    let resource = Resource::new(
        move || refresh_trigger.get(),
        |_| async move {
            fetch_data().await
        },
    );

    // 当 Resource 数据到达时写入缓存
    // TODO: 使用 Effect 监听 resource 的值，写入 cache
    // 提示: Effect::new(move || { if let Some(Ok(data)) = resource.get() { cache.set(data); } })
    Effect::new(move || {
        if let Some(Ok(data)) = resource.get() {
            cache.set(data);
        }
    });

    let on_refresh = move |_| {
        // TODO: 触发 Resource 重新加载
        // 提示: 递增 refresh_trigger 信号
        set_refresh.update(|n| *n += 1);
    };

    view! {
        <div>
            <h2>"Resource 缓存示例"</h2>
            <p>
                "缓存数据: "
                // 从缓存信号读取，不会触发新的请求
                {move || cache.read().clone()}
            </p>
            <p>
                "最新资源: "
                // Resource 的当前值（可能正在加载）
                {move || resource.get()
                    .map(|r| r.unwrap_or_else(|e| format!("错误: {}", e)))
                    .unwrap_or_else(|| "加载中...".to_string())
                }
            </p>
            <button on:click=on_refresh>
                "刷新数据"
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
// static mut FETCH_COUNT: i32 = 0;
//
// async fn fetch_data() -> Result<String, String> {
//     unsafe { FETCH_COUNT += 1 };
//     let count = unsafe { FETCH_COUNT };
//     Ok(format!("第 {} 次请求的数据", count))
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let cache: RwSignal<String> = RwSignal::new(String::new());
//     let (refresh_trigger, set_refresh) = signal(0u64);
//
//     let resource = Resource::new(
//         move || refresh_trigger.get(),
//         |_| async move { fetch_data().await },
//     );
//
//     Effect::new(move || {
//         if let Some(Ok(data)) = resource.get() {
//             cache.set(data);
//         }
//     });
//
//     let on_refresh = move |_| {
//         set_refresh.update(|n| *n += 1);
//     };
//
//     view! {
//         <div>
//             <h2>"Resource 缓存示例"</h2>
//             <p>"缓存数据: " {move || cache.read().clone()}</p>
//             <p>"最新资源: " {move || resource.get()
//                 .map(|r| r.unwrap_or_else(|e| format!("错误: {}", e)))
//                 .unwrap_or_else(|| "加载中...".to_string())
//             }</p>
//             <button on:click=on_refresh>"刷新数据"</button>
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
// - Resource 的依赖信号不变时不会重新执行 fetcher
// - 通过递增依赖信号的值来触发刷新
// - 使用独立的 RwSignal 作为缓存层，可将 Resource 数据持久化
// - Effect 监听 resource.get() 变化，自动同步到缓存
//
// </details>
