// ============================================================
// 练习 e312: WebSocket 集成 — 连接生命周期与双向通信
//
// 核心知识点:
//   - WebSocket 的 SSR 架构中升级连接（upgrade）
//   - 连接生命周期: connecting → connected → disconnected
//   - 双向消息传递（send / receive）
//   - 使用 wasm_bindgen 绑定浏览器 WebSocket API
//
// 难度: ⭐⭐⭐ (最小引导 — 仅有少数关键 TODO)
// ============================================================

use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// WebSocket 的 JS 绑定
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

/// WebSocket 连接状态
#[derive(Clone)]
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
}

/// Hook: 管理 WebSocket 连接生命周期与双向消息
///
/// 返回 (received信号, send函数, 连接状态信号)
fn use_websocket(url: &str) -> (ReadSignal<Option<String>>, impl Fn(String), ReadSignal<ConnectionState>) {
    let (message, set_message) = signal(None::<String>);
    let (connection_state, set_connection_state) = signal(ConnectionState::Connecting);
    let ws_store: Rc<RefCell<Option<Ws>>> = Rc::new(RefCell::new(None));

    let socket = Ws::new(url);

    // 连接打开 → Connecting → Connected
    let conn = set_connection_state.clone();
    let open_cb = Closure::<dyn FnMut()>::new(move || {
        conn.set(ConnectionState::Connected);
    });
    socket.set_onopen(open_cb.as_ref().unchecked_ref());
    open_cb.forget();

    // 收到消息 → 更新 received 信号
    let msg = set_message.clone();
    let msg_cb = Closure::<dyn FnMut(JsValue)>::new(move |ev: JsValue| {
        let ev: MsgEvt = ev.unchecked_into();
        if let Some(text) = ev.ws_data().as_string() {
            msg.set(Some(text));
        }
    });
    socket.set_onmessage(msg_cb.as_ref().unchecked_ref());
    msg_cb.forget();

    // 连接关闭 → 切换为 Disconnected
    let close = set_connection_state.clone();
    let close_cb = Closure::<dyn FnMut(JsValue)>::new(move |_: JsValue| {
        close.set(ConnectionState::Disconnected);
    });
    socket.set_onclose(close_cb.as_ref().unchecked_ref());
    close_cb.forget();

    *ws_store.borrow_mut() = Some(socket);

    // TODO: 实现 send 函数 — 向 WebSocket 发送文本消息
    // ⭐⭐⭐ 提示: 从 ws_store 取出 socket，调用 ws_send
    let ws = ws_store.clone();
    let send = move |text: String| {
        // TODO: call socket.ws_send(&text)
    };

    (message, send, connection_state)
}

#[component]
fn Exercise() -> impl IntoView {
    let (received, send, connection_state) = use_websocket("wss://echo.websocket.org");
    let (input_text, set_input) = signal(String::new());

    // TODO: 实现 on_send — 读取 input_text 并通过 send() 发送，清空输入框

    // 连接状态描述辅助函数
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

            // TODO: 添加连接状态显示区域
            // ⭐⭐⭐ 提示: 用 <p> 显示 state_label()
            //         State 用 <strong> 加粗

            // TODO: 添加消息输入和发送按钮
            // ⭐⭐⭐ 提示: <input prop:value=... on:input=... />
            //         <button on:click=on_send>"发送"</button>

            // TODO: 添加收到的消息显示区域
            // ⭐⭐⭐ 提示: 用 <p> 显示 received.get()
            //         默认显示 "(等待消息...)"

            <hr/>
            <details>
                <summary>"💡 SSR 中的 WebSocket 集成要点"</summary>
                <ul>
                    <li>"在 SSR 应用中，WebSocket upgrade 由反向代理（如 nginx）处理"</li>
                    <li>"升级请求头: " <code>"Connection: Upgrade, Upgrade: websocket"</code></li>
                    <li>"客户端连接使用浏览器原生 WebSocket API（通过 wasm_bindgen 绑定）"</li>
                    <li>"服务端可使用 axum::extract::ws::WebSocketUpgrade 处理升级"</li>
                </ul>
            </details>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
