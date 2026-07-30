// ============================================================
// 练习 198: parallel_load — 并行数据加载
//
// 目标: 多个 Resource 并行加载，全部完成后一起渲染
//
// 难度: ⭐⭐⭐⭐
// 核心知识点: 并行数据加载、Suspense
//
// TODO:
//   1. 实现 sleep 工具函数
//   2. 实现 fetch_users / fetch_stats / fetch_activity
//   3. 创建三个 Resource(或 LocalResource) 并行运行
//   4. 用 Suspense 等待全部完成后渲染
// ============================================================

use serde::{Deserialize, Serialize};
use std::time::Duration;
use futures::channel::oneshot;
use leptos::prelude::*;

async fn sleep(ms: u64) {
    let (tx, rx) = oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(ms));
    let _ = rx.await;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Stats {
    total: u32,
    monthly: u32,
    today: u32,
}

async fn fetch_users() -> Vec<String> {
    sleep(800).await;
    vec!["Alice".into(), "Bob".into(), "Charlie".into(), "Diana".into()]
}

async fn fetch_stats() -> Stats {
    sleep(600).await;
    Stats { total: 1250, monthly: 387, today: 42 }
}

async fn fetch_activity() -> Vec<String> {
    sleep(1000).await;
    vec![
        "用户 Alice 完成了购买".into(),
        "用户 Bob 发表了评论".into(),
        "用户 Charlie 登录了系统".into(),
    ]
}

#[component]
fn Exercise() -> impl IntoView {
    // 三个 Resource 同时启动
    let users = Resource::new(move || (), |_| async { fetch_users().await });
    let stats = Resource::new(move || (), |_| async { fetch_stats().await });
    let activity = Resource::new(move || (), |_| async { fetch_activity().await });

    view! {
        <div>
            <h2>"e198: 并行数据加载"</h2>
            <Suspense fallback=|| view! {
                <div>
                    <p>"⏳ 正在并行加载数据… (用户 800ms / 统计 600ms / 活动 1000ms)"</p>
                </div>
            }>
                <div>
                    <h3>"👥 用户列表"</h3>
                    <ul>
                        {move || users.get().map(|u| {
                            u.into_iter().map(|name| view! { <li>{name}</li> }).collect::<Vec<_>>()
                        })}
                    </ul>

                    <h3>"📊 平台统计"</h3>
                    {move || stats.get().map(|s| view! {
                        <table>
                            <tr><td>"总用户"</td><td>{s.total}</td></tr>
                            <tr><td>"月活跃"</td><td>{s.monthly}</td></tr>
                            <tr><td>"今日注册"</td><td>{s.today}</td></tr>
                        </table>
                    })}

                    <h3>"🕐 最近活动"</h3>
                    <ul>
                        {move || activity.get().map(|a| {
                            a.into_iter().map(|act| view! { <li>{act}</li> }).collect::<Vec<_>>()
                        })}
                    </ul>
                </div>
            </Suspense>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 核心思路
// 创建三个独立的 `Resource`(或 `LocalResource`)，各自异步运行。
// `<Suspense>` 追踪这些资源的读取，在所有资源就绪前显示回退内容。
//
// ### 关键代码
// ```rust
// let users = Resource::new(move || (), |_| async { fetch_users().await });
// let stats = Resource::new(move || (), |_| async { fetch_stats().await });
// let activity = Resource::new(move || (), |_| async { fetch_activity().await });
// ```
// `<Suspense fallback=|| view! { "加载中…" }>` 包裹全部内容。
//
// ### 知识点
// - Resource 的异步任务在创建后立即开始执行(不会串行)
// - Suspense 自动追踪子组件中的 Resource 读取
// - 对比: 串行加载(waterfall)  vs  并行加载
//
// </details>
