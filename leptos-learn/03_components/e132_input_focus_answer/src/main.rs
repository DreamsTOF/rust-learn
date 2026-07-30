use leptos::html::Input;
use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        let input_ref: NodeRef<Input> = NodeRef::new();

        Effect::new(move || {
            if let Some(el) = input_ref.get() {
                let _ = el.focus();
            }
        });

        view! {
            <p>"练习 132 (input_focus)"</p>
            <input
                type="text"
                placeholder="这个输入框会自动获得焦点"
                node_ref={input_ref}
                style="padding: 8px; font-size: 16px; width: 300px;"
            />
            <p style="color: #666; font-size: 14px;">
                "页面加载后输入框应自动获得焦点，可直接输入文字"
            </p>
        }
    });
}
