// ============================================================
// 练习 e136: document_window_access
//
// 目标: 访问 document / window 全局对象
//
// 难度: ⭐⭐
// 核心知识点: document() / window()
//
// TODO: 使用 leptos::prelude 提供的 document() 和 window() 函数
//       访问浏览器全局对象，显示文档标题、URL 和视口尺寸
// ============================================================

use leptos::prelude::*;

#[component]
fn DocumentWindowAccess() -> impl IntoView {
    // 获取 document / window
    let doc = document();
    let win = window();
    let title = doc.title();
    let href = win.location().href().unwrap_or_default();
    // inner_width/inner_height 返回 Result<f64, JsValue>
    let width = win.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(0.0);
    let height = win.inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(0.0);

    view! {
        <div>
            <h2>"练习 e136: Document / Window 访问"</h2>
            <p>"标题: " {title}</p>
            <p>"URL: " {href}</p>
            <p>"视口: " {format!("{width:.0} × {height:.0}")}</p>
        </div>
    }
}

fn main() {
    mount_to_body(DocumentWindowAccess);
}

// ============================================================
// 参考答案
// ============================================================
// <details>
// <summary>点击展开</summary>
//
// ```rust
// use leptos::prelude::*;
// let doc = document();      // web_sys::Document
// let win = window();        // web_sys::Window
// let title = doc.title();
// let href = win.location().href().unwrap_or_default();
// let w = win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
// let h = win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
// ```
//
// `document()` 和 `window()` 是线程局部缓存函数。
// `inner_width()` 返回 `Result<f64, JsValue>`。
//
// </details>
