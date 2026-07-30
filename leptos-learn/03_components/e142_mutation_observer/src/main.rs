// ============================================================
// 练习 e142: MutationObserver — DOM 子树变化监听
//
// 核心知识点:
//   - MutationObserver 构造函数
//   - observe_with_options 配置监听范围
//   - MutationObserverInit 配置 childList / subtree
//   - 读取 MutationRecord 的 type、addedNodes、removedNodes
//
// 难度: ⭐⭐⭐ (理解 JS 回调转 Rust)
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::JsCast;

fn main() {
    mount_to_body(move || {
        // 存储待观测容器中的元素
        let (items, set_items) = signal(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
        // 存储变更日志
        let (log, set_log) = signal(Vec::<String>::new());
        // 容器引用，用于挂载后启动 MutationObserver
        let container_ref: NodeRef<leptos::html::Div> = NodeRef::new();

        // === 步骤 1: 创建 MutationObserver 回调 ===
        // TODO: 使用 Closure 包装回调，读取 mutations 数组
        let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move |mutations: js_sys::Array| {
            for i in 0..mutations.length() {
                let record = mutations.get(i);
                let typ = js_sys::Reflect::get(&record, &"type".into())
                    .unwrap()
                    .as_string()
                    .unwrap_or_default();
                let mut msg = format!("类型: {}", typ);

                if typ == "childList" {
                    let added = js_sys::Reflect::get(&record, &"addedNodes".into()).unwrap();
                    let removed =
                        js_sys::Reflect::get(&record, &"removedNodes".into()).unwrap();
                    let added_len = js_sys::Reflect::get(&added, &"length".into())
                        .unwrap()
                        .as_f64()
                        .unwrap() as i32;
                    let removed_len = js_sys::Reflect::get(&removed, &"length".into())
                        .unwrap()
                        .as_f64()
                        .unwrap() as i32;
                    if added_len > 0 {
                        msg.push_str(&format!(" | 添加了 {} 个节点", added_len));
                    }
                    if removed_len > 0 {
                        msg.push_str(&format!(" | 移除了 {} 个节点", removed_len));
                    }
                }
                set_log.update(|v| v.push(msg));
            }
        }) as Box<dyn Fn(js_sys::Array)>);

        // === 步骤 2: 创建 MutationObserver 实例 ===
        let observer =
            web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()).unwrap();
        cb.forget();

        // === 步骤 3: 配置选项（监听子节点添加/移除） ===
        // TODO: 创建 MutationObserverInit，设置 childList = true, subtree = true
        let options = web_sys::MutationObserverInit::new();
        options.set_child_list(true);
        options.set_subtree(true);

        // === 步骤 4: 在容器挂载后启动观察 ===
        Effect::new(move |_| {
            if let Some(el) = container_ref.get() {
                observer.observe_with_options(el.as_ref(), &options).unwrap();
            }
        });

        view! {
            <div>
                <p>"添加/删除下方列表中的元素，观察日志变化："</p>

                {/* 被观测的容器 */}
                <div
                    node_ref=container_ref
                    style="border: 2px solid #E67E22; padding: 8px; border-radius: 4px; min-height: 60px;"
                >
                    <ul>
                        {move || items()
                            .iter()
                            .enumerate()
                            .map(|(_, name)| {
                                let name = name.clone();
                                view! { <li>{name}</li> }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                </div>

                <div style="margin-top: 8px; display: flex; gap: 8px;">
                    <button on:click=move |_| {
                        let next = (items().len() as u8 + 65) as char; // A, B, C...
                        set_items.update(|v| v.push(next.to_string()));
                    }>"添加元素"</button>
                    <button on:click=move |_| {
                        set_items.update(|v| { v.pop(); });
                    }>"删除最后一个"</button>
                    <button on:click=move |_| {
                        set_log.set(Vec::new());
                    }>"清空日志"</button>
                </div>

                <div style="margin-top: 12px;">
                    <p><strong>"变更日志："</strong></p>
                    <ul style="font-family: monospace; font-size: 13px; color: #555;">
                        {move || {
                            let logs = log();
                            logs.iter().rev().take(10).map(|msg| {
                                let msg = msg.clone();
                                view! { <li>{msg}</li> }
                            }).collect::<Vec<_>>()
                        }}
                    </ul>
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
//         let (items, set_items) = signal(vec!["A", "B", "C"]);
//         let (log, set_log) = signal(Vec::<String>::new());
//         let container_ref: NodeRef<leptos::html::Div> = NodeRef::new();
//
//         let cb = wasm_bindgen::closure::Closure::wrap(
//             Box::new(move |mutations: js_sys::Array| {
//                 for i in 0..mutations.length() {
//                     let record = mutations.get(i);
//                     let typ = js_sys::Reflect::get(&record, &"type".into())
//                         .unwrap().as_string().unwrap_or_default();
//                     let mut msg = format!("类型: {}", typ);
//                     if typ == "childList" {
//                         let added = js_sys::Reflect::get(&record, &"addedNodes".into()).unwrap();
//                         let removed = js_sys::Reflect::get(&record, &"removedNodes".into()).unwrap();
//                         let added_len = js_sys::Reflect::get(&added, &"length".into())
//                             .unwrap().as_f64().unwrap() as i32;
//                         let removed_len = js_sys::Reflect::get(&removed, &"length".into())
//                             .unwrap().as_f64().unwrap() as i32;
//                         if added_len > 0 { msg.push_str(&format!(" | 添加了 {} 个节点", added_len)); }
//                         if removed_len > 0 { msg.push_str(&format!(" | 移除了 {} 个节点", removed_len)); }
//                     }
//                     set_log.update(|v| v.push(msg));
//                 }
//             }) as Box<dyn Fn(js_sys::Array)>,
//         );
//
//         let observer = web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()).unwrap();
//         cb.forget();
//
//         let mut options = web_sys::MutationObserverInit::new();
//         options.set_child_list(true);
//         options.set_subtree(true);
//
//         Effect::new(move |_| {
//             if let Some(el) = container_ref.get() {
//                 observer.observe_with_options(el.as_ref(), &options);
//             }
//         });
//
//         view! {
//             <div>
//                 <p>"添加/删除下方列表中的元素，观察日志变化："</p>
//                 <div
//                     node_ref=container_ref
//                     style="border: 2px solid #E67E22; padding: 8px; border-radius: 4px; min-height: 60px;"
//                 >
//                     <ul>{move || items().iter().enumerate().map(|(i, name)| {
//                         let name = name.clone();
//                         view! { <li>{name}</li> }
//                     }).collect::<Vec<_>>()}</ul>
//                 </div>
//                 <div style="margin-top: 8px; display: flex; gap: 8px;">
//                     <button on:click=move |_| {
//                         let next = (items().len() as u8 + 65) as char;
//                         set_items.update(|v| v.push(next.to_string()));
//                     }>"添加元素"</button>
//                     <button on:click=move |_| { set_items.update(|v| { v.pop(); }); }>"删除最后一个"</button>
//                     <button on:click=move |_| { set_log.set(Vec::new()); }>"清空日志"</button>
//                 </div>
//                 <div style="margin-top: 12px;">
//                     <p><strong>"变更日志："</strong></p>
//                     <ul style="font-family: monospace; font-size: 13px; color: #555;">
//                         {move || {
//                             let logs = log();
//                             logs.iter().rev().take(10).map(|msg| {
//                                 let msg = msg.clone();
//                                 view! { <li>{msg}</li> }
//                             }).collect::<Vec<_>>()
//                         }}
//                     </ul>
//                 </div>
//             </div>
//         }
//     });
// }
// ```
//
// ### 知识点
// 1. MutationObserver 监听 DOM 子树的结构变化
// 2. `set_child_list(true)` 监听子节点添加/移除
// 3. `set_subtree(true)` 将监听范围扩展到所有后代
// 4. MutationRecord 包含 `type`、`addedNodes`、`removedNodes` 等字段
// 5. `observe_with_options()` 用于传参配置
// </details>
