// ============================================================
// 练习 141-145 综合答案
// 涵盖: ResizeObserver, MutationObserver, 自定义 Hook 三个层次
// ============================================================
#![feature(extern_types)]

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;

// ======================== e141 ========================
// ResizeObserver — 元素尺寸变化响应式监听
#[component]
fn E141ResizeObserver() -> impl IntoView {
    let (size, set_size) = signal("拖动右下角以调整大小".to_string());
    let div_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    // 创建 ResizeObserver 回调
    let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move |entries: js_sys::Array| {
        if entries.length() > 0 {
            let entry = entries.get(0);
            let rect =
                js_sys::Reflect::get(&entry, &"contentRect".into()).unwrap();
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

    let observer =
        web_sys::ResizeObserver::new(cb.as_ref().unchecked_ref()).unwrap();
    cb.forget();

    Effect::new(move |_| {
        if let Some(el) = div_ref.get() {
            observer.observe(el.as_ref());
        }
    });

    view! {
        <div style="border:1px solid #4A90D9; border-radius:6px; padding:12px; margin-bottom:16px; background:#F8FAFF;">
            <p><strong>"e141: ResizeObserver"</strong></p>
            <p>"调整下方蓝色方块右下角 ↙ 以改变尺寸："</p>
            <div
                node_ref=div_ref
                style="
                    resize: both; overflow: auto;
                    width: 200px; height: 100px;
                    border: 2px solid #4A90D9;
                    background: #E8F0FE;
                    padding: 8px; border-radius: 4px;
                    font-family: monospace; font-size: 16px;
                "
            >
                <p><strong>"尺寸: " {size}</strong></p>
            </div>
        </div>
    }
}

// ======================== e142 ========================
// MutationObserver — DOM 子树变化监听
#[component]
fn E142MutationObserver() -> impl IntoView {
    let (items, set_items) = signal(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    let (log, set_log) = signal(Vec::<String>::new());
    let container_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    // 创建 MutationObserver 回调
    let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move |mutations: js_sys::Array| {
        for i in 0..mutations.length() {
            let record = mutations.get(i);
            let typ = js_sys::Reflect::get(&record, &"type".into())
                .unwrap()
                .as_string()
                .unwrap_or_default();
            let mut msg = format!("类型: {}", typ);

            if typ == "childList" {
                let added = js_sys::Reflect::get(&record, &"addedNodes".into()).unwrap();
                let removed =
                    js_sys::Reflect::get(&record, &"removedNodes".into()).unwrap();
                let added_len = js_sys::Reflect::get(&added, &"length".into())
                    .unwrap()
                    .as_f64()
                    .unwrap() as i32;
                let removed_len = js_sys::Reflect::get(&removed, &"length".into())
                    .unwrap()
                    .as_f64()
                    .unwrap() as i32;
                if added_len > 0 {
                    msg.push_str(&format!(" | 添加了 {} 个节点", added_len));
                }
                if removed_len > 0 {
                    msg.push_str(&format!(" | 移除了 {} 个节点", removed_len));
                }
            }
            set_log.update(|v| v.push(msg));
        }
    }) as Box<dyn Fn(js_sys::Array)>);

    let observer =
        web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()).unwrap();
    cb.forget();

    let options = web_sys::MutationObserverInit::new();
    options.set_child_list(true);
    options.set_subtree(true);

    Effect::new(move |_| {
        if let Some(el) = container_ref.get() {
            observer.observe_with_options(el.as_ref(), &options).unwrap();
        }
    });

    view! {
        <div style="border:1px solid #E67E22; border-radius:6px; padding:12px; margin-bottom:16px; background:#FFF8F0;">
            <p><strong>"e142: MutationObserver"</strong></p>
            <p>"添加/删除下方列表中的元素，观察日志变化："</p>

            <div
                node_ref=container_ref
                style="border: 2px solid #E67E22; padding: 8px; border-radius: 4px; min-height: 60px;"
            >
                <ul>
                    {move || items()
                        .iter()
                        .enumerate()
                        .map(|(_, name)| {
                            let name = name.clone();
                            view! { <li>{name}</li> }
                        })
                        .collect::<Vec<_>>()}
                </ul>
            </div>

            <div style="margin-top: 8px; display: flex; gap: 8px;">
                <button on:click=move |_| {
                    let next = (items().len() as u8 + 65) as char;
                    set_items.update(|v| v.push(next.to_string()));
                }>"添加元素"</button>
                <button on:click=move |_| {
                    set_items.update(|v| { v.pop(); });
                }>"删除最后一个"</button>
                <button on:click=move |_| {
                    set_log.set(Vec::new());
                }>"清空日志"</button>
            </div>

            <div style="margin-top: 12px;">
                <p><strong>"变更日志："</strong></p>
                <ul style="font-family: monospace; font-size: 13px; color: #555;">
                    {move || {
                        let logs = log();
                        logs.iter().rev().take(10).map(|msg| {
                            let msg = msg.clone();
                            view! { <li>{msg}</li> }
                        }).collect::<Vec<_>>()
                    }}
                </ul>
            </div>
        </div>
    }
}

