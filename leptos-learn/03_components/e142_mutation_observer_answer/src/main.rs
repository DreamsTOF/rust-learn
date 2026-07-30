// ============================================================
// 练习 e142: MutationObserver — DOM 子树变化监听
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::MutationObserver;

#[component]
fn Exercise() -> impl IntoView {
    let (items, set_items) = signal(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    let (log, set_log) = signal(Vec::<String>::new());
    let container_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    let cb = Closure::wrap(Box::new(move |mutations: js_sys::Array| {
        for i in 0..mutations.length() {
            let record = mutations.get(i);
            let typ = js_sys::Reflect::get(&record, &"type".into())
                .unwrap()
                .as_string()
                .unwrap_or_default();
            let mut msg = format!("类型: {}", typ);

            if typ == "childList" {
                let added = js_sys::Reflect::get(&record, &"addedNodes".into()).unwrap();
                let removed = js_sys::Reflect::get(&record, &"removedNodes".into()).unwrap();
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

    let observer = MutationObserver::new(cb.as_ref().unchecked_ref()).unwrap();
    cb.forget();

    let options = web_sys::MutationObserverInit::new();
    options.set_child_list(true);
    options.set_subtree(true);

    Effect::new(move |_| {
        if let Some(el) = container_ref.get() {
            observer.observe_with_options(el.as_ref(), &options).unwrap();
        }
    });

    view! {
        <div>
            <h3>"练习 e142: MutationObserver"</h3>
            <p>"添加/删除下方列表中的元素，观察日志变化："</p>
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
                    let next = (items().len() as u8 + 65) as char;
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
}

fn main() {
    mount_to_body(Exercise);
}
