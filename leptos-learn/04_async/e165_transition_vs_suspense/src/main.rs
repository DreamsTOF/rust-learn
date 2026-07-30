// ============================================================
// 练习 165: transition_vs_suspense
//
// 目标: 对比 <Suspense> 与 <Transition> 在重新加载时的行为差异
//
// 难度: ⭐⭐⭐
// 核心知识点: Suspense vs Transition
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

/// 模拟异步加载
async fn load_page(page: u32) -> String {
    delay(2000).await;
    format!("Page {} content (loaded at {:?})", page, std::time::SystemTime::now())
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建两个独立的信号和 Resource（分别供 Suspense 和 Transition 使用）
    let (suspense_page, set_suspense_page) = signal(1u32);
    let (transition_page, set_transition_page) = signal(1u32);

    let suspense_data = Resource::new(
        move || suspense_page.get(),
        |page| async move { load_page(page).await },
    );
    let transition_data = Resource::new(
        move || transition_page.get(),
        |page| async move { load_page(page).await },
    );

    view! {
        <div>
            <h2>"Exercise 165: Transition vs Suspense"</h2>
            <div style="display: flex; gap: 20px;">
                // --- Suspense 示例 ---
                <div style="flex: 1; padding: 10px; border: 1px solid #e74c3c;">
                    <h3>"&lt;Suspense&gt;"</h3>
                    <button on:click=move |_| set_suspense_page.update(|n| *n += 1)>
                        "Next Page"
                    </button>
                    // TODO: 用 <Suspense> 包裹，加载时显示 fallback
                    <Suspense fallback=|| view! { <p style="color: #e74c3c;">"Suspense: Loading..."</p> }>
                        <p>{move || suspense_data.get()}</p>
                    </Suspense>
                </div>

                // --- Transition 示例 ---
                <div style="flex: 1; padding: 10px; border: 1px solid #27ae60;">
                    <h3>"&lt;Transition&gt;"</h3>
                    <button on:click=move |_| set_transition_page.update(|n| *n += 1)>
                        "Next Page"
                    </button>
                    // TODO: 用 <Transition> 包裹，加载时保留旧内容
                    <Transition fallback=|| view! { <p style="color: #27ae60;">"Transition: Loading..."</p> }>
                        <p>{move || transition_data.get()}</p>
                    </Transition>
                </div>
            </div>
            <hr/>
            <p>"提示: 点击按钮后，Suspense 会清空并显示 fallback，Transition 保留旧内容直到新数据到达。"</p>
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
// - 左右两个独立面板使用相同的 Resource 模式，分别用 Suspense 和 Transition 包裹
// - Suspense 在重新加载时显示 fallback（旧内容消失）
// - Transition 在重新加载时保持旧内容，新数据到达后无缝替换
//
// ### 知识点
// - **Suspense**: 每次重新加载都会回到 fallback，适合首次加载或完全不同的内容
// - **Transition**: 只在首次加载显示 fallback，后续重新加载保持旧 UI，适合页面/路由切换
// - 选择依据: 如果旧 UI 在加载期间仍有价值，用 Transition；如果需要从空白开始，用 Suspense
//
// </details>