// ======================== e143, e144, e145 ========================
// 自定义 Hook 三层演进

/// e143: 基础 Hook — 封装 signal(0)
fn use_counter() -> (ReadSignal<i32>, WriteSignal<i32>) {
    signal(0)
}

/// e144: 返回便利闭包 increment
fn use_counter_with_increment() -> (ReadSignal<i32>, WriteSignal<i32>, impl Fn()) {
    let (count, set_count) = signal(0);
    let increment = move || {
        set_count.set(count() + 1);
    };
    (count, set_count, increment)
}

/// e145: 参数化 Hook — 可指定初始值
fn use_counter_start(start: i32) -> (ReadSignal<i32>, WriteSignal<i32>, impl Fn()) {
    let (count, set_count) = signal(start);
    let increment = move || {
        set_count.set(count() + 1);
    };
    (count, set_count, increment)
}

#[component]
fn E143HookBasic() -> impl IntoView {
    let (count, set_count) = use_counter();

    view! {
        <div style="border:1px solid #27AE60; border-radius:6px; padding:12px; margin-bottom:16px; background:#F0FFF4;">
            <p><strong>"e143: use_counter 基础 Hook"</strong></p>
            <p style="font-size: 24px; font-weight: bold;">"计数: " {count}</p>
            <button on:click=move |_| { set_count.set(count() + 1); }>"+1"</button>
            <button on:click=move |_| { set_count.set(0); }>"重置"</button>
        </div>
    }
}

#[component]
fn E144HookReturnSignal() -> impl IntoView {
    let (count, _set_count, increment) = use_counter_with_increment();

    view! {
        <div style="border:1px solid #8E44AD; border-radius:6px; padding:12px; margin-bottom:16px; background:#F8F0FF;">
            <p><strong>"e144: use_counter + increment 闭包"</strong></p>
            <p style="font-size: 24px; font-weight: bold;">"计数: " {count}</p>
            <button on:click=move |_| { increment(); }>"+1 (使用 increment)"</button>
        </div>
    }
}

#[component]
fn E145HookParams() -> impl IntoView {
    let (count, _set_count, increment) = use_counter_start(10);

    view! {
        <div style="border:1px solid #E74C3C; border-radius:6px; padding:12px; margin-bottom:16px; background:#FFF5F5;">
            <p><strong>"e145: use_counter(10) 参数化 Hook"</strong></p>
            <p style="font-size: 24px; font-weight: bold;">"计数: " {count}</p>
            <button on:click=move |_| { increment(); }>"+1"</button>
            <p style="color: #888; font-size: 14px;">"初始值为 10，点击按钮递增"</p>
        </div>
    }
}

// ======================== e146 ========================
// Hook 依赖注入 — use_theme() 读取 context
#[derive(Clone, Debug, PartialEq)]
struct Theme {
    primary: String,
    background: String,
    text: String,
}

fn use_theme() -> Theme {
    use_context::<Theme>().expect("use_theme: Theme not found. Call provide_context first.")
}

#[component]
fn ThemedBox() -> impl IntoView {
    let theme = use_theme();

    view! {
        <div
            style=format!(
                "background-color:{}; color:{}; border:2px solid {}; padding:20px; border-radius:8px",
                theme.background, theme.text, theme.primary
            )
        >
            <h3>"主题展示"</h3>
            <p>"主色: " {theme.primary.clone()}</p>
            <p>"背景: " {theme.background.clone()}</p>
            <p>"文字: " {theme.text.clone()}</p>
        </div>
    }
}

