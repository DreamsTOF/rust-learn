// ============================================================
// 练习 e140: intersection_observer
//
// 目标: 使用 IntersectionObserver API 检测元素进入/离开视口
//
// 难度: ⭐⭐⭐
// 核心知识点: IntersectionObserver
//
// TODO: 利用 wasm_bindgen + inline_js 创建 IntersectionObserver，
//       监测目标元素的可见性
// ============================================================

use leptos::prelude::*;
use std::cell::Cell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;

// 将 IntersectionObserver 封装在 inline JS 中
#[wasm_bindgen(inline_js = r#"
export function observeIntersection(element, onEnter, onLeave) {
    const observer = new IntersectionObserver(
        (entries) => {
            if (entries[0].isIntersecting) onEnter(); else onLeave();
        },
        { threshold: 0.2 }
    );
    observer.observe(element);
    return observer;
}
"#)]
extern "C" {
    fn observeIntersection(el: &JsValue, onEnter: &JsValue, onLeave: &JsValue) -> JsValue;
}

#[component]
fn IntersectionObserverDemo() -> impl IntoView {
    let (visible, set_visible) = signal(false);
    let (visible_count, set_visible_count) = signal(0);
    let once = Cell::new(false);

    // 组件挂载后查找目标元素并创建 IntersectionObserver
    Effect::new(move || {
        if once.get() {
            return;
        }
        if let Some(el) = document().get_element_by_id("target-div") {
            once.set(true);

            let on_enter = {
                let v = set_visible.clone();
                let vc = set_visible_count.clone();
                Closure::new(Box::new(move || {
                    v.set(true);
                    vc.update(|n| *n += 1);
                }) as Box<dyn FnMut()>)
            };

            let on_leave = {
                Closure::new(Box::new(move || {
                    set_visible.set(false);
                }) as Box<dyn FnMut()>)
            };

            observeIntersection(&JsValue::from(el), on_enter.as_ref(), on_leave.as_ref());
            on_enter.forget();
            on_leave.forget();
        }
    });

    view! {
        <div>
            <h2>"练习 e140: IntersectionObserver"</h2>
            <p>"状态: "
                {move || if visible() { "🟢 可见" } else { "🔴 不可见" }}
            </p>
            <p>"可见次数: " {visible_count}</p>

            <div style="height: 150vh; background: #eee; display: flex; align-items: center; justify-content: center;">
                "⬇️ 向下滚动 ⬇️"
            </div>

            <div
                id="target-div"
                style="height: 200px; background: {move || if visible() { \"#4caf50\" } else { \"#f44336\" }}; display: flex; align-items: center; justify-content: center; border-radius: 8px; transition: background 0.3s; color: #fff;"
            >
                {move || if visible() { "元素在视口中！" } else { "滚动到此处..." }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(IntersectionObserverDemo);
}

// ============================================================
// 参考答案
// ============================================================
// <details>
// <summary>点击展开</summary>
//
// ```rust
// #[wasm_bindgen(inline_js = r#"export function observeIntersection(el, onEnter, onLeave) {
//     new IntersectionObserver((e) => { if (e[0].isIntersecting) onEnter(); else onLeave(); },
//         { threshold: 0.2 }).observe(el); }"#)]
// extern "C" { fn observeIntersection(el: &JsValue, onEnter: &JsValue, onLeave: &JsValue) -> JsValue; }
//
// let on_enter = Closure::new(Box::new(|| { /* ... */ }) as Box<dyn FnMut()>);
// observeIntersection(&JsValue::from(element), on_enter.as_ref(), on_leave.as_ref());
// on_enter.forget(); // 防止 GC
// ```
//
// - `inline_js` 内联 JS 代码，无需外部文件
// - `Closure::new` / `forget()` 管理 Rust 闭包在 JS 中的生命周期
// - `document().get_element_by_id()` 获取 DOM 元素
// - 适用于懒加载、无限滚动、曝光统计
//
// </details>
