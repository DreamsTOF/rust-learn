// ============================================================
// Exercise 200 - Real World Dashboard
// ============================================================

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use futures::channel::oneshot;
use leptos::prelude::*;

async fn sleep(ms: u64) {
    let (tx, rx) = oneshot::channel::<()>();
    set_timeout(move || { let _ = tx.send(()); }, Duration::from_millis(ms));
    let _ = rx.await;
}

fn pseudo_random() -> f64 {
    static COUNTER: AtomicU32 = AtomicU32::new(1);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let n = n.wrapping_mul(1103515245).wrapping_add(12345);
    (n >> 16) as f64 / 65536.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User { id: u32, name: String, role: String, online: bool, }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DashboardStats { total_users: u32, active_users: u32, revenue: f64, orders_today: u32, }

async fn fetch_users() -> Result<Vec<User>, String> {
    sleep(700).await;
    if pseudo_random() < 0.15 { return Err("用户服务暂时不可用".into()); }
    Ok(vec![
        User { id: 1, name: "Alice".into(), role: "管理员".into(), online: true },
        User { id: 2, name: "Bob".into(), role: "编辑".into(), online: false },
        User { id: 3, name: "Charlie".into(), role: "用户".into(), online: true },
        User { id: 4, name: "Diana".into(), role: "用户".into(), online: true },
        User { id: 5, name: "Eve".into(), role: "编辑".into(), online: false },
    ])
}

async fn fetch_stats() -> Result<DashboardStats, String> {
    sleep(500).await;
    if pseudo_random() < 0.15 { return Err("统计服务暂时不可用".into()); }
    Ok(DashboardStats { total_users: 1250, active_users: 387, revenue: 45280.50, orders_today: 42 })
}

#[component]
fn Exercise() -> impl IntoView {
    let refresh_trigger = RwSignal::new(0u32);

    let users = Resource::new(move || refresh_trigger.get(), |_| async { fetch_users().await });
    let stats = Resource::new(move || refresh_trigger.get(), |_| async { fetch_stats().await });

    let trigger_clone = refresh_trigger;
    set_interval(move || { trigger_clone.update(|v| *v += 1); }, Duration::from_secs(5));

    let manual_refresh = move |_| { refresh_trigger.update(|v| *v += 1); };

    let user_list = move || match users.get() {
        Some(Ok(list)) => list.into_iter().map(|u| {
            let status = if u.online { "🟢 在线" } else { "🔴 离线" };
            view! { <tr><td>{u.id}</td><td>{u.name}</td><td>{u.role}</td><td>{status}</td></tr> }.into_any()
        }).collect::<Vec<_>>(),
        Some(Err(e)) => vec![view! { <tr><td style="color:red">"⚠ 错误: " {e}</td></tr> }.into_any()],
        None => vec![],
    };

    let stats_view = move || match stats.get() {
        Some(Ok(s)) => view! {
            <div>
                <p>"👥 总用户: " {s.total_users}</p>
                <p>"📊 月活跃: " {s.active_users}
                    " (" {move || format!("{:.1}%", s.active_users as f64 / s.total_users as f64 * 100.0)} ")"
                </p>
                <p>"💰 收入: ¥" {move || format!("{:.2}", s.revenue)}</p>
                <p>"📦 今日订单: " {s.orders_today}</p>
            </div>
        }.into_any(),
        Some(Err(e)) => view! { <p style="color:red">"⚠ 统计错误: " {e}</p> }.into_any(),
        None => view! {}.into_any(),
    };

    view! {
        <div>
            <h2>"e200: 📊 综合异步仪表盘"</h2>
            <p>"自动每 5 秒刷新 · 15% 概率模拟服务异常"</p>
            <button on:click=manual_refresh>"🔄 手动刷新"</button>
            <Suspense fallback=|| view! { <p>"⏳ 仪表盘加载中…"</p> }>
                <div>
                    <h3>"统计数据"</h3>
                    {stats_view}
                    <h3>"用户列表"</h3>
                    <table>
                        <thead><tr><th>"ID"</th><th>"名称"</th><th>"角色"</th><th>"状态"</th></tr></thead>
                        <tbody>{user_list}</tbody>
                    </table>
                </div>
            </Suspense>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
