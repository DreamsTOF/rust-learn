// ============================================================
// 练习 163: suspense_nested
//
// 目标: Suspense 嵌套使用——外层等待大体内容，内层等待细节内容
//
// 难度: ⭐⭐
// 核心知识点: 嵌套 Suspense
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

/// 模拟加载文章标题（1 秒）
async fn load_title() -> String {
    delay(1000).await;
    "Leptos 入门指南".to_string()
}

/// 模拟加载文章详情（额外 2 秒）
async fn load_detail() -> String {
    delay(2000).await;
    "Suspense 是 Leptos 提供的强大工具...（此处省略 1000 字）".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建外层 Resource（标题）和内层 Resource（详情）
    let title = Resource::new(|| (), |_| async move { load_title().await });
    let detail = Resource::new(|| (), |_| async move { load_detail().await });

    view! {
        <div>
            <h2>"Exercise 163: Nested Suspense"</h2>
            // TODO: 外层 Suspense 等待标题，fallback 显示 "Loading article..."
            <Suspense fallback=|| view! { <p>"Loading article..."</p> }>
                <article>
                    <h3>{move || title.get()}</h3>
                    // TODO: 内层 Suspense 等待详情，fallback 显示 "Loading detail..."
                    <Suspense fallback=|| view! { <p>"Loading detail..."</p> }>
                        <p>{move || detail.get()}</p>
                    </Suspense>
                </article>
            </Suspense>
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
// - 外层 `<Suspense>` 包裹标题和内层 `<Suspense>`
// - 内层 `<Suspense>` 包裹详情内容，拥有自己的 fallback
//
// ### 知识点
// - 嵌套 Suspense 允许不同粒度的加载状态
// - 外层 fallback 显示直到所有外层资源就绪
// - 内层 fallback 在外层内容显示后、内层资源加载期间展示
// - 适合"先显示骨架，再逐步加载细节"的场景
//
// </details>
