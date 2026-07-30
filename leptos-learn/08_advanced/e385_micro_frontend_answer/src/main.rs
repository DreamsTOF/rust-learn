// ============================================================
// 练习 e385: 微前端集成 — iframe 嵌入与跨文档消息通信
//
// 核心知识点:
//   - 使用 <iframe> 嵌入微前端页面
//   - window_event_listener(ev::message, ...) 监听跨域消息
//   - iframe.contentWindow.postMessage 发送消息
//   - Blob URL 内联生成微前端 HTML
//
// 难度: ⭐⭐⭐
// ============================================================

use leptos::prelude::*;
use leptos::ev;
use leptos::web_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// 微前端 HTML 模板 — 自包含的迷你应用，支持 postMessage 双向通信
const MICRO_FRONTEND_HTML: &str = r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>微前端</title>
<style>body{font-family:sans-serif;padding:8px;background:#f0f4ff;margin:0}
h4{margin:4px 0;color:#1976d2}
#log{font-size:12px;color:#333}
.msg{background:#e3f2fd;padding:4px 8px;margin:2px 0;border-radius:4px}
.reply{background:#c8e6c9;padding:4px 8px;margin:2px 0;border-radius:4px}</style>
</head>
<body>
<h4>🧩 微前端 App</h4>
<div id="log"><div class="msg">✅ 微前端已启动</div></div>
<script>
window.addEventListener('message', function(e) {
    var log = document.getElementById('log');
    var div = document.createElement('div');
    div.className = 'reply';
    div.textContent = '📩 收到: ' + (e.data.text || '(空)');
    log.appendChild(div);
    window.parent.postMessage({
        type: 'micro_reply',
        text: '微前端已处理: ' + (e.data.text || '(空)'),
        timestamp: Date.now()
    }, '*');
});
window.parent.postMessage({type: 'micro_ready', text: '微前端已就绪'}, '*');
</script>
</body></html>"#;

// 使用 inline_js 创建 Blob URL 的 JS 辅助函数
#[wasm_bindgen(inline_js = r#"
export function _create_blob_url(html) {
    var blob = new Blob([html], {type: 'text/html'});
    return URL.createObjectURL(blob);
}
"#)]
extern "C" {
    fn _create_blob_url(html: &str) -> String;
}

#[component]
fn Exercise() -> impl IntoView {
    let messages: RwSignal<Vec<String>> = RwSignal::new(Vec::new());
    let input_text = RwSignal::new(String::new());
    let micro_ready = RwSignal::new(false);
    let iframe_ref: NodeRef<leptos::html::Iframe> = NodeRef::new();

    // 生成微前端的 blob URL
    let src = _create_blob_url(MICRO_FRONTEND_HTML);

    // 监听来自微前端的 message 事件
    window_event_listener(ev::message, move |event: web_sys::MessageEvent| {
        if let Some(data) = event.data().as_string() {
            // Try to parse as JSON or use raw text
            messages.update(|log| {
                log.push(format!("[微前端] {}", data));
            });
        } else {
            // Handle JsValue objects — use js_sys::Reflect would need js-sys dep
            // For simplicity, just show a generic message
            messages.update(|log| {
                log.push("[微前端] 收到消息对象".to_string());
            });
        }

        // Check for micro_ready signal
        if let Some(data) = event.data().as_string() {
            if data.contains("micro_ready") || data.contains("微前端已就绪") {
                micro_ready.set(true);
            }
        }
    });

    // 发送消息到微前端
    let send_message = move |_| {
        let text = input_text.get();
        if text.trim().is_empty() {
            return;
        }

        if let Some(iframe_element) = iframe_ref.get() {
            let iframe: web_sys::HtmlIFrameElement = iframe_element.unchecked_into();
            if let Some(content_window) = iframe.content_window() {
                let msg = JsValue::from_str(&format!(r#"{{"text":"{}"}}"#, text));
                let _ = content_window.post_message(&msg, "*");
                messages.update(|log| {
                    log.push(format!("[主应用] 发送: {}", text));
                });
                input_text.set(String::new());
            } else {
                messages.update(|log| {
                    log.push("[系统] 无法获取 iframe 的 contentWindow".to_string());
                });
            }
        } else {
            messages.update(|log| {
                log.push("[系统] iframe 尚未挂载".to_string());
            });
        }
    };

    view! {
        <div>
            <h2>"🏗️ 微前端集成"</h2>

            <div style="display:flex;gap:16px;flex-wrap:wrap">
                <div style="flex:1;min-width:300px">
                    <h3>"主应用 (Container)"</h3>
                    <div>
                        <p>"微前端状态: " {move || if micro_ready.get() { "✅ 已就绪" } else { "⏳ 加载中..." }}</p>
                    </div>
                    <div>
                        <input
                            type="text"
                            placeholder="发送消息到微前端..."
                            prop:value={move || input_text.get()}
                            on:input=move |ev| {
                                input_text.set(event_target_value(&ev));
                            }
                        />
                        <button on:click=send_message>"发送"</button>
                    </div>
                    <div>
                        <h4>"消息日志"</h4>
                        <div style="max-height:250px;overflow-y:auto;border:1px solid #ddd;padding:8px;border-radius:4px">
                            {move || messages.get().iter().map(|msg| {
                                view! { <div>{msg.clone()}</div> }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </div>
                <div style="flex:1;min-width:300px">
                    <h3>"微前端 (Micro App)"</h3>
                    <iframe
                        node_ref=iframe_ref
                        src={src}
                        style="width:100%;height:300px;border:2px solid #1976d2;border-radius:8px"
                    ></iframe>
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
