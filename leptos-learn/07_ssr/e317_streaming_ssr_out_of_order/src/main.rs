use leptos::prelude::*;


// ============================================================
// 练习 317: 乱序流式 SSR (Out-of-Order Streaming)
//
// 目标: 实现乱序流式输出，较快数据的区块先发送，较慢的后来替换 placeholder
//
// 核心知识点:
//   - Resource::new() 创建异步数据源
//   - <Suspense> 流式边界 + placeholder 替换
//   - 乱序模式: 各边界独立加载，快者先发
//
// ⭐⭐⭐: 创建多个不同延迟的异步区块
// ============================================================

// TODO: 创建 async fn fetch_with_delay(name: &str, ms: u64) -> String
//   使不同区块有不同的延迟，模拟快/慢数据源

// TODO: 创建 SlowBlock 组件
//   props: name: &'static str, delay_ms: u64
//   内部: Resource 使用不同延迟
//   模板: <section> + <Suspense fallback=...> + data.map()

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"练习 317: 乱序流式 SSR"</h1>
            // TODO: 添加 3 个 SlowBlock，延迟分别为 300ms, 1000ms, 600ms
            // 观察输出顺序与延迟的关系
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
