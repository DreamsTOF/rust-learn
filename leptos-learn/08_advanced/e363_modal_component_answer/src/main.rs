// ============================================================
// 练习 e363: modal_component — 可复用模态框组件
//
// 核心知识点:
//   - Portal 将模态框渲染到 body 层级
//   - Callback 类型传递关闭回调
//   - 背景遮罩 + ESC 键关闭
//   - 焦点陷阱集成
//
// 难度: ⭐⭐
// ============================================================

use std::sync::Arc;
use leptos::ev::KeyboardEvent;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// 焦点陷阱的纯 JS 实现
#[wasm_bindgen(inline_js = r#"
export function trap_focus(container, shiftKey) {
    if (!container) return;
    const sel = 'input, button, select, textarea, a[href], [tabindex]:not([tabindex="-1"])';
    const nodes = container.querySelectorAll(sel);
    if (nodes.length === 0) return;
    const active = document.activeElement;
    const idx = Array.prototype.indexOf.call(nodes, active);
    if (shiftKey) {
        nodes[idx <= 0 ? nodes.length - 1 : idx - 1].focus();
    } else {
        nodes[idx >= nodes.length - 1 ? 0 : idx + 1].focus();
    }
}
"#)]
extern "C" {
    fn trap_focus(container: &JsValue, shift_key: bool);
}

/// 可复用模态框组件
#[component]
fn Modal(
    title: String,
    show: ReadSignal<bool>,
    on_close: Callback<()>,
    children: ChildrenFn,
) -> impl IntoView {
    // 使用 Arc 包装非 Copy 值，使其在 view! 宏生成的闭包中可安全共享
    let title_text = Arc::new(title);
    let modal_ref = NodeRef::<leptos::html::Div>::new();

    // 通过 CSS display 控制显隐
    let show_style = move || {
        if show() {
            "position: fixed; inset: 0; z-index: 1000;
             display: flex; align-items: center; justify-content: center;
             background: rgba(0, 0, 0, 0.5);
             animation: modalFadeIn 0.2s ease;"
        } else {
            "display: none;"
        }
    };

    // ESC 键关闭 + Tab 键焦点陷阱
    let on_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Escape" {
            on_close.run(());
        }
        if ev.key() == "Tab" {
            if let Some(modal) = modal_ref.get() {
                ev.prevent_default();
                trap_focus(modal.as_ref(), ev.shift_key());
            }
        }
    };

    // 背景遮罩点击关闭（内容区已 stopPropagation）
    let on_backdrop_click = move |_ev: leptos::ev::MouseEvent| {
        on_close.run(());
    };

    view! {
        <Portal>
            <div
                style={show_style}
                on:click={on_backdrop_click}
                on:keydown={on_keydown}
                tabindex="0"
            >
                <div
                    node_ref={modal_ref}
                    style="background: white; border-radius: 12px; padding: 0;
                           max-width: 480px; width: 90%;
                           box-shadow: 0 8px 32px rgba(0,0,0,0.2);
                           animation: modalSlideIn 0.25s ease; outline: none;"
                    on:click={move |ev| ev.stop_propagation()}
                    tabindex="-1"
                >
                    <div style="display: flex; justify-content: space-between;
                                align-items: center; padding: 16px 24px;
                                border-bottom: 1px solid #eee;">
                        <h3 style="margin: 0; font-size: 1.1em;">{title_text.as_ref().clone()}</h3>
                        <button on:click={move |_| on_close.run(())}
                            style="background: none; border: none; font-size: 22px;
                                   cursor: pointer; color: #999; padding: 4px 8px;
                                   border-radius: 4px; line-height: 1;">
                            "✕"
                        </button>
                    </div>
                    <div style="padding: 24px;">
                        {children()}
                    </div>
                </div>
            </div>
        </Portal>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (show_modal, set_show_modal) = signal(false);
    let (input_value, set_input_value) = signal(String::new());

    view! {
        <div style="max-width: 500px; margin: 20px auto; font-family: sans-serif;">
            <h3>"模态框演示"</h3>

            <button on:click={move |_| set_show_modal.set(true)}
                style="padding: 10px 20px; background: #3498db; color: white;
                       border: none; border-radius: 6px; cursor: pointer; font-size: 1em;">
                "打开模态框"
            </button>

            <p style="margin-top: 20px; color: #666;">
                "已输入: "
                <strong>{move || input_value()}</strong>
            </p>

            <Modal
                title="用户信息".to_string()
                show={show_modal}
                on_close={Callback::new(move |_| set_show_modal.set(false))}
            >
                <p>"请在下方输入您的信息："</p>
                <input
                    prop:value={input_value}
                    on:input={move |ev| set_input_value.set(event_target_value(&ev))}
                    placeholder="输入内容..."
                    style="width: 100%; padding: 8px; border: 1px solid #ccc;
                           border-radius: 4px; box-sizing: border-box;"
                />
                <div style="margin-top: 16px; display: flex; gap: 8px; justify-content: flex-end;">
                    <button on:click={move |_| set_show_modal.set(false)}
                        style="padding: 8px 16px; background: #95a5a6; color: white;
                               border: none; border-radius: 4px; cursor: pointer;">
                        "取消"
                    </button>
                    <button on:click={move |_| set_show_modal.set(false)}
                        style="padding: 8px 16px; background: #2ecc71; color: white;
                               border: none; border-radius: 4px; cursor: pointer;">
                        "确认"
                    </button>
                </div>
            </Modal>

            <style>
                "@keyframes modalFadeIn {
                    from { opacity: 0; }
                    to { opacity: 1; }
                }
                @keyframes modalSlideIn {
                    from { transform: translateY(-20px); opacity: 0; }
                    to { transform: translateY(0); opacity: 1; }
                }"
            </style>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
