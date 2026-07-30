use leptos::prelude::*;
use leptos::ev;
use leptos::web_sys;

#[component]
fn Exercise() -> impl IntoView {
    let (content, set_content) = signal(String::new());

    window_event_listener(ev::keydown, move |ev: web_sys::KeyboardEvent| {
        if ev.ctrl_key() && ev.key() == "s" {
            ev.prevent_default();
            leptos::logging::log!("保存: {}", content.get());
        }
        if ev.key() == "Escape" {
            set_content.set(String::new());
            leptos::logging::log!("已取消");
        }
    });

    view! {
        <div style="padding: 1rem;">
            <h2>"键盘快捷键"</h2>
            <p>"Ctrl+S 保存 | Escape 取消"</p>
            <textarea
                rows="4"
                prop:value={content.get()}
                on:input=move |ev| set_content.set(event_target_value(&ev))
                style="width: 100%; max-width: 500px;"
                placeholder="输入内容..."
            ></textarea>
            <p>"内容: " {content.get()}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
