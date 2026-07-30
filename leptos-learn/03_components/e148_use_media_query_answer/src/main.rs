// ============================================================
// Exercise 148 - Answer: use_media_query
// ============================================================

use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
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
