// ============================================================
// 练习 320: Suspense SSR 集成
//
// 目标: 在 SSR 中使用 <Suspense> 实现异步数据的流式加载
//
// 核心知识点:
//   - <Suspense> fallback 在 SSR 中作为占位符
//   - Resource::new() 服务端触发异步数据加载
//   - 流式边界: Suspense 定义哪些部分可以流式替换
//
// ⭐⭐: 实现 Resource + Suspense 组合
// ============================================================

use leptos::prelude::*;

// TODO: 创建 async fn fetch_user(id: u32) -> String
//   模拟用户 API 调用，返回 "用户 #ID" 格式

// TODO: 创建 UserProfile 组件
//   props: user_id: u32
//   内部: Resource::new 根据 user_id 加载数据
//   模板: <div> + <Suspense fallback=...> + data.map()

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"练习 320: Suspense SSR 集成"</h1>
            // TODO: 添加 2 个 UserProfile 组件
            //   user_id: 1, 2
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