#[component]
fn E146HookDi() -> impl IntoView {
    provide_context(Theme {
        primary: "#4f46e5".into(),
        background: "#ffffff".into(),
        text: "#1e293b".into(),
    });

    view! {
        <div style="border:1px solid #4f46e5; border-radius:6px; padding:12px; margin-bottom:16px; background:#F5F3FF;">
            <p><strong>"e146: Hook 依赖注入"</strong></p>
            <ThemedBox />
        </div>
    }
}

// ======================== e147 ========================
// use_local_storage — localStorage 读写

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_namespace = ["localStorage"])]
    fn getItem(key: &str) -> Option<String>;
    #[wasm_bindgen(js_namespace = ["localStorage"])]
    fn setItem(key: &str, value: &str);
}

fn use_local_storage(key: &str, default: &str) -> (ReadSignal<String>, WriteSignal<String>) {
    let initial = unsafe { getItem(key) }.unwrap_or_else(|| default.to_string());
    let (value, set_value) = signal(initial);
    let key = key.to_string();

    Effect::new(move |_| {
        let v = value.get();
        unsafe { setItem(&key, &v); }
    });

    (value, set_value)
}

#[component]
fn E147UseLocalStorage() -> impl IntoView {
    let (name, set_name) = use_local_storage("username", "匿名用户");

    view! {
        <div style="border:1px solid #E67E22; border-radius:6px; padding:12px; margin-bottom:16px; background:#FFF8F0;">
            <p><strong>"e147: use_local_storage"</strong></p>
            <input
                type="text"
                prop:value=name
                on:input=move |ev| set_name(event_target_value(&ev))
                placeholder="输入你的名字"
            />
            <p>"你好, " {name}</p>
            <p style="font-size:0.8em;color:#888">"刷新页面后名字依然保留!"</p>
        </div>
    }
}

// ======================== e148 ========================
// use_media_query — 响应式媒体查询
use leptos::ev;

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_namespace = ["window"])]
    fn innerWidth() -> f64;
}

fn extract_threshold(query: &str) -> f64 {
    query
        .split(|c: char| !c.is_ascii_digit() && c != '.')
        .filter_map(|s| s.parse::<f64>().ok())
        .next()
        .unwrap_or(0.0)
}

fn use_media_query(query: &str) -> ReadSignal<bool> {
    let threshold = extract_threshold(query);
    let (is_match, set_is_match) = signal(unsafe { innerWidth() } >= threshold);

    window_event_listener(ev::resize, move |_| {
        set_is_match.set(unsafe { innerWidth() } >= threshold);
    });

    is_match
}

#[component]
fn E148UseMediaQuery() -> impl IntoView {
    let is_wide = use_media_query("(min-width: 600px)");

    view! {
        <div style="border:1px solid #27AE60; border-radius:6px; padding:12px; margin-bottom:16px; background:#F0FFF4;">
            <p><strong>"e148: use_media_query"</strong></p>
            <p>
                "当前视口: "
                {move || if is_wide.get() { "宽屏 (≥600px)" } else { "窄屏 (<600px)" }}
            </p>
            <p style="font-size:0.8em;color:#888">"缩小/放大浏览器窗口查看变化"</p>
        </div>
    }
}

// ======================== e149 ========================
// use_geolocation — 地理位置 API

#[wasm_bindgen]
unsafe extern "C" {
    type Geo;
    type GeoCoords;
    type GeoPosition;
    type GeoError;

    #[wasm_bindgen(method, getter = coords)]
    fn geo_coords(this: &GeoPosition) -> GeoCoords;
    #[wasm_bindgen(method, getter = latitude)]
    fn geo_lat(this: &GeoCoords) -> f64;
    #[wasm_bindgen(method, getter = longitude)]
    fn geo_lng(this: &GeoCoords) -> f64;
    #[wasm_bindgen(method, getter = message)]
    fn geo_msg(this: &GeoError) -> String;

    #[wasm_bindgen(method, js_name = watchPosition)]
    fn watch_position(this: &Geo, success: &JsValue, error: &JsValue);

    #[wasm_bindgen(js_namespace = ["navigator"])]
    fn geolocation() -> Option<Geo>;
}

