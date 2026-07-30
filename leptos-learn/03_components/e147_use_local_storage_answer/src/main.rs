// ============================================================
// Exercise 147 - Answer: use_local_storage
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["localStorage"])]
    fn getItem(key: &str) -> Option<String>;
    #[wasm_bindgen(js_namespace = ["localStorage"])]
    fn setItem(key: &str, value: &str);
}

fn use_local_storage(key: &str, default: &str) -> (ReadSignal<String>, WriteSignal<String>) {
    let initial = getItem(key).unwrap_or_else(|| default.to_string());
    let (value, set_value) = signal(initial);
    let key = key.to_string();

    Effect::new(move |_| {
        let v = value.get();
        setItem(&key, &v);
    });

    (value, set_value)
}

#[component]
fn Exercise() -> impl IntoView {
    let (name, set_name) = use_local_storage("username", "匿名用户");

    view! {
        <div>
            <h2>"e147: use_local_storage"</h2>
            <input
                type="text"
                prop:value=name
                on:input=move |ev| set_name(event_target_value(&ev))
                placeholder="输入你的名字"
            />
            <p>"你好, " {name}</p>
            <p style="font-size:0.8em;color:#888">
                "刷新页面后名字依然保留!"
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
