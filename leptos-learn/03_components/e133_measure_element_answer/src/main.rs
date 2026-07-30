use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        let (info, set_info) = signal("测量中...".to_string());
        let div_ref: NodeRef<leptos::html::Div> = NodeRef::new();

        Effect::new(move || {
            if let Some(el) = div_ref.get() {
                let w = el.offset_width();
                let h = el.offset_height();
                set_info.set(format!("width: {}px | height: {}px", w, h));
            }
        });

        view! {
            <p>"练习 133 (measure_element)"</p>
            <div
                node_ref={div_ref}
                style="
                    width: 320px; height: 100px;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    border-radius: 8px;
                    display: flex; align-items: center; justify-content: center;
                    color: white; font-size: 14px; font-family: monospace;
                "
            >
                {info}
            </div>
            <p style="color: #666; font-size: 13px;">
                "提示：打开浏览器 DevTools 调整视口，重新加载可看到不同尺寸"
            </p>
        }
    });
}
