// ============================================================
// 练习 161: suspense_basic
//
// 目标: 使用 <Suspense> 在异步加载期间显示 fallback
//
// 难度: ⭐⭐
// 核心知识点: <Suspense>, Resource
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use std::time::Duration;

/// 模拟异步加载数据
async fn load_data() -> String {
    let (tx, rx) = futures::channel::oneshot::channel::<()>();
    set_timeout(
        move || {
            let _ = tx.send(());
        },
        Duration::from_secs(2),
    );
    rx.await.unwrap();
    "Hello from Suspense!".to_string()
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建一个 Resource 加载异步数据
    let data = Resource::new(|| (), |_| async move { load_data().await });

    view! {
        <div>
            <h2>"Exercise 161: Suspense Basic"</h2>
            // TODO: 用 <Suspense> 包裹内容，设置 fallback 显示 "Loading..."
            <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                <p>{move || data.get()}</p>
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
// - `Resource::new(|| (), |_| async move { load_data().await })` 创建异步资源
// - `<Suspense fallback=|| view! { <p>"Loading..."</p> }>` 在加载时显示 fallback
//
// ### 知识点
// - `<Suspense>` 包裹异步内容，加载期间显示 fallback，加载完成后自动切换
// - `Resource::new` 第一个参数是源信号（依赖），第二个是异步获取函数
// - `data.get()` 在 Suspense 内返回 `Option<T>`，加载中为 `None`
//
// </details>
