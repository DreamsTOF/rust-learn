// ============================================================
// 练习 e382: Service Worker 消息通信 — 主线程 ↔ SW 双向通信
//
// 核心知识点:
//   - 使用 wasm_bindgen inline_js 调用浏览器 Service Worker API
//   - navigator.serviceWorker.register 注册 SW
//   - postMessage 实现主线程 ↔ SW 双向通信
//   - Closure 处理 JS 回调
//
// 难度: ⭐⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use leptos::task::spawn_local;

// Service Worker 内联脚本: 收到消息后回复
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
                reply: 'SW 收到: ' + (data.text || ''),
                timestamp: Date.now()
            });
        });
    });
});
"#;

// TODO 1: 完成 wasm-bindgen 外部函数接口
// 需要定义以下 JS 函数:
//   1) registerSW(code: string) → 创建 Blob URL 并注册 SW
//   2) postToSW(registration, text: string) → 向 SW 发送消息
//   3) onSWMessage(callback) → 监听来自 SW 的消息
#[wasm_bindgen(inline_js = r#"
// TODO: 实现 registerSW 函数
// - 使用 Blob 从 SW 代码字符串创建 URL
// - 调用 navigator.serviceWorker.register(url)
// - 返回 registration 对象

// TODO: 实现 postToSW 函数
// - 检查 registration.active 是否存在
// - 调用 registration.active.postMessage({text: text})

// TODO: 实现 onSWMessage 函数
// - 在 navigator.serviceWorker 上监听 'message' 事件
// - 调用 callback(event.data)
"#)]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn registerSW(code: &str) -> Result<JsValue, JsValue>;
    fn postToSW(registration: &JsValue, text: &str);
    fn onSWMessage(callback: &JsValue);
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO 2: 创建所需的信号
    // - registration: RwSignal<Option<JsValue>> — 存储 SW 注册对象
    // - status: RwSignal<String> — SW 注册状态
    // - messages: RwSignal<Vec<String>> — 消息日志
    // - input_text: RwSignal<String> — 输入框内容

    // TODO 3: 使用 spawn_local 在组件初始化时注册 SW
    // - 调用 registerSW(SW_SCRIPT).await
    // - 成功时更新 registration 和 status 信号
    // - 使用 Closure::new + onSWMessage 设置消息监听
    // - 通过 js_sys::Reflect::get 从 JS 对象中提取 reply 字段
    // - 将收到的消息添加到 messages 日志

    // TODO 4: 实现发送消息函数
    // - 从 registration 获取注册对象
    // - 调用 postToSW 发送消息
    // - 将发送的消息添加到日志

    view! {
        <div>
            <h2>"🔄 Service Worker 消息通信"</h2>

            <div>
                <h3>"状态"</h3>
                // TODO 5: 显示 SW 注册状态
            </div>

            <div>
                <h3>"发送消息"</h3>
                // TODO 6: 创建输入框和发送按钮
                // - 输入框绑定 input_text 信号
                // - 发送按钮调用 postToSW
                <input type="text" placeholder="输入消息..." />
                <button>"发送"</button>
            </div>

            <div>
                <h3>"消息日志"</h3>
                // TODO 7: 遍历 messages 信号，显示所有消息
            </div>
        </div>
    }
}

 fn main() {
     mount_to_body(Exercise);
 }
