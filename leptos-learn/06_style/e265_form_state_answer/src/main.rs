// ============================================================
// Exercise 265 - Answer
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let dirty = RwSignal::new(false);
    let submitted = RwSignal::new(false);
    let validating = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        validating.set(true);
        // 模拟验证过程
        let n = name.get();
        let e = email.get();
        if !n.is_empty() && !e.is_empty() && e.contains('@') {
            submitted.set(true);
        }
        validating.set(false);
    };

    let reset = move |_| {
        name.set(String::new());
        email.set(String::new());
        dirty.set(false);
        submitted.set(false);
        validating.set(false);
    };

    view! {
        <div>
            <h2>"表单状态跟踪"</h2>
            <form on:submit=on_submit>
                <div>
                    <label>"姓名："</label>
                    <input
                        type="text"
                        prop:value=name
                        on:input=move |ev| {
                            name.set(event_target_value(&ev));
                            dirty.set(true);
                        }
                    />
                </div>
                <div>
                    <label>"邮箱："</label>
                    <input
                        type="text"
                        prop:value=email
                        on:input=move |ev| {
                            email.set(event_target_value(&ev));
                            dirty.set(true);
                        }
                    />
                </div>
                <button type="submit">"提交"</button>
                <button type="button" on:click=reset>"重置"</button>
            </form>
            <div style="margin-top: 12px; padding: 8px; border: 1px solid #ccc;">
                <p>
                    "状态："
                    {move || if validating.get() {
                        view! { <span style="color: orange">"验证中..."</span> }.into_any()
                    } else if submitted.get() {
                        view! { <span style="color: green">"已提交"</span> }.into_any()
                    } else if dirty.get() {
                        view! { <span style="color: blue">"已修改（未提交）"</span> }.into_any()
                    } else {
                        view! { <span style="color: gray">"未修改"</span> }.into_any()
                    }}
                </p>
                <p>"脏状态（Dirty）：" {move || if dirty.get() { "是" } else { "否" }}</p>
                <p>"已提交（Submitted）：" {move || if submitted.get() { "是" } else { "否" }}</p>
                <p>"验证中（Validating）：" {move || if validating.get() { "是" } else { "否" }}</p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
