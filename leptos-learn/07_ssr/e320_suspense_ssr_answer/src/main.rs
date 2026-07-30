use leptos::prelude::*;

// ============================================================
// 练习 320: Suspense SSR 集成 — 参考答案
//
// 核心: Suspense fallback 在 SSR 中作为占位符，Resource 数据到达后流式替换
// ============================================================

/// 模拟异步用户数据获取
async fn fetch_user(user_id: u32) -> String {
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    format!("用户 #{}", user_id)
}

/// 带 Suspense 边界的用户资料卡片
#[component]
fn UserProfile(user_id: u32) -> impl IntoView {
    let user_data = Resource::new(
        move || user_id,
        move |id| async move { fetch_user(id).await },
    );

    view! {
        <div style="border: 1px solid #ccc; padding: 8px; margin: 8px 0;">
            <Suspense fallback=move || view! { <p>"正在加载用户..."</p> }>
                <p>"欢迎, " {move || user_data.map(|v| v.clone())}</p>
            </Suspense>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"练习 320: Suspense SSR 集成"</h1>
            <p>"以下用户资料在 SSR 流式输出："</p>
            // ponytail: 每个 UserProfile 创建一个 Suspense 边界
            //          SSR 中 fallback 先输出为占位 HTML
            //          Resource 决议后真实内容流式替换
            //          如果使用 hydrate()，水合后状态保持一致
            <UserProfile user_id=1/>
            <UserProfile user_id=2/>
            <UserProfile user_id=3/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
