use leptos::html::Div;
use leptos::prelude::*;

// TODO e134: 使用 scroll_to() 控制元素滚动
//
// 通过 NodeRef 获取容器元素，调用 scroll_to() / set_scroll_top()
// 实现编程式滚动控制（如滚动到顶部或指定位置）。

fn main() {
    mount_to_body(|| {
        let container_ref: NodeRef<Div> = NodeRef::new();

        view! {
            <p>"练习 134 (scroll_control)"</p>

            <div
                node_ref={container_ref}
                style="
                    width: 260px; height: 120px; overflow-y: auto;
                    border: 1px solid #999; border-radius: 4px;
                    padding: 8px;
                "
            >
                <div style="height: 600px;">
                    <p>"↓ 向下滚动查看效果"</p>
                    <p style="margin-top: 100px;">"第 2 段"</p>
                    <p style="margin-top: 100px;">"第 3 段"</p>
                    <p style="margin-top: 100px;">"第 4 段（底部）"</p>
                </div>
            </div>

            <div style="margin-top: 8px;">
                <button on:click=move |_| {
                    if let Some(el) = container_ref.get() {
                        el.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                }>
                    "滚动到顶部"
                </button>

                <button on:click=move |_| {
                    if let Some(el) = container_ref.get() {
                        el.scroll_to_with_x_and_y(0.0, 100.0);
                    }
                }>
                    "滚动 100px"
                </button>

                <button on:click=move |_| {
                    if let Some(el) = container_ref.get() {
                        el.set_scroll_top(el.scroll_height());
                    }
                }>
                    "滚动到底部"
                </button>
            </div>
        }
    });
}
