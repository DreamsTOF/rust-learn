use crate::state::AppState;
use crate::types::ActivityEvent;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn ActivityPage() -> impl IntoView {
    let _state = use_context::<AppState>().expect("AppState not provided");
    let filter_type = RwSignal::new(String::new());
    let filter_user = RwSignal::new(String::new());

    let events = RwSignal::new(vec![
        ActivityEvent {
            id: Uuid::new_v4().to_string(),
            user_id: String::new(),
            username: "系统".to_string(),
            action: "创建".to_string(),
            target_type: "workspace".to_string(),
            target_id: String::new(),
            target_name: "NoteFlow".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        },
    ]);

    let filtered_events = move || {
        // TODO: 练习 - 实现活动事件过滤
        // 提示: 根据 filter_type 和 filter_user 过滤 events，然后按时间戳降序排列
        let mut items = events.get();
        let ft = filter_type.get();
        let fu = filter_user.get();
        if !ft.is_empty() {
            items.retain(|e| e.action == ft);
        }
        if !fu.is_empty() {
            items.retain(|e| e.username.contains(&fu));
        }
        items.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        items
    };

    view! {
        <div class="activity-page">
            <h1>"活动日志"</h1>
            <div class="activity-filters">
                <select
                    prop:value=filter_type
                    on:change=move |ev| { filter_type.set(event_target_value(&ev)); }
                >
                    <option value="">"全部操作"</option>
                    <option value="创建">"创建"</option>
                    <option value="编辑">"编辑"</option>
                    <option value="删除">"删除"</option>
                    <option value="分享">"分享"</option>
                </select>
                <input
                    type="text"
                    placeholder="按用户筛选..."
                    prop:value=filter_user
                    on:input=move |ev| { filter_user.set(event_target_value(&ev)); }
                />
            </div>
            <div class="activity-timeline">
                <For
                    each=filtered_events
                    key=|e| e.id.clone()
                    children=move |event: ActivityEvent| {
                        view! {
                            <div class="activity-item">
                                <div class="activity-icon">
                                    {match event.action.as_str() {
                                        "创建" => "➕",
                                        "编辑" => "✏️",
                                        "删除" => "🗑️",
                                        "分享" => "📤",
                                        _ => "📌",
                                    }}
                                </div>
                                <div class="activity-body">
                                    <span class="activity-user">{event.username.clone()}</span>
                                    <span class="activity-action">{event.action.clone()}</span>
                                    <span class="activity-target">{event.target_name.clone()}</span>
                                    <span class="activity-time">
                                        {chrono::DateTime::from_timestamp(event.timestamp, 0)
                                            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                                            .unwrap_or_default()}
                                    </span>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
