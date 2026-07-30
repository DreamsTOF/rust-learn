use leptos::prelude::*;

// ============================================================
// 练习 317: 乱序流式 SSR (Out-of-Order Streaming) — 参考答案
//
// 核心: 每个 Suspense 边界独立加载，快数据先发、慢数据后替换 placeholder
// ============================================================

/// 模拟可变延迟的异步数据加载
async fn fetch_with_delay(name: &'static str, delay_ms: u64) -> String {
    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
    format!("{} (延迟 {}ms)", name, delay_ms)
}

/// 单个流式区块：可通过 delay_ms 控制速度
#[component]
fn SlowBlock(name: &'static str, delay_ms: u64) -> impl IntoView {
    let data = Resource::new(
        move || (),
        move |_| {
            let name = name;
            async move { fetch_with_delay(name, delay_ms).await }
        },
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
            <h1>"练习 317: 乱序流式 SSR"</h1>
            <p>"以下区块按完成顺序流式输出（快者先发）："</p>
            // ponytail: 乱序模式下各 Suspense 边界独立流式
            //          300ms 最先完成 → 先发送
            //          600ms 第二完成 → 替换其自身 placeholder
            //          1000ms 最后完成 → 最后替换
            //          在浏览器中会看到快区块优先显示内容
            //          SSR 框架输出时快阻塞的 placeholder 会被
            //          已完成的真实内容替换，页面不必等最慢的
            <SlowBlock name="区块 A（快）" delay_ms=300/>
            <SlowBlock name="区块 B（慢）" delay_ms=1000/>
            <SlowBlock name="区块 C（中）" delay_ms=600/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
