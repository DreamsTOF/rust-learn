 use leptos::prelude::*;

use leptos::ev;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AnalyticsEvent {
    name: String,
    /// 事件属性列表 (key, value)
    properties: Vec<(String, String)>,
    /// 事件发生时间戳 (毫秒)
    timestamp: f64,
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO 1: 创建信号 events: RwSignal<Vec<AnalyticsEvent>>
    // 用于存储所有已记录的事件
    // let events = RwSignal::new(Vec::new());

    // TODO 2: 实现 track_event 函数
    // 1) 获取当前时间戳 (使用 std::time::SystemTime::now())
    // 2) 创建 AnalyticsEvent 结构体
    // 3) 将事件添加到 events 信号中
    // 4) 使用 leptos::logging::log! 打印事件到控制台
    // let track_event = move |name: &str, properties: Vec<(String, String)>| {
    //     // ... 你的代码 ...
    // };

    // TODO 3: 使用 window_event_listener 监听页面可见性变化
    // 页面隐藏时记录 "page_hidden" 事件，页面显示时记录 "page_visible" 事件
    // 使用 document().hidden() 判断当前可见性
    // window_event_listener(ev::visibilitychange, move |_| {
    //     // ... 你的代码 ...
    // });

    // TODO 4: 在组件初始化时记录 "page_view" 事件
    // 携带 url 和 title 属性

    view! {
        <div>
            <h2>"📊 用户行为分析"</h2>

            <div>
                <h3>"操作面板"</h3>
                // TODO 5: 添加三个按钮，分别触发不同的事件:
                // - "点击测试" 按钮 → 记录 "button_click" 事件
                // - "提交表单" 按钮 → 记录 "form_submit" 事件
                // - "清除日志" 按钮 → 清除所有已记录事件
                // 提示：使用 on:click=move |_| { ... }
            </div>

            <div>
                <h3>"事件日志" (0 条)</h3>
                // TODO 6: 遍历 events 信号，显示每条事件
                // 需要显示事件名称、时间戳和属性列表
                // 提示：使用 move || events.get().iter().rev().map(|event| { view! { ... } }).collect::<Vec<_>>()
            </div>

            <div>
                <h3>"事件统计"</h3>
                // TODO 7: 统计显示不同类型事件的数量
                // 例如：总事件数、按钮点击数、表单提交数、页面访问数
            </div>
        </div>
    }
}

 fn main() {
    mount_to_body(Exercise);
 }
