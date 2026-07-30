use leptos::html::Div;
use leptos::prelude::*;

// TODO e131: NodeRef 回调模式
//
// 使用 NodeRef + Effect 实现元素挂载后的回调：
// 1. 创建 NodeRef<Div>
// 2. 通过 node_ref 属性绑定到元素
// 3. 用 Effect 检测 NodeRef 被填充后执行逻辑

fn main() {
    mount_to_body(|| {
        let (message, set_message) = signal("元素尚未挂载".to_string());
        let div_ref: NodeRef<Div> = NodeRef::new();

        // NodeRef.get() 是响应式的 — 当 ref 被填充时 effect 会重新运行
        Effect::new(move || {
            if div_ref.get().is_some() {
                set_message.set("元素已挂载！Effect 检测到 NodeRef 已填充".to_string());
            }
        });

        view! {
            <p>"练习 131 (node_ref_callback)"</p>
            <div
                node_ref={div_ref}
                style="padding: 16px; border: 1px solid #4CAF50; border-radius: 4px;"
            >
                {message}
            </div>
        }
    });
}
