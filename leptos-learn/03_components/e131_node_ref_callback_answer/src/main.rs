use leptos::html::Div;
use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        let (message, set_message) = signal("元素尚未挂载".to_string());
        let div_ref: NodeRef<Div> = NodeRef::new();

        Effect::new(move || {
            if div_ref.get().is_some() {
                set_message
                    .set("元素已挂载！Effect 检测到 NodeRef 已填充".to_string());
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
