// ============================================================
// 参考答案 e366: 无限滚动加载 — IntersectionObserver 触发加载更多
//
// 核心知识点:
//   - IntersectionObserver API 检测哨兵元素进入视口
//   - wasm_bindgen::closure::Closure 包装 JS 回调
//   - Effect::new + Cell<bool> 做一次性初始化
//   - 模拟异步分批加载数据
// ============================================================

use leptos::prelude::*;
use std::cell::Cell;
use std::time::Duration;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// 通过 inline_js 定义 IntersectionObserver 封装
#[wasm_bindgen(inline_js = r#"
export function observeIntersection(element, callback) {
    const observer = new IntersectionObserver(
        (entries) => {
            if (entries[0].isIntersecting) callback();
        },
        { threshold: 0.1 }
    );
    observer.observe(element);
    return observer;
}
"#)]
extern "C" {
    fn observeIntersection(el: &JsValue, callback: &JsValue) -> JsValue;
}

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(Vec::<String>::new());
    let (loading, set_loading) = signal(false);
    let (has_more, set_has_more) = signal(true);
    let initialized = Cell::new(false);

    // 加载更多数据（模拟异步 API）
    let load_more = {
        let set_items = set_items.clone();
        let set_loading = set_loading.clone();
        let set_has_more = set_has_more.clone();
        move || {
            if loading() || !has_more() {
                return;
            }
            set_loading(true);
            let current_len = items().len();
            set_timeout(
                move || {
                    let mut new_items: Vec<String> = (current_len + 1..=current_len + 10)
                        .map(|i| format!("项目 #{} — 模拟加载的数据项", i))
                        .collect();
                    set_items.update(|v| v.append(&mut new_items));
                    set_loading(false);
                    if items().len() >= 100 {
                        set_has_more(false);
                    }
                },
                Duration::from_millis(800),
            );
        }
    };

    // 首次加载
    load_more();

    // 设置 IntersectionObserver — 一次性初始化
    Effect::new(move || {
        if initialized.get() {
            return;
        }
        if let Some(sentinel) = document().get_element_by_id("sentinel") {
            let callback = Closure::new(Box::new(load_more.clone()) as Box<dyn FnMut()>);
            observeIntersection(&JsValue::from(sentinel), callback.as_ref());
            callback.forget();
            initialized.set(true);
        }
    });

    view! {
        <div style="padding: 1rem; font-family: sans-serif; max-width: 480px; margin: 0 auto;">
            <h3>"练习 e366: 无限滚动（IntersectionObserver）"</h3>
            <p style="color: #666; font-size: 14px;">
                "滚动到底部 — 哨兵元素进入视口时自动加载更多"
            </p>

            <div style="border: 1px solid #ddd; border-radius: 8px; padding: 4px 12px;">
                {move || {
                    items()
                        .into_iter()
                        .enumerate()
                        .map(|(_idx, item)| {
                            view! {
                                <div
        
                                    style="padding: 10px 8px; border-bottom: 1px solid #f0f0f0; font-size: 15px;"
                                >
                                    {item}
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>

            // 哨兵元素 — 用于 IntersectionObserver 检测
            <div id="sentinel" style="height: 1px;"></div>

            <div style="text-align: center; padding: 12px; color: #888;">
                {move || {
                    if loading() {
                        view! { <span>"⏳ 加载中..."</span> }.into_any()
                    } else if !has_more() {
                        view! { <span>"— 已加载全部数据 —"</span> }.into_any()
                    } else {
                        view! { <span>"⬇️ 滚动加载更多 ⬇️"</span> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
