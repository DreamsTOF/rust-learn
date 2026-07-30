// ============================================================
// 答案 e354: Web Worker — 后台计算斐波那契数
//
// 完整可编译实现，不含 TODO。
// 通过内联 JS 创建 Web Worker，异步获取计算结果。
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// 内联 JS：创建 Web Worker 计算 fib(n)，返回 Promise<number>
#[wasm_bindgen(inline_js = r#"
export function run_worker(input) {
    return new Promise((resolve, reject) => {
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
    // 创建 LocalResource，在 Worker 中计算斐波那契数
    let fib_task = LocalResource::new(|| async move {
        let promise = run_worker(42);
        let val = wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .unwrap();
        val.as_f64().unwrap() as i32
    });

    view! {
        <div>
            <h2>"答案 e354: Web Worker 后台计算"</h2>
            <p>"点击按钮在 Web Worker 中计算 fib(42)，UI 不会阻塞。"</p>
            <button on:click=move |_| fib_task.refetch()>
                "重新计算 fib(42)"
            </button>
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
