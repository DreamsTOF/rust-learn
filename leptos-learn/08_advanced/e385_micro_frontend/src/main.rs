// ============================================================
// 练习 e385: 微前端集成 — iframe 嵌入与跨文档消息通信
//
// 核心知识点:
//   - 使用 <iframe> 嵌入微前端页面
//   - window_event_listener(ev::message, ...) 监听跨域消息
//   - iframe.contentWindow.postMessage 发送消息
//   - Blob URL 内联生成微前端 HTML
//
// 难度: ⭐⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;
use leptos::ev;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// 微前端 HTML 模板 — 一个自包含的迷你应用
const MICRO_FRONTEND_HTML: &str = r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>微前端</title>
<style>body{font-family:sans-serif;padding:8px;background:#f0f4ff;margin:0}
h4{margin:4px 0;color:#1976d2}
#log{font-size:12px;color:#333}
.msg{background:#e3f2fd;padding:4px 8px;margin:2px 0;border-radius:4px}</style>
</head>
<body>
<h4>🧩 微前端 App</h4>
<div id="log"><div class="msg">微前端已就绪</div></div>
<script>
window.addEventListener('message', function(e) {
    var log = document.getElementById('log');
    var div = document.createElement('div');
    div.className = 'msg';
    div.textContent = '收到: ' + (e.data.text || '(空)');
    log.appendChild(div);
    window.parent.postMessage({
        type: 'micro_reply',
        text: '微前端已处理: ' + (e.data.text || '(空)'),
        timestamp: Date.now()
    }, '*');
});
// 通知父应用微前端已就绪
window.parent.postMessage({type: 'micro_ready', text: '微前端已就绪'}, '*');
</script>
</body></html>"#;

// TODO 1: 使用 wasm_bindgen inline_js 定义 _create_blob_url 函数
// - 从 HTML 字符串创建 Blob 对象
// - 使用 URL.createObjectURL 生成 URL
// - 返回 blob URL 字符串
#[wasm_bindgen(inline_js = r#"
// TODO: 实现 createBlobURL(html) 函数
// - new Blob([html], {type: 'text/html'})
// - URL.createObjectURL(blob)
// - return url
"#)]
extern "C" {
    fn _create_blob_url(html: &str) -> String;
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO 2: 创建所需的信号
    // - messages: RwSignal<Vec<String>> — 通信日志
    // - input_text: RwSignal<String> — 输入框内容
    // - micro_ready: RwSignal<bool> — 微前端是否就绪

    // TODO 3: 创建 iframe 的 node_ref
    // let iframe_ref: NodeRef<leptos::html::Iframe> = NodeRef::new();

    // TODO 4: 使用 window_event_listener 监听 message 事件
    // - 检查 event.origin 或 event.source 判断消息来源
    // - 将收到的消息添加到 messages 日志
    // - 当收到 micro_ready 消息时，更新 micro_ready 信号

    // TODO 5: 创建 postMessage 发送函数
    // - 通过 iframe_ref 获取 contentWindow
    // - 调用 contentWindow.postMessage({text: ...}, '*')

    // TODO 6: 生成 iframe 的 blob URL（组件初始化时）
    // let src = _create_blob_url(MICRO_FRONTEND_HTML);

    view! {
        <div>
            <h2>"🏗️ 微前端集成"</h2>

            <div style="display:flex;gap:16px;flex-wrap:wrap">
                <div style="flex:1;min-width:300px">
                    <h3>"主应用 (Container)"</h3>
                    // TODO 7: 创建输入框和发送按钮
                    // TODO 8: 显示 micro_ready 状态
                    <div>
                        <input type="text" placeholder="发送消息到微前端..." />
                        <button>"发送"</button>
                    </div>
                    <div>
                        <h4>"消息日志"</h4>
                        // TODO 9: 遍历 messages 信号显示所有消息
                    </div>
                </div>
                <div style="flex:1;min-width:300px">
                    <h3>"微前端 (Micro App)"</h3>
                    // TODO 10: 使用 iframe 嵌入微前端页面
                    // 设置 node_ref=iframe_ref 以便访问 contentWindow
                    // 设置 src=生成的 blob URL
                    // 设置 style="width:100%;height:300px;border:2px solid #1976d2;border-radius:8px"
                </div>
            </div>
        </div>
    }
}

 fn main() {
     mount_to_body(Exercise);
 }
