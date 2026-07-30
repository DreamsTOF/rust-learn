// ============================================================
// 练习 e315: SSR 异步渲染 — Suspense + Resource 流式输出
//
// 核心知识点:
//   - Resource::new(|| source, |_| async {}) 异步数据加载
//   - <Suspense fallback> 定义流式边界和占位符
//   - data.map() 读取 Resource 结果（返回 Option<T>）
//   - SSR 流式渲染: async 数据就绪后流式替换占位符 HTML
//
// 难度: ⭐⭐ (关键 TODOs，约 50% 已补全)
// ============================================================

use leptos::prelude::*;

/// 模拟异步用户数据加载
async fn fetch_user(user_id: u32) -> String {
    // TODO: 模拟 300ms~800ms 的网络延迟
    // ⭐⭐ 提示: tokio::time::sleep(Duration::from_millis(...))
    // 返回 format!("用户 #{}", user_id)
    format!("用户 #{}", user_id)
}

/// 单用户卡片 — 独立的 Suspense 流式边界
///
/// 每个 UserCard 在 SSR 中创建一个流式区块:
/// - fallback 先输出为占位 HTML
/// - 数据就绪后真实内容流式替换
// TODO: 创建 UserCard 组件
// ⭐⭐ 提示:
//   #[component]
//   fn UserCard(user_id: u32) -> impl IntoView {
//       let data = Resource::new(
//           move || user_id,
//           |id| async move { fetch_user(id).await },
//       );
//       view! {
//           <div style="border: 1px solid #ccc; padding: 8px; margin: 8px 0;">
//               <Suspense fallback=move || view! { <p>"正在加载用户..."</p> }>
//                   <p>"用户: " {move || data.map(|v| v.clone())}</p>
//               </Suspense>
//           </div>
//       }
//   }

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="max-width: 640px; margin: 24px auto; font-family: system-ui, sans-serif;">
            <h2>"⏳ SSR 异步渲染 — Suspense + Resource"</h2>

            <section>
                <h3>"1. 独立 Suspense 边界"</h3>
                <p>"每个用户卡片是一个独立的流式区块："</p>
                // TODO: 添加 3 个 UserCard 组件 (user_id: 1, 2, 3)
                // ⭐⭐ 提示: <UserCard user_id=1/>
            </section>

            <hr/>

            <section>
                <h3>"2. SSR 异步渲染流程"</h3>
                <ol>
                    <li>"HTTP 请求到达服务端，组件树开始渲染"</li>
                    <li>"Resource source 同步执行，future 在后台等待"</li>
                    <li>"Suspense fallback 作为占位 HTML 先发送到浏览器"</li>
                    // TODO: 补全第 4-5 步
                    // ⭐⭐ 提示:
                    //   4. 异步数据就绪 → 真实 HTML 流式替换 fallback
                    //   5. 所有 Suspense 边界完成后页面完全展示
                    <li>"// TODO: 数据就绪后的行为"</li>
                    <li>"// TODO: 所有边界完成"</li>
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
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"空白或 loading，等待 JS + API"</td>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"fallback HTML 即刻展示"</td>
                    </tr>
                    // TODO: 补全 "数据获取" 对比行
                    // ⭐⭐ 提示: CSR="客户端发请求 → 等待响应" SSR="服务端预取数据 → 流式推送到浏览器"
                    <tr>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"数据获取"</td>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"// TODO"</td>
                        <td style="padding: 6px; border-bottom: 1px solid #eee;">"// TODO"</td>
                    </tr>
                </table>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
