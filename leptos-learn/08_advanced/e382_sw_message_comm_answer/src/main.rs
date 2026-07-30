// ============================================================
// e382 Answer: Service Worker 消息通信 — 主线程 ↔ SW 双向通信
//
// 核心知识点:
//   - 使用 wasm_bindgen inline_js 调用浏览器 Service Worker API
//   - navigator.serviceWorker.register 注册 SW
//   - postMessage 实现主线程 ↔ SW 双向通信
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

// Service Worker 内联脚本: 安装后立即激活，收到消息后回复
const SW_SCRIPT: &str = r#"
self.addEventListener('install', () => self.skipWaiting());
self.addEventListener('activate', (e) => e.waitUntil(self.clients.claim()));
self.addEventListener('message', (event) => {
    const data = event.data;
    self.clients.matchAll().then(clients => {
        clients.forEach(client => {
            client.postMessage({
                type: 'sw_reply',
                original: data.text || '',
                reply: 'SW 回复: ' + (data.text || ''),
                timestamp: Date.now()
            });
        });
    });
});
"#;

// 使用 inline_js 定义 JS 辅助函数:
// 1) _reg_sw: 创建 Blob URL 并注册 Service Worker（回调模式，无 async）
// 2) _post_to_sw: 向 SW 发送 postMessage
// 3) _on_sw_msg: 监听来自 SW 的消息
#[wasm_bindgen(inline_js = r#"
export function _reg_sw(code, on_ok, on_err) {
    const blob = new Blob([code], {type: 'application/javascript'});
    const url = URL.createObjectURL(blob);
    navigator.serviceWorker.register(url)
        .then(reg => on_ok(reg))
        .catch(e => on_err(e.message));
}
export function _post_to_sw(reg, text) {
    if (reg && reg.active) {
        reg.active.postMessage({text: text});
    }
}
export function _on_sw_msg(cb) {
    navigator.serviceWorker.addEventListener('message', function(e) {
        cb(e.data);
    });
}
"#)]
extern "C" {
    fn _reg_sw(code: &str, on_ok: &JsValue, on_err: &JsValue);
    fn _post_to_sw(reg: &JsValue, text: &str);
    fn _on_sw_msg(cb: &JsValue);
}

#[component]
fn Exercise() -> impl IntoView {
    // 存储 SW 注册对象引用
    let registration: RwSignal<Option<JsValue>> = RwSignal::new(None);
    // SW 注册状态
    let status = RwSignal::new("⏳ 正在注册...".to_string());
    // 消息通信日志
    let messages: RwSignal<Vec<String>> = RwSignal::new(Vec::new());
    // 输入框内容
    let input_text = RwSignal::new(String::new());

    // 注册 Service Worker — 用回调模式，避免 Rust async/await
    {
        let status = status.clone();
        let messages = messages.clone();
        let registration = registration.clone();

        // 成功回调：Promise resolved
        let on_success = Closure::<dyn FnMut(JsValue)>::new(move |reg: JsValue| {
            registration.set(Some(reg));
            status.set("✅ 已注册并激活".to_string());
            messages.update(|log| {
                log.push("[系统] Service Worker 注册成功".to_string());
            });
        });

        // 失败回调：Promise rejected
        let on_error = Closure::<dyn FnMut(JsValue)>::new(move |err: JsValue| {
            let err_msg = err.as_string().unwrap_or_default();
            status.set(format!("❌ 注册失败: {}", err_msg));
            messages.update(|log| {
                log.push(format!("[系统] 注册失败: {}", err_msg));
            });
        });

        // 调用 JS 函数启动注册（Promise 异步执行）
        _reg_sw(SW_SCRIPT, on_success.as_ref().unchecked_ref(), on_error.as_ref().unchecked_ref());
        on_success.forget();
        on_error.forget();
    }

    // 设置消息监听回调 — 独立于注册流程
    {
        let msg_log = messages.clone();
        let sw_callback = Closure::<dyn FnMut(JsValue)>::new(move |data: JsValue| {
            // 从 JS 对象中提取 reply 字段
            let reply = js_sys::Reflect::get(&data, &JsValue::from("reply"))
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_default();
            if !reply.is_empty() {
                msg_log.update(|log| {
                    log.push(reply);
                });
            }
        });
        // 将回调注册为 SW 消息监听器
        _on_sw_msg(sw_callback.as_ref().unchecked_ref());
        sw_callback.forget();
    }

    // 发送消息处理函数
    let send_message = move |_| {
        let text = input_text.get();
        if text.trim().is_empty() {
            return;
        }
        // 获取 registration 引用并发消息
        if let Some(reg) = registration.get() {
            _post_to_sw(&reg, &text);
            messages.update(|log| {
                log.push(format!("[我] {}", text));
            });
            input_text.set(String::new());
        } else {
            messages.update(|log| {
                log.push("[系统] SW 尚未就绪".to_string());
            });
        }
    };

    view! {
        <div>
            <h2>"🔄 Service Worker 消息通信"</h2>

            <div>
                <h3>"状态"</h3>
                <p>{move || status.get()}</p>
            </div>

            <div>
                <h3>"发送消息"</h3>
                <input
                    type="text"
                    placeholder="输入消息..."
                    prop:value={move || input_text.get()}
                    on:input=move |ev| {
                        input_text.set(event_target_value(&ev));
                    }
                />
                <button on:click=send_message>"发送"</button>
            </div>

            <div>
                <h3>"消息日志"</h3>
                <ul>
                    {move || messages.get().iter().map(|msg| {
                        view! { <li>{msg.clone()}</li> }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
