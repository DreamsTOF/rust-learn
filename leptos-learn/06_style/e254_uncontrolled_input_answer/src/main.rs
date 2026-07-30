// ============================================================
// 练习 254 — 参考答案
// ============================================================

use leptos::prelude::*;
use leptos::html::Input;

fn main() {
    mount_to_body(Exercise);
}

#[component]
fn Exercise() -> impl IntoView {
    let input_ref: NodeRef<Input> = NodeRef::new();
    let value = RwSignal::new(String::new());

    let read_value = move |_| {
        if let Some(input) = input_ref.get() {
            value.set(input.value());
        }
    };

    view! {
        <div>
            <h2>"练习 254: 非受控输入框"</h2>
            <div>
                <input type="text" node_ref=input_ref />
                <button on:click=read_value>"读取值"</button>
            </div>
            <p>"读取的值: " {value}</p>
        </div>
    }
}
