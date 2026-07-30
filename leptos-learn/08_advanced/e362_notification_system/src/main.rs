// ============================================================
// 练习 e362: notification_system — 桌面通知系统
//
// 核心知识点:
//   - 浏览器 Notification API 的 wasm_bindgen 绑定
//   - 通知权限请求（requestPermission）
//   - JsFuture / spawn_local 异步处理
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// 通过 inline JS 绑定浏览器 Notification API
#[wasm_bindgen(inline_js = r#"
export function request_notification_permission() {
    return Notification.requestPermission();
}
export function send_notification(title, body) {
    if (Notification.permission === 'granted') {
        new Notification(title, { body });
    }
}
"#)]
extern "C" {
    fn request_notification_permission() -> js_sys::Promise;
    fn send_notification(title: &str, body: &str);
}

#[component]
fn Exercise() -> impl IntoView {
    let (permission, set_permission) = signal("尚未请求".to_string());
    let (title, set_title) = signal(String::new());
    let (body, set_body) = signal(String::new());
    let (history, set_history) = signal::<Vec<(String, String)>>(Vec::new());

    // === 步骤 1: 请求通知权限 ——————————————
    // TODO: 使用 spawn_local + JsFuture 调用 request_notification_permission()
    // 获取结果后调用 set_permission.set() 更新权限状态
    let on_request_permission = move |_| {
        spawn_local(async {
            // 你的代码开始
            todo!("实现通知权限请求");
            // 你的代码结束
        });
    };

    // === 步骤 2: 发送桌面通知 ——————————————
    // TODO: 调用 send_notification(&title, &body) 发送通知
    // 将已发送通知加入 history，并清空输入
    let on_send = move |_| {
        // 你的代码开始
        todo!("实现发送通知逻辑");
        // 你的代码结束
    };

    view! {
        <div style="max-width: 500px; margin: 20px auto; font-family: sans-serif;">
            <h3>"桌面通知系统"</h3>

            <section style="margin-bottom: 20px;">
                <h4>"权限状态"</h4>
                <p>
                    "当前权限: "
                    <strong>{move || permission()}</strong>
                </p>
                <button on:click={on_request_permission}
                    style="padding: 8px 16px; background: #3498db; color: white;
                           border: none; border-radius: 4px; cursor: pointer;">
                    "请求通知权限"
                </button>
            </section>

            <section style="margin-bottom: 20px;">
                <h4>"发送通知"</h4>
                <div style="display: flex; flex-direction: column; gap: 8px;">
                    <input
                        prop:value={title}
                        on:input={move |ev| set_title.set(event_target_value(&ev))}
                        placeholder="通知标题"
                        style="padding: 8px; border: 1px solid #ccc; border-radius: 4px;"
                    />
                    <textarea
                        prop:value={body}
                        on:input={move |ev| set_body.set(event_target_value(&ev))}
                        placeholder="通知正文"
                        rows="3"
                        style="padding: 8px; border: 1px solid #ccc; border-radius: 4px; resize: vertical;"
                    ></textarea>
                    <button on:click={on_send}
                        style="padding: 8px 16px; background: #2ecc71; color: white;
                               border: none; border-radius: 4px; cursor: pointer;">
                        "发送桌面通知"
                    </button>
                </div>
            </section>

            <section>
                <h4>"发送历史"</h4>
                {move || {
                    let list = history();
                    if list.is_empty() {
                        return view! { <p style="color: #999;">"暂无通知记录"</p> }.into_any();
                    }
                    view! {
                        <ul style="list-style: none; padding: 0;">
                            {list.into_iter().rev().map(|(t, b)| {
                                view! {
                                    <li style="padding: 8px; margin-bottom: 4px;
                                               background: #f9f9f9; border-radius: 4px;
                                               border-left: 3px solid #3498db;">
                                        <strong>{t}</strong>
                                        <p style="margin: 4px 0 0 0; color: #666; font-size: 0.9em;">
                                            {b}
                                        </p>
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    }.into_any()
                }}
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
