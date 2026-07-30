// ============================================================
// 练习 e150: use_websocket — WebSocket 连接管理
//
// 目标: 封装浏览器 WebSocket API 为响应式 Hook
//
// 难度: ⭐⭐⭐
// 核心知识点: WebSocket 连接, 事件处理, 双向通信
//
// TODO: 按照注释提示补全代码
// ============================================================

use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// 通过自定义 wasm_bindgen 绑定访问 WebSocket API
#[wasm_bindgen]
extern "C" {
    type Ws;
    #[wasm_bindgen(constructor)]
    fn new(url: &str) -> Ws;
    #[wasm_bindgen(method, js_name = send)]
    fn ws_send(this: &Ws, data: &str);
    #[wasm_bindgen(method, setter = onopen)]
    fn set_onopen(this: &Ws, handler: &JsValue);
    #[wasm_bindgen(method, setter = onmessage)]
    fn set_onmessage(this: &Ws, handler: &JsValue);
    #[wasm_bindgen(method, setter = onclose)]
    fn set_onclose(this: &Ws, handler: &JsValue);

    type MsgEvt;
    #[wasm_bindgen(method)]
    fn ws_data(this: &MsgEvt) -> JsValue;
}

/// Hook: WebSocket 连接管理
fn use_websocket(url: &str) -> (ReadSignal<Option<String>>, impl Fn(String), ReadSignal<bool>) {
    let (message, set_message) = signal(None::<String>);
    let (connected, set_connected) = signal(false);
    let ws_store: Rc<RefCell<Option<Ws>>> = Rc::new(RefCell::new(None));

    let socket = Ws::new(url);
    let conn = set_connected.clone();
    let open_cb = Closure::<dyn FnMut()>::new(move || {
        conn.set(true);
    });
    socket.set_onopen(open_cb.as_ref().unchecked_ref());
    open_cb.forget();

    let msg = set_message.clone();
    let msg_cb = Closure::<dyn FnMut(JsValue)>::new(move |ev: JsValue| {
        let ev: MsgEvt = ev.unchecked_into();
        if let Some(text) = ev.ws_data().as_string() {
            msg.set(Some(text));
        }
    });
    socket.set_onmessage(msg_cb.as_ref().unchecked_ref());
    msg_cb.forget();

    let conn2 = set_connected.clone();
    let close_cb = Closure::<dyn FnMut(JsValue)>::new(move |_: JsValue| {
        conn2.set(false);
    });
    socket.set_onclose(close_cb.as_ref().unchecked_ref());
    close_cb.forget();

    *ws_store.borrow_mut() = Some(socket);

    let ws = ws_store.clone();
    let send = move |text: String| {
        if let Some(ref socket) = *ws.borrow() {
            socket.ws_send(&text);
        }
    };

    (message, send, connected)
}

#[component]
fn Exercise() -> impl IntoView {
    let (received, send, connected) = use_websocket("wss://echo.websocket.org");
    let (input_text, set_input) = signal(String::new());

    let on_send = move |_| {
        let msg = input_text.get();
        if !msg.is_empty() {
            send(msg);
            set_input.set(String::new());
        }
    };

    view! {
        <div>
            <h2>"e150: use_websocket"</h2>
            <p>
                "连接状态: "
                {move || if connected.get() { "已连接" } else { "未连接" }}
            </p>
            <div>
                <input
                    type="text"
                    prop:value=input_text
                    on:input=move |ev| set_input(event_target_value(&ev))
                    placeholder="输入消息"
                />
                <button on:click=on_send>"发送"</button>
            </div>
            <p>
                "收到: "
                {move || received.get().unwrap_or_else(|| "(等待消息)".into())}
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// use std::cell::RefCell;
// use std::rc::Rc;
// use leptos::prelude::*;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
//
// #[wasm_bindgen]
// extern "C" {
//     type Ws;
//     #[wasm_bindgen(constructor)]
//     fn new(url: &str) -> Ws;
//     #[wasm_bindgen(method, js_name = send)]
//     fn ws_send(this: &Ws, data: &str);
//     #[wasm_bindgen(method, setter = onopen)]
//     fn set_onopen(this: &Ws, handler: &JsValue);
//     #[wasm_bindgen(method, setter = onmessage)]
//     fn set_onmessage(this: &Ws, handler: &JsValue);
//     #[wasm_bindgen(method, setter = onclose)]
//     fn set_onclose(this: &Ws, handler: &JsValue);
//     type MsgEvt;
//     #[wasm_bindgen(method)]
//     fn ws_data(this: &MsgEvt) -> JsValue;
// }
//
// fn use_websocket(url: &str) -> (ReadSignal<Option<String>>, impl Fn(String), ReadSignal<bool>) {
//     let (message, set_message) = signal(None);
//     let (connected, set_connected) = signal(false);
//     let ws_store: Rc<RefCell<Option<Ws>>> = Rc::new(RefCell::new(None));
//     let socket = Ws::new(url);
//     let conn = set_connected.clone();
//     let open_cb = Closure::<dyn FnMut()>::new(move || conn.set(true));
//     socket.set_onopen(open_cb.as_ref().unchecked_ref());
//     open_cb.forget();
//     let msg = set_message.clone();
//     let msg_cb = Closure::<dyn FnMut(JsValue)>::new(move |ev: JsValue| {
//         let ev: MsgEvt = ev.unchecked_into();
//         if let Some(text) = ev.ws_data().as_string() { msg.set(Some(text)); }
//     });
//     socket.set_onmessage(msg_cb.as_ref().unchecked_ref());
//     msg_cb.forget();
//     let close_cb = Closure::<dyn FnMut(JsValue)>::new(move |_: JsValue| {
//         set_connected.set(false);
//     });
//     socket.set_onclose(close_cb.as_ref().unchecked_ref());
//     close_cb.forget();
//     *ws_store.borrow_mut() = Some(socket);
//     let ws = ws_store.clone();
//     let send = move |text: String| {
//         if let Some(ref socket) = *ws.borrow() { socket.ws_send(&text); }
//     };
//     (message, send, connected)
// }
// ```
//
// ### 知识点
// - `Ws::new(url)` 创建连接，通过 `#[wasm_bindgen(constructor)]` 绑定 JS 构造函数
// - `set_onopen` / `set_onmessage` / `set_onclose` 注册事件处理器（JS setter 绑定）
// - `Closure::forget()` 防止 Rust 闭包被 GC（WASM 中闭包引用计数管理）
// - `Rc<RefCell<Option<Ws>>>` 保存 WebSocket 实例（替代 StoredValue）
// - `unchecked_into::<T>()` 将 raw JsValue 转型为自定义类型
//
// </details>
