// ============================================================
// 练习 e141: ResizeObserver — 元素尺寸变化响应式监听
//
// 核心知识点:
//   - ResizeObserver 构造函数
//   - Closure 包装 Rust 回调供 JS 调用
//   - observe() 开始监听 / disconnect() 停止监听
//   - js_sys::Reflect 读取 JS 对象属性
//
// 难度: ⭐⭐⭐ (理解 Closure 生命周期)
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::JsCast;

fn main() {
    mount_to_body(move || {
        let (size, set_size) = signal("拖动右下角以调整大小".to_string());
        let div_ref: NodeRef<leptos::html::Div> = NodeRef::new();

        // === 步骤 1: 创建 ResizeObserver 回调 ===
        // TODO: 使用 wasm_bindgen::closure::Closure 包装回调函数
        // 回调接收 entries: js_sys::Array，从中读取第一个 entry 的 contentRect
        let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move |entries: js_sys::Array| {
            if entries.length() > 0 {
                let entry = entries.get(0);
                // 通过 js_sys::Reflect 读取 contentRect 属性
                let rect =
                    js_sys::Reflect::get(&entry, &"contentRect".into()).unwrap();
                let w = js_sys::Reflect::get(&rect, &"width".into())
                    .unwrap()
                    .as_f64()
                    .unwrap() as i32;
                let h = js_sys::Reflect::get(&rect, &"height".into())
                    .unwrap()
                    .as_f64()
                    .unwrap() as i32;
                set_size.set(format!("{} × {} px", w, h));
            }
        }) as Box<dyn Fn(js_sys::Array)>);

        // === 步骤 2: 实例化 ResizeObserver ===
        // TODO: 传入 js_sys::Function（通过 .as_ref().unchecked_ref() 转换）
        // 注意: 需要 forget() 防止回调被释放
        let observer =
            web_sys::ResizeObserver::new(cb.as_ref().unchecked_ref()).unwrap();
        cb.forget(); // 回调在 JS 侧永久存活

        // === 步骤 3: 在元素挂载后开始观测 ===
        // TODO: 使用 Effect + NodeRef 在元素挂载后调用 observer.observe()
        Effect::new(move |_| {
            if let Some(el) = div_ref.get() {
                observer.observe(el.as_ref());
            }
        });

        view! {
            <div>
                <p>"调整下方蓝色方块右下角 ↙ 以改变尺寸："</p>
                <div
                    node_ref=div_ref
                    style="
                        resize: both; overflow: auto;
                        width: 200px; height: 100px;
                        border: 2px solid #4A90D9;
                        background: #E8F0FE;
                        padding: 8px; border-radius: 4px;
                        font-family: monospace; font-size: 16px;
                    "
                >
                    <p><strong>"尺寸: " {size}</strong></p>
                </div>
            </div>
        }
    });
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// use leptos::prelude::*;
//
// fn main() {
//     mount_to_body(move || {
//         let (size, set_size) = signal("拖动右下角以调整大小".to_string());
//         let div_ref: NodeRef<leptos::html::Div> = NodeRef::new();
//
//         let cb = wasm_bindgen::closure::Closure::wrap(
//             Box::new(move |entries: js_sys::Array| {
//                 if entries.length() > 0 {
//                     let entry = entries.get(0);
//                     let rect =
//                         js_sys::Reflect::get(&entry, &"contentRect".into()).unwrap();
//                     let w = js_sys::Reflect::get(&rect, &"width".into())
//                         .unwrap().as_f64().unwrap() as i32;
//                     let h = js_sys::Reflect::get(&rect, &"height".into())
//                         .unwrap().as_f64().unwrap() as i32;
//                     set_size.set(format!("{} × {} px", w, h));
//                 }
//             }) as Box<dyn Fn(js_sys::Array)>,
//         );
//
//         let observer =
//             web_sys::ResizeObserver::new(cb.as_ref().unchecked_ref()).unwrap();
//         cb.forget();
//
//         Effect::new(move |_| {
//             if let Some(el) = div_ref.get() {
//                 observer.observe(el.as_ref());
//             }
//         });
//
//         view! {
//             <div>
//                 <p>"调整下方蓝色方块右下角 ↙ 以改变尺寸："</p>
//                 <div
//                     node_ref=div_ref
//                     style="resize: both; overflow: auto; width: 200px; height: 100px;
//                            border: 2px solid #4A90D9; background: #E8F0FE;
//                            padding: 8px; border-radius: 4px;
//                            font-family: monospace; font-size: 16px;"
//                 >
//                     <p><strong>"尺寸: " {size}</strong></p>
//                 </div>
//             </div>
//         }
//     });
// }
// ```
//
// ### 知识点
// 1. ResizeObserver 可以监听任意 HTML 元素的尺寸变化
// 2. wasm-bindgen 的 Closure 用于将 Rust 闭包转为 JS 函数
// 3. `.forget()` 让闭包在 JS 侧永久存活，避免被 Rust 所有权释放
// 4. `js_sys::Reflect::get()` 可动态读取 JS 对象属性
// 5. CSS `resize: both` 让 div 出现拖动手柄，方便测试
// </details>
