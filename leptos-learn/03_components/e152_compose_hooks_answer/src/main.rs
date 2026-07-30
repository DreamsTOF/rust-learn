// ============================================================
// Exercise 152 - Answer: compose_hooks
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use js_sys::Reflect;

/// 自定义计数器 Hook
fn use_counter(initial: i32) -> (ReadSignal<i32>, impl Fn() + 'static, impl Fn() + 'static, impl Fn() + 'static) {
    let (count, set_count) = signal(initial);
    let increment = move || set_count.update(|c| *c += 1);
    let decrement = move || set_count.update(|c| *c -= 1);
    let reset = move || set_count.set(initial);
    (count, increment, decrement, reset)
}

/// 从 localStorage 读取值
fn get_local_storage(key: &str) -> Option<String> {
    let global = js_sys::global();
    let storage = Reflect::get(&global, &JsValue::from("localStorage")).ok()?;
    Reflect::get(&storage, &JsValue::from(key)).ok()?.as_string()
}

/// 写入值到 localStorage
fn set_local_storage(key: &str, value: &str) {
    let global = js_sys::global();
    if let Ok(storage) = Reflect::get(&global, &JsValue::from("localStorage")) {
        let _ = Reflect::set(&storage, &JsValue::from(key), &JsValue::from(value));
    }
}

/// 自定义 localStorage Hook
fn use_local_storage(key: &'static str, initial: i32) -> (ReadSignal<i32>, WriteSignal<i32>) {
    let stored = get_local_storage(key)
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(initial);

    let (value, set_value) = signal(stored);

    Effect::new(move |_| {
        let v = value();
        set_local_storage(key, &v.to_string());
    });

    (value, set_value)
}

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = use_local_storage("counter_e152", 0);
    let increment = move || set_count.update(|c| *c += 1);
    let decrement = move || set_count.update(|c| *c -= 1);
    let reset = move || set_count.set(0);

    view! {
        <div>
            <h3>"练习 152: compose_hooks"</h3>
            <p>"计数: " {count}</p>
            <button on:click=move |_| increment()>"+1"</button>
            <button on:click=move |_| decrement()>"-1"</button>
            <button on:click=move |_| reset()>"重置"</button>
            <p><small>"（刷新页面后计数仍保持）"</small></p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
