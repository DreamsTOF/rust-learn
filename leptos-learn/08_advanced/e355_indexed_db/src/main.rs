// ============================================================
// 练习 e355: IndexedDB — 在浏览器中做 CRUD 存取数据
//
// 核心知识点:
//   - 通过 #[wasm_bindgen(inline_js)] 封装 IndexedDB API
//   - indexedDB.open / createObjectStore / put / get / delete
//   - 用 Promise 桥接 JS IndexedDB 操作和 Rust Future
//   - IndexedDB 数据持久化（刷新页面后数据保留）
//
// 难度: ⭐⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;

/// 内联 JS：封装 IndexedDB 的 set / get / delete 操作
/// 每个函数返回 Promise，完成后 resolve
#[wasm_bindgen(inline_js = r#"
function withDb(mode, callback) {
    return new Promise((resolve, reject) => {
        const request = indexedDB.open('LeptosDB', 1);
        request.onupgradeneeded = (e) => {
            e.target.result.createObjectStore('store', { keyPath: 'id' });
        };
        request.onsuccess = (e) => {
            const db = e.target.result;
            const tx = db.transaction('store', mode);
            callback(tx.objectStore('store'), db, tx, resolve, reject);
        };
        request.onerror = (e) => reject(e.target.error);
    });
}

export function dbSet(key, value) {
    return withDb('readwrite', (store, db, tx, resolve, reject) => {
        store.put({ id: key, value });
        tx.oncomplete = () => { db.close(); resolve(); };
        tx.onerror = (e) => reject(e.target.error);
    });
}

export function dbGet(key) {
    return withDb('readonly', (store, db, tx, resolve, reject) => {
        const req = store.get(key);
        req.onsuccess = () => {
            db.close();
            resolve(req.result ? req.result.value : null);
        };
        req.onerror = (e) => reject(e.target.error);
    });
}

