// ============================================================
// 练习 e148: use_media_query — 媒体查询响应式封装
//
// 目标: 将 CSS 媒体查询封装为响应式信号
//
// 难度: ⭐⭐⭐
// 核心知识点: window.matchMedia, Closure 回调, 响应式封装
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// 绑定 window.innerWidth
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window"])]
    fn innerWidth() -> f64;
}

/// 从媒体查询字符串中提取阈值 (如 "(min-width: 600px)" -> 600.0)
fn extract_threshold(query: &str) -> f64 {
    query
        .split(|c: char| !c.is_ascii_digit() && c != '.')
        .filter_map(|s| s.parse::<f64>().ok())
        .next()
        .unwrap_or(0.0)
}

/// Hook: 监听媒体查询变化（通过 innerWidth + resize 事件模拟）
fn use_media_query(query: &str) -> ReadSignal<bool> {
    let threshold = extract_threshold(query);
    let (is_match, set_is_match) = signal(innerWidth() >= threshold);

    window_event_listener(ev::resize, move |_| {
        set_is_match.set(innerWidth() >= threshold);
    });

    is_match
}

#[component]
fn Exercise() -> impl IntoView {
    let is_wide = use_media_query("(min-width: 600px)");

    view! {
        <div>
            <h2>"e148: use_media_query"</h2>
            <p>
                "当前视口: "
                {move || if is_wide.get() { "宽屏 (≥600px)" } else { "窄屏 (<600px)" }}
            </p>
            <p>"缩小/放大浏览器窗口查看变化"</p>
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
// use leptos::ev;
// use leptos::prelude::*;
// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window"])]
//     fn innerWidth() -> f64;
// }
//
// fn use_media_query(query: &str) -> ReadSignal<bool> {
//     let threshold = query.split(|c: char| !c.is_ascii_digit() && c != '.')
//         .filter_map(|s| s.parse::<f64>().ok()).next().unwrap_or(0.0);
//     let (is_match, set_is_match) = signal(innerWidth() >= threshold);
//     window_event_listener(ev::resize, move |_| {
//         set_is_match.set(innerWidth() >= threshold);
//     });
//     is_match
// }
// ```
//
// ### 知识点
// - `window.innerWidth` 获取视口宽度，通过 `#[wasm_bindgen]` 绑定
// - `window_event_listener(ev::resize, ...)` 监听窗口大小变化
// - 从媒体查询字符串中提取阈值实现通用 Hook
// - ponytail: 完整实现应使用 `window.matchMedia` + `addEventListener("change")`
//   当 `web-sys` 的 `MediaQueryList` feature 可用时升级
//
// </details>
