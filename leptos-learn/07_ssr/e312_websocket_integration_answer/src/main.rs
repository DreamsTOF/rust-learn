// ============================================================
// 练习 e312: WebSocket 集成 — 参考答案
//
// 核心知识点:
//   - WebSocket 连接生命周期: connecting → connected → disconnected
//   - wasm_bindgen 绑定浏览器 WebSocket API
//   - 双向消息传递（send / receive）
//   - SSR 架构中 WS upgrade 的反向代理配置
// ============================================================

use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// WebSocket 原生 JS 绑定
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
    #[wasm_bindgen(method, js_name = data)]
    fn ws_data(this: &MsgEvt) -> JsValue;
}

/// WebSocket 连接生命周期状态
#[derive(Clone)]
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
}

/// Hook: 管理 WebSocket 连接生命周期与双向消息传递
///
/// 返回 (received, send, connection_state) 三元组
fn use_websocket(url: &str) -> (ReadSignal<Option<String>>, impl Fn(String), ReadSignal<ConnectionState>) {
    let (message, set_message) = signal(None::<String>);
    let (connection_state, set_connection_state) = signal(ConnectionState::Connecting);
    let ws_store: Rc<RefCell<Option<Ws>>> = Rc::new(RefCell::new(None));

    let socket = Ws::new(url);

    // onopen — 连接建立，状态切换为 Connected
    let conn = set_connection_state.clone();
    let open_cb = Closure::<dyn FnMut()>::new(move || {
        conn.set(ConnectionState::Connected);
    });
    socket.set_onopen(open_cb.as_ref().unchecked_ref());
    open_cb.forget();

    // onmessage — 收到消息，更新 received 信号
    let msg = set_message.clone();
    let msg_cb = Closure::<dyn FnMut(JsValue)>::new(move |ev: JsValue| {
        let ev: MsgEvt = ev.unchecked_into();
        if let Some(text) = ev.ws_data().as_string() {
            msg.set(Some(text));
        }
    });
    socket.set_onmessage(msg_cb.as_ref().unchecked_ref());
    msg_cb.forget();

    // onclose — 连接关闭，状态切换为 Disconnected
    let close = set_connection_state.clone();
    let close_cb = Closure::<dyn FnMut(JsValue)>::new(move |_: JsValue| {
        close.set(ConnectionState::Disconnected);
    });
    socket.set_onclose(close_cb.as_ref().unchecked_ref());
    close_cb.forget();

    *ws_store.borrow_mut() = Some(socket);

    // send — 向 WebSocket 发送文本消息
    let ws = ws_store.clone();
    let send = move |text: String| {
        if let Some(ref socket) = *ws.borrow() {
            socket.ws_send(&text);
        }
    };

    (message, send, connection_state)
}

#[component]
fn Exercise() -> impl IntoView {
    let (received, send, connection_state) = use_websocket("wss://echo.websocket.org");
    let (input_text, set_input) = signal(String::new());

    // 发送消息: 读取输入框内容 → 发送 → 清空
    let on_send = move |_| {
        let msg = input_text.get();
        if !msg.is_empty() {
            send(msg);
            set_input.set(String::new());
        }
    };

    // 连接状态转可读文本
    let state_label = move || -> &'static str {
        match connection_state.get() {
            ConnectionState::Connected => "已连接",
            ConnectionState::Connecting => "连接中...",
            ConnectionState::Disconnected => "已断开",
        }
    };

    view! {
        <div style="max-width: 480px; margin: 24px auto; font-family: system-ui, sans-serif;">
            <h2>"📡 WebSocket 集成 — 连接生命周期"</h2>

            <p>
                "连接状态: "
                <strong>{state_label}</strong>
            </p>

            <div style="display: flex; gap: 8px; margin: 16px 0;">
                <input
                    type="text"
                    prop:value=input_text
                    on:input=move |ev| set_input(event_target_value(&ev))
                    placeholder="输入消息后发送"
                    style="flex: 1; padding: 6px 10px;"
                />
                <button on:click=on_send style="padding: 6px 16px;">
                    "发送"
                </button>
            </div>

            <p>
                "收到: "
                {move || received.get().unwrap_or_else(|| "(等待消息...)".into())}
            </p>

            <hr/>
            <details>
                <summary>"💡 SSR 中的 WebSocket 集成要点"</summary>
                <ul>
                    <li>"WebSocket upgrade 由反向代理（nginx）处理，配置 Upgrade / Connection 头"</li>
                    <li>"客户端通过浏览器原生 WebSocket API 连接（wasm_bindgen 绑定）"</li>
                    <li>"服务端使用 axum::extract::ws::WebSocketUpgrade 处理 upgrade 请求"</li>
                    <li>"连接生命周期: Connecting → Connected → Disconnected（onopen / onclose）"</li>
                    <li>"双向通信: send() 发送，onmessage 接收"</li>
                </ul>
            </details>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
