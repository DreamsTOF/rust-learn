use leptos::prelude::*;

// ============================================================
// 练习 316: 顺序流式 SSR (In-Order Streaming) — 参考答案
//
// 核心: 多个 Suspense 边界在 SSR 中按顺序流式发送
// ============================================================

/// 模拟异步数据加载
async fn fetch_data(name: &'static str) -> String {
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    format!("{} 数据已加载", name)
}

/// 单个流式区块：使用 Resource + Suspense 定义边界
#[component]
fn SlowBlock(name: &'static str) -> impl IntoView {
    let data = Resource::new(
        move || (),
        move |_| async move { fetch_data(name).await },
    );

    view! {
        <section>
            <h2>{name}</h2>
            <Suspense fallback=move || view! { <p>"加载中..."</p> }>
                <p>{move || data.map(|v| v.clone())}</p>
            </Suspense>
        </section>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"练习 316: 顺序流式 SSR"</h1>
            <p>"以下区块按顺序加载和发送："</p>
            // ponytail: 顺序由 Suspense 边界排列顺序决定
            //          SSR 框架按文档顺序流式输出每个边界
            <SlowBlock name="区块 A"/>
            <SlowBlock name="区块 B"/>
            <SlowBlock name="区块 C"/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
