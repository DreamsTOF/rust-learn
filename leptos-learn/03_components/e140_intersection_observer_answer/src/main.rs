// ============================================================
// 练习 e140: intersection_observer - 答案
// ============================================================

use leptos::prelude::*;
use std::cell::Cell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;

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
