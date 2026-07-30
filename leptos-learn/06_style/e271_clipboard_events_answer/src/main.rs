// ============================================================
// 练习 271 — 参考答案
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (copied_text, set_copied_text) = signal(String::new());
    let (pasted_text, set_pasted_text) = signal(String::new());
    let (cut_text, set_cut_text) = signal(String::new());

    view! {
        <div style="padding: 20px;">
            <h2>"剪贴板事件"</h2>

            <div>
                <label>"复制演示："</label>
                <input type="text" placeholder="选中文本后按 Ctrl+C"
                    on:copy=move |ev| set_copied_text.set(event_target_value(&ev)) />
                <p>"复制的文本：" {move || copied_text.get()}</p>
            </div>

            <div>
                <label>"粘贴演示："</label>
                <input type="text" placeholder="按 Ctrl+V 粘贴"
                    on:input=move |ev| set_pasted_text.set(event_target_value(&ev)) />
                <p>"粘贴的文本：" {move || pasted_text.get()}</p>
            </div>

            <div>
                <label>"剪切演示："</label>
                <input type="text" placeholder="选中文本后按 Ctrl+X"
                    on:cut=move |ev| set_cut_text.set(event_target_value(&ev)) />
                <p>"剪切的文本：" {move || cut_text.get()}</p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
