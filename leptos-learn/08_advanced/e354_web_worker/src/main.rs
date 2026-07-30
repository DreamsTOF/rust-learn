// ============================================================
// 练习 e354: Web Worker — 在后台线程计算斐波那契数
//
// 核心知识点:
//   - #[wasm_bindgen(inline_js = "...")] 嵌入内联 JS
//   - Web Worker 在独立线程执行计算，不阻塞 UI
//   - 通过 postMessage / onmessage 与 Worker 通信
//   - 用 Promise 桥接 JS Worker 和 Rust Future
//
// 难度: ⭐⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// 内联 JS：创建一个 Web Worker 计算斐波那契数
/// 返回 Promise<number>，计算完成后 resolve
#[wasm_bindgen(inline_js = r#"
export function run_worker(input) {
    return new Promise((resolve, reject) => {
        // Worker 代码（以 blob URL 形式创建）
        const workerCode = `
            self.onmessage = function(e) {
                function fib(n) {
                    if (n <= 1) return n;
                    return fib(n - 1) + fib(n - 2);
                }
                self.postMessage(fib(e.data));
            };
        `;
        const blob = new Blob([workerCode], { type: 'application/javascript' });
        const worker = new Worker(URL.createObjectURL(blob));
        worker.postMessage(input);
        worker.onmessage = (e) => {
            resolve(e.data);
            worker.terminate();
        };
        worker.onerror = (e) => reject(e.error);
    });
}
"#)]
extern "C" {
    fn run_worker(input: i32) -> js_sys::Promise;
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 用 signal 保存计算结果字符串
    let (result, set_result) = signal("点击按钮开始计算".to_string());

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 创建 LocalResource，在异步块中调用 run_worker(42)
    //   - 用 JsFuture::from(promise).await 等待 Worker 完成
    //   - 将结果转为 i32
    let fib_task = LocalResource::new(|| async move {
        let promise = run_worker(42);
        let val = wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .unwrap();
        val.as_f64().unwrap() as i32
    });

    view! {
        <div>
            <h2>"练习 e354: Web Worker 后台计算"</h2>
            <p>"点击按钮在 Web Worker 中计算 fib(42)，UI 不会阻塞。"</p>
            <button on:click=move |_| {
                set_result.set("计算中...".to_string());
                fib_task.refetch();
            }>
                "开始计算 fib(42)"
            </button>
            // TODO: 显示计算结果
            //   - fib_task.get() 返回 Option<i32>
            //   - 用 set_result 或直接显示
            <p>
                "结果: "
                {move || match fib_task.get() {
                    None => "等待计算...".to_string(),
                    Some(n) => format!("fib(42) = {}", n),
                }}
            </p>
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
// ### 完整代码
// ```rust
// use leptos::prelude::*;
// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen(inline_js = r#"
// export function run_worker(input) {
//     return new Promise((resolve, reject) => {
//         const workerCode = `
//             self.onmessage = function(e) {
//                 function fib(n) {
//                     if (n <= 1) return n;
//                     return fib(n - 1) + fib(n - 2);
//                 }
//                 self.postMessage(fib(e.data));
//             };
//         `;
//         const blob = new Blob([workerCode], { type: 'application/javascript' });
//         const worker = new Worker(URL.createObjectURL(blob));
//         worker.postMessage(input);
//         worker.onmessage = (e) => {
//             resolve(e.data);
//             worker.terminate();
//         };
//         worker.onerror = (e) => reject(e.error);
//     });
// }
// "#)]
// extern "C" {
//     fn run_worker(input: i32) -> js_sys::Promise;
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (result, set_result) = signal("点击按钮开始计算".to_string());
//
//     let fib_task = LocalResource::new(|| async move {
//         let promise = run_worker(42);
//         let val = wasm_bindgen_futures::JsFuture::from(promise)
//             .await
//             .unwrap();
//         val.as_f64().unwrap() as i32
//     });
//
//     view! {
//         <div>
//             <h2>"练习 e354: Web Worker 后台计算"</h2>
//             <p>"点击按钮在 Web Worker 中计算 fib(42)，UI 不会阻塞。"</p>
//             <button on:click=move |_| {
//                 set_result.set("计算中...".to_string());
//                 fib_task.refetch();
//             }>
//                 "开始计算 fib(42)"
//             </button>
//             <p>
//                 "结果: "
//                 {move || match fib_task.get() {
//                     None => "等待计算...".to_string(),
//                     Some(n) => format!("fib(42) = {}", n),
//                 }}
//             </p>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - `#[wasm_bindgen(inline_js = r#"..."#)]` 在 Rust 中嵌入原生 JS 代码
// - Web Worker 在独立线程运行，通过 postMessage/onmessage 通信
// - JS Promise 通过 `JsFuture::from(promise).await` 转为 Rust Future
// - `LocalResource::refetch()` 手动触发资源重新加载
// - `worker.terminate()` 用完后及时销毁 Worker
//
// </details>
