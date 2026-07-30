// ============================================================
// 练习 e141: ResizeObserver — 元素尺寸变化响应式监听
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::ResizeObserver;

#[component]
fn Exercise() -> impl IntoView {
    let (size, set_size) = signal("拖动右下角以调整大小".to_string());
    let div_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    let cb = Closure::wrap(Box::new(move |entries: js_sys::Array| {
        if entries.length() > 0 {
            let entry = entries.get(0);
            let rect = js_sys::Reflect::get(&entry, &"contentRect".into()).unwrap();
            let w = js_sys::Reflect::get(&rect, &"width".into())
                .unwrap()
                .as_f64()
                .unwrap() as i32;
            let h = js_sys::Reflect::get(&rect, &"height".into())
                .unwrap()
                .as_f64()
                .unwrap() as i32;
            set_size.set(format!("{} × {} px", w, h));
        }
    }) as Box<dyn Fn(js_sys::Array)>);

    let observer = ResizeObserver::new(cb.as_ref().unchecked_ref()).unwrap();
    cb.forget();

    Effect::new(move |_| {
        if let Some(el) = div_ref.get() {
            observer.observe(el.as_ref());
        }
    });

    view! {
        <div>
            <h3>"练习 e141: ResizeObserver"</h3>
            <p>"调整下方方块大小查看变化："</p>
            <div
                node_ref=div_ref
                style="resize: both; overflow: auto; width: 200px; height: 100px; border: 2px solid #4A90D9; background: #E8F0FE; padding: 8px; border-radius: 4px; font-family: monospace; font-size: 16px;"
            >
                <p><strong>"尺寸: " {size}</strong></p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
