use leptos::prelude::*;
use leptos::ev;

#[component]
fn Exercise() -> impl IntoView {
    let (content, set_content) = signal(String::new());

    // TODO: 使用 window_event_listener(ev::keydown, ...) 监听键盘事件
    //   - Ctrl+S: ev.prevent_default() 后 log 保存内容
    //   - Escape: 清空 content

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
