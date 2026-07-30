// ============================================================
// 练习 e381: 用户行为分析 — 事件追踪、页面访问记录
//
// 核心知识点:
//   - 使用 RwSignal 存储事件记录
//   - window_event_listener 监听页面可见性
//   - serde 序列化事件数据
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;
use leptos::ev;
use leptos::web_sys;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AnalyticsEvent {
    name: String,
    properties: Vec<(String, String)>,
    timestamp: f64,
}

#[component]
fn Exercise() -> impl IntoView {
    // 创建事件存储信号
    let events = RwSignal::new(Vec::<AnalyticsEvent>::new());

    // track_event: 记录事件到内存并模拟发送到服务器
    let track_event = {
        let events = events.clone();
        move |name: &str, properties: Vec<(String, String)>| {
            // 获取当前时间戳
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64()
                * 1000.0;

            let event = AnalyticsEvent {
                name: name.to_string(),
                properties,
                timestamp,
            };

            // 存储事件
            events.update(|e| e.push(event.clone()));

            // 模拟发送到服务器（console.log 输出）
            leptos::logging::log!(
                "[Analytics] 事件: {} | 属性: {:?} | 时间戳: {}",
                event.name,
                event.properties,
                event.timestamp
            );
        }
    };

    // 监听页面可见性变化
    {
        let track_event = track_event.clone();
        window_event_listener(ev::visibilitychange, move |_| {
            if document().hidden() {
                track_event("page_hidden", vec![]);
            } else {
                track_event("page_visible", vec![]);
            }
        });
    }

    // 记录初始页面访问事件
    track_event("page_view", vec![
        (
            "url".to_string(),
            web_sys::window()
                .and_then(|w| w.location().href().ok())
                .unwrap_or_default(),
        ),
        ("title".to_string(), document().title()),
    ]);

    // 按钮点击处理函数
    let on_button_click = {
        let track_event = track_event.clone();
        move |_| {
            track_event("button_click", vec![
                ("button_id".to_string(), "main_button".to_string()),
                ("label".to_string(), "点击测试".to_string()),
            ]);
        }
    };

    let on_form_submit = {
        let track_event = track_event.clone();
        move |_| {
            track_event("form_submit", vec![
                ("form_id".to_string(), "demo_form".to_string()),
                ("fields_count".to_string(), "3".to_string()),
            ]);
        }
    };

    let on_clear = {
        let track_event = track_event.clone();
        move |_| {
            track_event("clear_log", vec![
                ("action".to_string(), "user_cleared".to_string()),
            ]);
            events.set(Vec::new());
        }
    };

    view! {
        <div>
            <h2>"📊 用户行为分析"</h2>

            <div>
                <h3>"操作面板"</h3>
                <button on:click=on_button_click>"点击测试"</button>
                <button on:click=on_form_submit>"提交表单"</button>
                <button on:click=on_clear>"清除日志"</button>
            </div>

            <div>
                <h3>"事件日志" ({move || events.get().len()} 条)</h3>
                <ul>
                    {move || events.get().iter().rev().map(|event| {
                        view! {
                            <li>
                                <strong>{event.name.clone()}</strong>
                                " @ "
                                {format!("{:.0}", event.timestamp)}
                                <ul>
                                    {event.properties.iter().map(|(k, v)| {
                                        view! { <li><em>{k.clone()}</em>: {v.clone()}</li> }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>

            <div>
                <h3>"事件统计"</h3>
                {move || {
                    let all = events.get();
                    let total = all.len();
                    let button_clicks = all.iter().filter(|e| e.name == "button_click").count();
                    let form_submits = all.iter().filter(|e| e.name == "form_submit").count();
                    let page_views = all.iter().filter(|e| e.name == "page_view").count();
                    let visibility_events = all.iter().filter(|e| e.name.starts_with("page_")).count();
                    view! {
                        <ul>
                            <li>"总事件数: " {total}</li>
                            <li>"按钮点击: " {button_clicks}</li>
                            <li>"表单提交: " {form_submits}</li>
                            <li>"页面/可见性事件: " {visibility_events}</li>
                            <li>"其中 page_view: " {page_views}</li>
                        </ul>
                    }
                }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
