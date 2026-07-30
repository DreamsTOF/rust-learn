// ============================================================
// Exercise 154 - Answer: use_clipboard
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type JsClipboard;

    #[wasm_bindgen(method, js_name = writeText)]
    fn write_text(this: &JsClipboard, text: &str) -> JsValue;

    #[wasm_bindgen(method, js_name = readText)]
    fn read_text(this: &JsClipboard) -> JsValue;

    type JsPromise;

    #[wasm_bindgen(method, js_name = then)]
    fn then(this: &JsPromise, cb: &Closure<dyn FnMut(JsValue)>) -> JsPromise;

    type JsWin;
    #[wasm_bindgen(method, getter, js_name = navigator)]
    fn navigator(this: &JsWin) -> JsValue;
    type JsNav;
    #[wasm_bindgen(method, getter)]
    fn clipboard(this: &JsNav) -> JsValue;
}

fn get_clipboard() -> Option<JsClipboard> {
    let window_val: JsValue = window().into();
    let js_win: &JsWin = window_val.unchecked_ref();
    let nav: JsValue = js_win.navigator();
    let nav: &JsNav = nav.unchecked_ref();
    let cb: JsValue = nav.clipboard();
    Some(cb.unchecked_into::<JsClipboard>())
}

fn use_clipboard() -> (
    ReadSignal<Option<String>>,
    impl Fn(String) + 'static,
    impl Fn() + 'static,
) {
    let (text, set_text) = signal::<Option<String>>(None);

    let copy = move |val: String| {
        if let Some(clipboard) = get_clipboard() {
            clipboard.write_text(&val);
        }
        set_text.set(Some(val));
    };

    let read = move || {
        if let Some(clipboard) = get_clipboard() {
            let promise = clipboard.read_text();
            let promise = promise.unchecked_into::<JsPromise>();
            let setter = set_text.clone();
            let cb = Closure::wrap(Box::new(move |val: JsValue| {
                setter.set(val.as_string().or(Some("读取失败".to_string())));
            }) as Box<dyn FnMut(JsValue)>);
            promise.then(&cb);
            cb.forget();
        }
    };

    (text, copy, read)
}

#[component]
fn Exercise() -> impl IntoView {
    let (text, copy, read) = use_clipboard();
    let (input_text, set_input_text) = signal("你好，剪贴板！".to_string());

    view! {
        <div>
            <h3>"练习 154: use_clipboard"</h3>
            <input
                type="text"
                prop:value=input_text
                on:input=move |ev| set_input_text.set(event_target_value(&ev))
            />
            <button on:click=move |_| copy(input_text())>"复制到剪贴板"</button>
            <button on:click=move |_| read()>"从剪贴板读取"</button>
            <p>"剪贴板内容: " {move || text().unwrap_or_default()}</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
