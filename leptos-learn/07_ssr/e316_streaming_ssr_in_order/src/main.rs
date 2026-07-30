use leptos::prelude::*;


// ============================================================
// 练习 316: 顺序流式 SSR (In-Order Streaming)
//
// 目标: 实现多个按顺序加载的异步区块，展示流式边界概念
//
// 核心知识点:
//   - Resource::new() 创建异步数据源
//   - <Suspense> 定义流式边界
//   - 顺序模式: 前一个边界完成后才开始下一个
//
// ⭐⭐⭐: 创建组件和 Suspense 边界即可
// ============================================================

// TODO: 创建异步数据加载函数
//   async fn fetch_data(name: &'static str) -> String
//   返回 format!("{} 数据已加载", name)

// TODO: 创建 SlowBlock 组件
//   props: name: &'static str
//   内部: Resource::new(|| (), |_| async { fetch_data(name).await })
//   模板: <section> + <Suspense fallback=...> + data.map()

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"练习 316: 顺序流式 SSR"</h1>
            // TODO: 添加 2-3 个 SlowBlock 组件
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