export function dbDelete(key) {
    return withDb('readwrite', (store, db, tx, resolve, reject) => {
        store.delete(key);
        tx.oncomplete = () => { db.close(); resolve(); };
        tx.onerror = (e) => reject(e.target.error);
    });
}
"#)]
extern "C" {
    fn dbSet(key: &str, value: &str) -> js_sys::Promise;
    fn dbGet(key: &str) -> js_sys::Promise;
    fn dbDelete(key: &str) -> js_sys::Promise;
}

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建信号存储输入值、已保存数据和状态信息
    let (input_value, set_input_value) = signal(String::new());
    let (saved_data, set_saved_data) = signal::<Option<String>>(None);
    let (status, set_status) = signal("准备就绪".to_string());

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 使用 spawn_local + JsFuture 实现保存功能
    let on_save = move |_| {
        let value = input_value.get();
        if value.trim().is_empty() {
            set_status.set("⚠️ 输入不能为空".to_string());
            return;
        }
        set_status.set("保存中...".to_string());
        spawn_local({
            let value = value.clone();
            async move {
                let promise = dbSet("my_data", &value);
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                set_saved_data.set(Some(value));
                set_status.set("✅ 已保存到 IndexedDB".to_string());
            }
        });
    };

    // === 步骤 3 ——————————————————————————————————————————
    // TODO: 实现加载功能：调用 dbGet，处理 null/undefined
    let on_load = move |_| {
        set_status.set("加载中...".to_string());
        spawn_local(async move {
            let promise = dbGet("my_data");
            let val = wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .unwrap_throw();
            if val.is_null() || val.is_undefined() {
                set_status.set("⚠️ 未找到数据".to_string());
            } else {
                let s = val.as_string().unwrap_or_default();
                set_input_value.set(s.clone());
                set_saved_data.set(Some(s));
                set_status.set("✅ 已从 IndexedDB 加载".to_string());
            }
        });
    };

    // === 步骤 4 ——————————————————————————————————————————
    // TODO: 实现删除功能
    let on_delete = move |_| {
        set_status.set("删除中...".to_string());
        spawn_local(async move {
            let promise = dbDelete("my_data");
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            set_input_value.set(String::new());
            set_saved_data.set(None);
            set_status.set("🗑️ 已从 IndexedDB 删除".to_string());
        });
    };

    view! {
        <div>
            <h2>"练习 e355: IndexedDB 数据存取"</h2>
            <p>"输入内容后保存，刷新页面后加载，数据应持久保留。"</p>
            <input
                type="text"
                prop:value={move || input_value.get()}
                on:input=move |ev| {
                    set_input_value.set(event_target_value(&ev));
                }
                placeholder="输入要存储的内容..."
                style="width: 300px; padding: 4px;"
            />
            <div style="margin: 8px 0;">
                <button on:click=on_save>"💾 保存"</button>
                <button on:click=on_load>"📂 加载"</button>
                <button on:click=on_delete>"🗑️ 删除"</button>
            </div>
            <p>"状态: " {move || status.get()}</p>
            // TODO: 显示已保存的数据
            {move || saved_data.get().map(|data| view! {
                <div style="margin-top: 8px; padding: 8px; background: #f0f0f0; border-radius: 4px;">
                    <p>"已保存的数据: " {data}</p>
                </div>
            })}
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
// use leptos::task::spawn_local;
// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen(inline_js = r#"
// function withDb(mode, callback) {
//     return new Promise((resolve, reject) => {
//         const request = indexedDB.open('LeptosDB', 1);
//         request.onupgradeneeded = (e) => {
//             e.target.result.createObjectStore('store', { keyPath: 'id' });
//         };
//         request.onsuccess = (e) => {
//             const db = e.target.result;
//             const tx = db.transaction('store', mode);
//             callback(tx.objectStore('store'), db, tx, resolve, reject);
//         };
//         request.onerror = (e) => reject(e.target.error);
//     });
// }
//
// export function dbSet(key, value) {
//     return withDb('readwrite', (store, db, tx, resolve, reject) => {
//         store.put({ id: key, value });
//         tx.oncomplete = () => { db.close(); resolve(); };
//         tx.onerror = (e) => reject(e.target.error);
//     });
// }
//
// export function dbGet(key) {
//     return withDb('readonly', (store, db, tx, resolve, reject) => {
//         const req = store.get(key);
//         req.onsuccess = () => {
//             db.close();
//             resolve(req.result ? req.result.value : null);
//         };
//         req.onerror = (e) => reject(e.target.error);
//     });
// }
//
// export function dbDelete(key) {
//     return withDb('readwrite', (store, db, tx, resolve, reject) => {
//         store.delete(key);
//         tx.oncomplete = () => { db.close(); resolve(); };
//         tx.onerror = (e) => reject(e.target.error);
//     });
// }
// "#)]
// extern "C" {
//     fn dbSet(key: &str, value: &str) -> js_sys::Promise;
//     fn dbGet(key: &str) -> js_sys::Promise;
//     fn dbDelete(key: &str) -> js_sys::Promise;
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (input_value, set_input_value) = signal(String::new());
//     let (saved_data, set_saved_data) = signal::<Option<String>>(None);
//     let (status, set_status) = signal("准备就绪".to_string());
//
//     let on_save = move |_| {
//         let value = input_value.get();
//         if value.trim().is_empty() {
//             set_status.set("⚠️ 输入不能为空".to_string());
//             return;
//         }
//         set_status.set("保存中...".to_string());
//         spawn_local({
//             let value = value.clone();
//             async move {
//                 let promise = dbSet("my_data", &value);
//                 let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
//                 set_saved_data.set(Some(value));
//                 set_status.set("✅ 已保存到 IndexedDB".to_string());
//             }
//         });
//     };
//
//     let on_load = move |_| {
//         set_status.set("加载中...".to_string());
//         spawn_local(async move {
//             let promise = dbGet("my_data");
//             let val = wasm_bindgen_futures::JsFuture::from(promise)
//                 .await
//                 .unwrap_throw();
//             if val.is_null() || val.is_undefined() {
//                 set_status.set("⚠️ 未找到数据".to_string());
//             } else {
//                 let s = val.as_string().unwrap_or_default();
//                 set_input_value.set(s.clone());
//                 set_saved_data.set(Some(s));
//                 set_status.set("✅ 已从 IndexedDB 加载".to_string());
//             }
//         });
//     };
//
//     let on_delete = move |_| {
//         set_status.set("删除中...".to_string());
//         spawn_local(async move {
//             let promise = dbDelete("my_data");
//             let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
//             set_input_value.set(String::new());
//             set_saved_data.set(None);
//             set_status.set("🗑️ 已从 IndexedDB 删除".to_string());
//         });
//     };
//
//     view! {
//         <div>
//             <h2>"练习 e355: IndexedDB 数据存取"</h2>
//             <p>"输入内容后保存，刷新页面后加载，数据应持久保留。"</p>
//             <input
//                 type="text"
//                 prop:value={move || input_value.get()}
//                 on:input=move |ev| {
//                     set_input_value.set(event_target_value(&ev));
//                 }
//                 placeholder="输入要存储的内容..."
//                 style="width: 300px; padding: 4px;"
//             />
//             <div style="margin: 8px 0;">
//                 <button on:click=on_save>"💾 保存"</button>
//                 <button on:click=on_load>"📂 加载"</button>
//                 <button on:click=on_delete>"🗑️ 删除"</button>
//             </div>
//             <p>"状态: " {move || status.get()}</p>
//             {move || saved_data.get().map(|data| view! {
//                 <div style="margin-top: 8px; padding: 8px; background: #f0f0f0; border-radius: 4px;">
//                     <p>"已保存的数据: " {data}</p>
//                 </div>
//             })}
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
// - IndexedDB 是浏览器内置的 NoSQL 数据库，数据持久保存
// - `#[wasm_bindgen(inline_js)]` 封装复杂浏览器 API 返回 Promise
// - `JsFuture::from(promise).await` 将 JS Promise 转为 Rust Future
// - `spawn_local` 在 Leptos CSR 中执行异步操作
// - `signal::<Option<String>>` 跟踪可选数据状态
//
// </details>
