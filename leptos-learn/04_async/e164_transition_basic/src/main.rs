// ============================================================
// 练习 164: transition_basic
//
// 目标: 使用 <Transition> 在数据重新加载时保留旧 UI
//
// 难度: ⭐⭐⭐
// 核心知识点: <Transition>
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

/// 延迟指定毫秒数
async fn delay(ms: u64) {
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    set_timeout(
        move || {
            let _ = tx.send(());
        },
        Duration::from_millis(ms),
    );
    rx.await.unwrap();
}

/// 模拟根据 ID 加载数据（2 秒延迟）
async fn fetch_item(id: u32) -> String {
    delay(2000).await;
    format!("Item #{}: some data loaded at {:?}", id, std::time::SystemTime::now())
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建一个信号来切换数据 ID
    let (id, set_id) = signal(1u32);

    // TODO: 创建依赖 id 的 Resource，id 变化时会重新加载
    let data = Resource::new(
        move || id.get(),
        |id| async move { fetch_item(id).await },
    );

    view! {
        <div>
            <h2>"Exercise 164: Transition Basic"</h2>
            <p>"Current ID: " {move || id.get()}</p>
            <button on:click=move |_| set_id.update(|n| *n += 1)>
                "Load Item " {move || id.get() + 1}
            </button>
            <hr/>
            // TODO: 用 <Transition> 代替 <Suspense>，观察旧内容保留效果
            <Transition fallback=|| view! { <p>"Loading..."</p> }>
                <div style="padding: 8px; border: 1px solid #ccc;">
                    {move || data.get()}
                </div>
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
// ### 关键代码
// - `Resource::new(move || id.get(), |id| async move { fetch_item(id).await })` 依赖信号
// - `<Transition fallback=|| view! { <p>"Loading..."</p> }>` 保持旧 UI
//
// ### 知识点
// - `<Transition>` 与 `<Suspense>` 的区别：Transition 在重新加载时保留旧内容
// - 初次加载时 Transition 也会显示 fallback
// - 适合导航切换、列表筛选等不希望 UI 闪烁的场景
//
// </details>
