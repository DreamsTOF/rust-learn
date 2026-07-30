// ============================================================
// 练习 e193: 无限滚动 (Infinite Scroll)
//
// 目标: 监听滚动事件，当用户滚动到页面底部时自动加载更多数据
//
// 难度: ⭐⭐⭐⭐
// 核心知识点: window_event_listener、滚动检测、动态加载
//
// TODO: 补全代码，实现滚动到底部自动加载更多项目的功能
// ============================================================

use leptos::ev;
use leptos::prelude::*;
use leptos::web_sys;
use std::time::Duration;

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(Vec::<String>::new());
    let (loading, set_loading) = signal(false);
    let (has_more, set_has_more) = signal(true);
    let page = RwSignal::new(0u32);

    // 加载一批数据（模拟 API 调用）
    let load_batch = {
        let set_items = set_items.clone();
        let set_loading = set_loading.clone();
        let set_has_more = set_has_more.clone();
        move || {
            if loading() || !has_more() {
                return;
            }
            set_loading(true);
            let p = page();
            page.set(p + 1);

            set_timeout(
                move || {
                    let start = items().len() + 1;
                    let mut batch: Vec<String> = (start..start + 8)
                        .map(|i| format!("项目 #{} — 这是第 {} 项数据", i, i))
                        .collect();
                    set_items.update(|v| v.append(&mut batch));
                    set_loading(false);
                    if items().len() >= 60 {
                        set_has_more(false);
                    }
                },
                Duration::from_millis(600),
            );
        }
    };

    // 初始加载
    load_batch();

    // 监听窗口滚动事件
    window_event_listener(ev::scroll, move |_| {
        if loading() || !has_more() {
            return;
        }
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(el) = doc.document_element() {
                    let scroll_top = win.scroll_y().unwrap_or(0.0);
                    let client_h = win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let scroll_h = el.scroll_height() as f64;
                    // 距离底部 150px 时触发加载
                    if scroll_top + client_h >= scroll_h - 150.0 {
                        load_batch();
                    }
                }
            }
        }
    });

    view! {
        <div style="padding: 1rem; font-family: sans-serif; max-width: 480px;">
            <h3>"练习 e193: 无限滚动"</h3>
            <p style="color: #666; font-size: 14px;">"向下滚动以加载更多数据"</p>

            <div style="border: 1px solid #eee; border-radius: 6px; padding: 8px 12px;">
                {move || {
                    items()
                        .into_iter()
                        .enumerate()
                        .map(|(_idx, item)| {
                            view! {
                                <div
                                    style="padding: 10px 8px; border-bottom: 1px solid #f0f0f0;"
                                >{item}</div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>

            <div style="text-align: center; padding: 12px; color: #888;">
                {move || {
                    if loading() {
                        view! { <span>"⏳ 加载中..."</span> }.into_any()
                    } else if !has_more() {
                        view! { <span>"— 已加载全部数据 —"</span> }.into_any()
                    } else {
                        view! { <span>"继续向下滚动..."</span> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
