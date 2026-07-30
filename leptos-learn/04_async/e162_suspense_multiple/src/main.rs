// ============================================================
// 练习 162: suspense_multiple
//
// 目标: 使用一个 <Suspense> 等待多个 Resource 全部加载完成
//
// 难度: ⭐⭐⭐
// 核心知识点: 多个 Resource 配合 <Suspense>
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

/// 模拟加载用户信息（1.5 秒）
async fn load_user() -> String {
    delay(1500).await;
    "Alice".to_string()
}

/// 模拟加载用户分数（2.5 秒）
async fn load_score() -> u32 {
    delay(2500).await;
    95
}

/// 模拟加载用户等级（2 秒）
async fn load_level() -> String {
    delay(2000).await;
    "Gold".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建三个 Resource，分别加载用户、分数和等级
    let user = Resource::new(|| (), |_| async move { load_user().await });
    let score = Resource::new(|| (), |_| async move { load_score().await });
    let level = Resource::new(|| (), |_| async move { load_level().await });

    view! {
        <div>
            <h2>"Exercise 162: Suspense Multiple Resources"</h2>
            // TODO: 用一个 <Suspense> 同时等待所有 Resource
            <Suspense fallback=|| view! { <p>"Loading user data..."</p> }>
                <ul>
                    <li>"User: " {move || user.get()}</li>
                    <li>"Score: " {move || score.get().map(|s| s.to_string())}</li>
                    <li>"Level: " {move || level.get()}</li>
                </ul>
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
// - 多个 Resource 各自独立加载数据
// - 一个 `<Suspense>` 包裹所有 Resource，等待全部加载完成后才显示
//
// ### 知识点
// - `<Suspense>` 会跟踪其内读取的所有 Resource，等待全部 resolve
// - 资源之间是并行加载的（不是串行），总等待时间 ≈ 最慢的资源
// - 适合仪表盘等需要多组数据同时展示的场景
//
// </details>
