// ============================================================
// 练习 e356: 离线支持 — 检测网络在线/离线状态并显示提示
//
// 核心知识点:
//   - window_event_listener 监听 online/offline 事件
//   - 使用 Signal 驱动 UI 响应状态变化
//   - 模拟缓存数据并在离线时展示
//
// 难度: ⭐⭐
// ============================================================

use leptos::ev;
use leptos::prelude::*;

/// 模拟的离线缓存数据
const CACHED_ITEMS: &[&str] = &[
    "📄 缓存文档 v2.3",
    "🖼️ 缓存图片 profile.png",
    "📝 缓存笔记「离线工作流」",
    "📊 缓存报告 Q3-2024",
];

#[component]
fn Exercise() -> impl IntoView {
    let (is_online, set_online) = signal(true);

    let _ = window_event_listener(ev::online, move |_| {
        set_online.set(true);
    });

    let _ = window_event_listener(ev::offline, move |_| {
        set_online.set(false);
    });

    view! {
        <div style="max-width: 600px; margin: 2rem auto; font-family: sans-serif;">
            <h1>"🌐 网络状态检测"</h1>

            {move || {
                if is_online.get() {
                    view! {
                        <div style="padding: 1rem; border-radius: 8px; margin: 1rem 0; background: #e8f5e9; color: #2e7d32; font-weight: bold;">
                            "✅ 您当前处于在线状态"
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div style="padding: 1rem; border-radius: 8px; margin: 1rem 0; background: #ffebee; color: #c62828; font-weight: bold;">
                            "❌ 您当前处于离线状态 — 正在显示缓存数据"
                        </div>
                    }.into_any()
                }
            }}

            <h2>"📦 缓存数据"</h2>
            <ul style="list-style: none; padding: 0;">
                {CACHED_ITEMS.iter().map(|item| {
                    view! {
                        <li style="padding: 0.5rem; margin: 0.25rem 0; background: #f5f5f5; border-radius: 4px;">
                            {*item}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
