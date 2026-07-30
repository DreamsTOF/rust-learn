// ============================================================
// 练习 e367: 虚拟滚动 — 只渲染可视区域的大列表
//
// 核心知识点:
//   - 固定行高 + 总容器高度占位
//   - on:scroll 事件监听滚动位置
//   - 计算可视范围行索引
//   - 只渲染可视区域内的列表项
//
// 难度: ⭐⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::ev;
use leptos::prelude::*;
use leptos::web_sys;

const ITEM_COUNT: usize = 10000;
const ROW_HEIGHT: f64 = 40.0;
const CONTAINER_HEIGHT: f64 = 500.0;

// 生成模拟数据
fn generate_items() -> Vec<String> {
    (1..=ITEM_COUNT)
        .map(|i| format!("行 #{} — 虚拟滚动项目，索引 {}", i, i))
        .collect()
}

#[component]
fn Exercise() -> impl IntoView {
    let items = generate_items();
    let scroll_top = RwSignal::new(0.0);

    // 计算总内容高度
    let total_height = items.len() as f64 * ROW_HEIGHT;

    // TODO: 计算可见行范围
    // visible_start_idx = floor(scroll_top / ROW_HEIGHT)
    // visible_end_idx = min(visible_start_idx + ceil(CONTAINER_HEIGHT / ROW_HEIGHT) + 1, items.len())
    // visible_items = items[visible_start_idx..visible_end_idx].to_vec()
    let visible_items = RwSignal::new(Vec::<String>::new());
    let offset_y = RwSignal::new(0.0);

    // 处理滚动事件
    let on_scroll = move |ev: ev::Event| {
        // TODO: 从事件目标中获取 scrollTop
        // 使用 event_target() 获取目标元素
        // 然后用 .scroll_top() 获取滚动位置
        // 更新 scroll_top, visible_items, offset_y
    };

    // TODO: 在 view! 中:
    // 1. 外层 div 固定高度 + overflow-y: auto + on:scroll
    // 2. 内层 div 高度 = total_height，用于撑开滚动条
    // 3. 实际渲染的 div 使用 transform: translateY(offset_y) 定位
    // 4. 遍历 visible_items() 渲染

    view! {
        <div style="padding: 1rem; font-family: sans-serif; max-width: 480px; margin: 0 auto;">
            <h3>"练习 e367: 虚拟滚动"</h3>
            <p style="color: #666; font-size: 14px;">
                {format!("共 {} 个项目，仅渲染可见区域", ITEM_COUNT)}
            </p>

            <div style="padding: 8px; background: #f5f5f5; border-radius: 4px; margin-bottom: 8px;">
                {move || format!("滚动位置: {} px", scroll_top())}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
