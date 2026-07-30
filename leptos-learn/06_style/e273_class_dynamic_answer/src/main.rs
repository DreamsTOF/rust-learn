use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (is_active, set_is_active) = signal(false);

    let toggle_active = move |_| {
        set_is_active.update(|v| *v = !*v);
    };

    view! {
        <div style="padding: 20px;">
            <h2>"动态类名"</h2>

            <div
                class:active={move || is_active.get()}
                style="padding: 20px; border: 1px solid #ccc; border-radius: 8px; transition: all 0.3s; margin: 10px 0;"
            >
                <p>"这个元素的样式会随 active 类切换而变化"</p>
                <p>"当前状态：" {move || if is_active.get() { "激活" } else { "未激活" }}</p>
            </div>

            <button on:click=toggle_active style="padding: 8px 16px; cursor: pointer;">
                {move || if is_active.get() { "取消激活" } else { "激活" }}
            </button>

            <style>
                ".active {
                    background-color: #4caf50;
                    color: white;
                    font-weight: bold;
                    border-color: #388e3c !important;
                }"
            </style>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
