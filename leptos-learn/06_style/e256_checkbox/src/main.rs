use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建一个 checked signal (bool 类型)
    // 提示: 使用 signal(false) 返回 (ReadSignal, WriteSignal)

    view! {
        <div>
            <h2>"练习 256 — 复选框"</h2>
            <label>
                <input type="checkbox"
                    // TODO: 使用 prop:checked 绑定到 checked signal
                    // TODO: 绑定 on:change 事件来更新 signal
                    // 提示: event_target::<leptos::web_sys::HtmlInputElement>(&ev).checked()
                />
                " 勾选此项"
            </label>
            <p>
                // TODO: 根据 checked 的值显示 "✓ 已勾选" 或 "✗ 未勾选"
                // 提示: 使用 move || if checked.get() { ... } else { ... }
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