fn use_geolocation() -> (ReadSignal<Option<f64>>, ReadSignal<Option<f64>>, ReadSignal<Option<String>>) {
    let (lat, set_lat) = signal(None::<f64>);
    let (lng, set_lng) = signal(None::<f64>);
    let (error, set_error) = signal(None::<String>);

    if let Some(geo) = unsafe { geolocation() } {
        let success = Closure::<dyn FnMut(JsValue)>::new(move |pos: JsValue| {
            let pos: GeoPosition = pos.unchecked_into();
            let c = unsafe { pos.geo_coords() };
            set_lat.set(Some(unsafe { c.geo_lat() }));
            set_lng.set(Some(unsafe { c.geo_lng() }));
        });
        let fail = Closure::<dyn FnMut(JsValue)>::new(move |err: JsValue| {
            let err: GeoError = err.unchecked_into();
            set_error.set(Some(unsafe { err.geo_msg() }));
        });
        unsafe { geo.watch_position(success.as_ref().unchecked_ref(), fail.as_ref().unchecked_ref()); }
        success.forget();
        fail.forget();
    } else {
        set_error.set(Some("浏览器不支持地理位置".into()));
    }

    (lat, lng, error)
}

#[component]
fn E149UseGeolocation() -> impl IntoView {
    let (lat, lng, error) = use_geolocation();

    view! {
        <div style="border:1px solid #8E44AD; border-radius:6px; padding:12px; margin-bottom:16px; background:#F8F0FF;">
            <p><strong>"e149: use_geolocation"</strong></p>
            {move || {
                if let Some(err) = error.get() {
                    view! { <p style="color:red">"错误: " {err}</p> }.into_any()
                } else {
                    let lat_str = lat.get().map(|v| format!("{:.6}", v));
                    let lng_str = lng.get().map(|v| format!("{:.6}", v));
                    view! {
                        <div>
                            <p>"纬度: " {lat_str.unwrap_or_else(|| "获取中...".into())}</p>
                            <p>"经度: " {lng_str.unwrap_or_else(|| "获取中...".into())}</p>
                            <p style="font-size:0.8em;color:#888">"位置信息基于浏览器定位，精度因设备而异"</p>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

// ======================== e150 ========================
// use_websocket — WebSocket 连接管理
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
unsafe extern "C" {
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

fn use_websocket(url: &str) -> (ReadSignal<Option<String>>, impl Fn(String), ReadSignal<bool>) {
    let (message, set_message) = signal(None::<String>);
    let (connected, set_connected) = signal(false);
    let ws_store: Rc<RefCell<Option<Ws>>> = Rc::new(RefCell::new(None));

    let socket = unsafe { Ws::new(url) };
    let conn = set_connected.clone();
    let open_cb = Closure::<dyn FnMut()>::new(move || {
        conn.set(true);
    });
    unsafe { socket.set_onopen(open_cb.as_ref().unchecked_ref()); }
    open_cb.forget();

    let msg = set_message.clone();
    let msg_cb = Closure::<dyn FnMut(JsValue)>::new(move |ev: JsValue| {
        let ev: MsgEvt = ev.unchecked_into();
        if let Some(text) = unsafe { ev.ws_data() }.as_string() {
            msg.set(Some(text));
        }
    });
    unsafe { socket.set_onmessage(msg_cb.as_ref().unchecked_ref()); }
    msg_cb.forget();

    let conn2 = set_connected.clone();
    let close_cb = Closure::<dyn FnMut(JsValue)>::new(move |_: JsValue| {
        conn2.set(false);
    });
    unsafe { socket.set_onclose(close_cb.as_ref().unchecked_ref()); }
    close_cb.forget();

    *ws_store.borrow_mut() = Some(socket);

    let ws = ws_store.clone();
    let send = move |text: String| {
        if let Some(ref socket) = *ws.borrow() {
            unsafe { socket.ws_send(&text); }
        }
    };

    (message, send, connected)
}

#[component]
fn E150UseWebSocket() -> impl IntoView {
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
        <div style="border:1px solid #E74C3C; border-radius:6px; padding:12px; margin-bottom:16px; background:#FFF5F5;">
            <p><strong>"e150: use_websocket"</strong></p>
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

// ======================== App ========================
#[component]
fn App() -> impl IntoView {
    view! {
        <div style="max-width:700px; margin:0 auto; padding:16px; font-family:sans-serif;">
            <h1>"练习 141-150 答案"</h1>
            <E141ResizeObserver/>
            <E142MutationObserver/>
            <E143HookBasic/>
            <E144HookReturnSignal/>
            <E145HookParams/>
            <E146HookDi/>
            <E147UseLocalStorage/>
            <E148UseMediaQuery/>
            <E149UseGeolocation/>
            <E150UseWebSocket/>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
