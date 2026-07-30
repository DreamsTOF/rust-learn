// ============================================================
// 参考答案 e367: 虚拟滚动 — 只渲染可视区域的大列表
//
// 核心知识点:
//   - 固定行高 + 总容器高度占位
//   - on:scroll 事件监听滚动位置
//   - 计算可视范围行索引
//   - 只渲染可视区域内的列表项
// ============================================================

use leptos::ev;
use leptos::prelude::*;
use leptos::web_sys;

const ITEM_COUNT: usize = 10000;
const ROW_HEIGHT: f64 = 40.0;
const CONTAINER_HEIGHT: f64 = 500.0;
const OVERSCAN: usize = 5;

fn generate_items() -> Vec<String> {
    (1..=ITEM_COUNT)
        .map(|i| format!("行 #{} — 虚拟滚动项目，索引 {}", i, i))
        .collect()
}

#[component]
fn Exercise() -> impl IntoView {
    let items = generate_items();
    let scroll_top = RwSignal::new(0.0);
    let total_height = items.len() as f64 * ROW_HEIGHT;

    let visible_items = RwSignal::new(Vec::<String>::new());
    let offset_y = RwSignal::new(0.0);

    // 初始计算可见范围
    {
        let first = scroll_top() / ROW_HEIGHT;
        let start = first.floor() as usize;
        let visible_count = (CONTAINER_HEIGHT / ROW_HEIGHT).ceil() as usize + OVERSCAN;
        let end = (start + visible_count).min(items.len());
        visible_items.set(items[start..end].to_vec());
        offset_y.set(start as f64 * ROW_HEIGHT);
    }

    // 滚动事件处理
    let on_scroll = move |ev: ev::Event| {
        let target = event_target::<web_sys::HtmlElement>(&ev);
        let st = target.scroll_top() as f64;
        scroll_top.set(st);

        let first = st / ROW_HEIGHT;
        let start = first.floor() as usize;
        let visible_count = (CONTAINER_HEIGHT / ROW_HEIGHT).ceil() as usize + OVERSCAN;
        let end = (start + visible_count).min(items.len());
        visible_items.set(items[start..end].to_vec());
        offset_y.set(start as f64 * ROW_HEIGHT);
    };

    view! {
        <div style="padding: 1rem; font-family: sans-serif; max-width: 480px; margin: 0 auto;">
            <h3>"练习 e367: 虚拟滚动"</h3>
            <p style="color: #666; font-size: 14px;">
                {format!("共 {} 个项目，仅渲染可见区域", ITEM_COUNT)}
            </p>

            <div style="padding: 8px; background: #f5f5f5; border-radius: 4px; margin-bottom: 8px;">
                {move || format!("滚动位置: {} px | 可见项: {}", scroll_top(), visible_items().len())}
            </div>

            <div
                style="height: {CONTAINER_HEIGHT}px; overflow-y: auto; border: 1px solid #ddd; border-radius: 8px;"
                on:scroll=on_scroll
            >
                <div style={format!("height: {}px;", total_height)}>
                    <div style={move || format!("transform: translateY({}px);", offset_y())}>
                        {move || {
                            visible_items()
                                .into_iter()
                                .enumerate()
                                .map(|(_idx, item)| {
                                    view! {
                                        <div
                                            style={format!(
                                                "height: {}px; line-height: {}px; padding: 0 12px; \
                                                 border-bottom: 1px solid #f0f0f0; font-size: 14px; \
                                                 box-sizing: border-box; overflow: hidden;",
                                                ROW_HEIGHT, ROW_HEIGHT
                                            )}
                                        >
                                            {item}
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
