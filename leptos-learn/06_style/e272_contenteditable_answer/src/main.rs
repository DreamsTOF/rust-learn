// ============================================================
// 练习 272 — 参考答案
// ============================================================

use leptos::prelude::*;
use leptos::web_sys;

#[component]
fn Exercise() -> impl IntoView {
    let (html_content, set_html_content) = signal(String::from("点击此处开始编辑..."));

    let handle_input = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlElement>(&ev);
        set_html_content.set(target.inner_html());
    };

    view! {
        <div style="padding: 20px;">
            <div>
                <h2>"可编辑内容"</h2>
                <p>"下面是一个可编辑的 div，修改内容后下方会实时显示 HTML："</p>
            </div>
            <div contenteditable="true"
                style="border: 1px solid #ccc; padding: 10px; min-height: 80px; margin: 10px 0;"
                on:input=handle_input
            >
                "点击此处开始编辑..."
            </div>
            <div>
                <h3>"当前的 HTML 内容："</h3>
                <pre style="background: #f5f5f5; padding: 10px; border-radius: 4px;">
                    {move || html_content.get()}
                </pre>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
