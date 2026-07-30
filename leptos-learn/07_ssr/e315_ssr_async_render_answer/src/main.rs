// ============================================================
// 练习 e315: SSR 异步渲染 — Suspense + Resource 流式输出
//
// 核心知识点:
//   - Resource::new 创建异步数据源，SSR 中服务端预取数据
//   - <Suspense fallback> 定义流式边界，fallback 先作为占位 HTML
//   - data.map() 读取 Resource 结果（返回 Option<T>）
//   - SSR 流式渲染: 异步数据就绪后实时替换占位符
// ============================================================

use leptos::prelude::*;

/// 模拟异步用户数据加载
async fn fetch_user(user_id: u32) -> String {
    // 模拟网络延迟，让流式效果可观察
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    format!("用户 #{}", user_id)
}

/// 单用户卡片 — 独立的 Suspense 流式边界
///
/// 每个 UserCard 在 SSR 中创建一个流式区块:
/// 1. fallback 先输出为占位 HTML
/// 2. Resource 在服务端解析后，真实 HTML 流式替换 fallback
#[component]
fn UserCard(user_id: u32) -> impl IntoView {
    let data = Resource::new(
        move || user_id,
        |id| async move { fetch_user(id).await },
    );

    view! {
        <div style="border: 1px solid #ccc; border-radius: 6px; padding: 12px; margin: 8px 0; background: #fafafa;">
            <Suspense fallback=move || view! { <p style="color: #888;">"⏳ 正在加载用户..."</p> }>
                <p style="font-weight: bold;">
                    "👤 " {move || data.map(|v| v.clone())}
                </p>
            </Suspense>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="max-width: 640px; margin: 24px auto; font-family: system-ui, sans-serif;">
            <h2>"⏳ SSR 异步渲染 — Suspense + Resource"</h2>

            <section>
                <h3>"1. 独立 Suspense 边界"</h3>
                <p>"每个用户卡片是一个独立的流式区块："</p>
                // ponytail: 每个 UserCard 创建一个 Suspense 边界
                //          SSR 中 fallback 先输出为占位 HTML
                //          Resource 决议后真实内容流式替换
                <UserCard user_id=1/>
                <UserCard user_id=2/>
                <UserCard user_id=3/>
            </section>

            <hr/>

            <section>
                <h3>"2. SSR 异步渲染流程"</h3>
                <ol>
                    <li>"HTTP 请求到达服务端，组件树开始渲染"</li>
                    <li>"Resource source 同步执行，future 在后台等待"</li>
                    <li>"Suspense fallback 作为占位 HTML 先发送到浏览器"</li>
                    <li>"异步数据就绪 → 真实内容 HTML 流式替换 fallback"</li>
                    <li>"所有 Suspense 边界完成后，页面完整展示"</li>
                </ol>
            </section>

            <hr/>

            <section>
                <h3>"3. 与传统 CSR 对比"</h3>
                <table style="width: 100%; border-collapse: collapse;">
                    <tr>
                        <th style="text-align: left; border-bottom: 1px solid #ccc; padding: 6px;">"方面"</th>
                        <th style="text-align: left; border-bottom: 1px solid #ccc; padding: 6px;">"CSR"</th>
                        <th style="text-align: left; border-bottom: 1px solid #ccc; padding: 6px;">"SSR + Suspense"</th>
                    </tr>
                    <tr>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"首屏展示"</td>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"空白或 loading，等待 JS 加载 + API 请求"</td>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"fallback HTML 即刻展示，无空白等待"</td>
                    </tr>
                    <tr>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"数据获取"</td>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"客户端发请求 → 等待响应 → 渲染"</td>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"服务端预取数据 → 流式推送到浏览器"</td>
                    </tr>
                    <tr>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"SEO"</td>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"搜索引擎看到空白页面"</td>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"搜索引擎看到完整内容"</td>
                    </tr>
                </table>
            </section>

            <hr/>

            <section>
                <h3>"💡 关键 API"</h3>
                <ul>
                    <li>
                        <code>"Resource::new(|| source, |_| async { ... })"</code>
                        " — 响应式异步数据源"
                    </li>
                    <li>
                        <code>"data.map(|v| v.clone())"</code>
                        " — 读取 Resource 值（返回 Option&lt;T&gt;）"
                    </li>
                    <li>
                        <code>"&lt;Suspense fallback=...&gt;"</code>
                        " — 流式边界，fallback 为加载占位"
                    </li>
                </ul>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
