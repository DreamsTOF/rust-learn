use leptos::prelude::*;

// 练习: 拖拽排序
//
// 目标: 创建可拖拽排序的列表
//
// 实现提示:
// 1. 使用 RwSignal<Vec<String>> 存储列表项
// 2. 使用 RwSignal<Option<usize>> 记录当前拖拽项的索引
// 3. 列表项设置 draggable="true"
// 4. ondragstart: 记录源索引，设置 data_transfer.effect_allowed
// 5. ondragover: ev.prevent_default() 允许放置
// 6. ondrop: 将源项移动到目标位置
// 7. 使用 move || items.get().iter().enumerate().map(...) 遍历渲染

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 items 信号 (RwSignal<Vec<String>>)
    // TODO: 创建 drag_idx 信号 (RwSignal<Option<usize>>)
    // TODO: 实现 handle_dragstart / handle_dragover / handle_drop

    view! {
        <div style="padding: 1rem;">
            <h2>"拖拽排序"</h2>
            <p>"请实现拖拽排序功能"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
