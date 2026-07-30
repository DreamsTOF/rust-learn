// ============================================================
// 练习 e361: focus_management — 焦点管理（自动对焦、焦点陷阱）
//
// 核心知识点:
//   - NodeRef 与 Effect::new 实现自动对焦
//   - 焦点陷阱（Focus Trap）限制 Tab 循环
//   - html::Input、html::Div 类型的使用
//
// 难度: ⭐⭐
// ============================================================

use leptos::ev::KeyboardEvent;
use leptos::html::{Div, Input};
use leptos::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

// 焦点陷阱的纯 JS 实现——避免 web-sys feature 限制
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

#[component]
fn Exercise() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();
    let trap_ref = NodeRef::<Div>::new();
    let (show_trap, set_show_trap) = signal(false);

    // 自动对焦：组件挂载后将焦点设置到输入框
    Effect::new(move || {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });

    let toggle_trap = move |_| {
        set_show_trap.update(|v| *v = !*v);
    };

    // 焦点陷阱：通过 inline JS 实现 Tab 循环
    let on_trap_keydown = move |ev: KeyboardEvent| {
        if ev.key() != "Tab" {
            return;
        }
        ev.prevent_default();
        if let Some(trap) = trap_ref.get() {
            trap_focus(trap.as_ref(), ev.shift_key());
        }
    };

    view! {
        <div style="max-width: 500px; margin: 20px auto; font-family: sans-serif;">
            <h3>"焦点管理演示"</h3>

            <section style="margin-bottom: 24px;">
                <h4>"1. 自动对焦"</h4>
                <p style="color: #666; font-size: 0.9em;">
                    "页面加载后输入框自动获得焦点"
                </p>
                <input
                    node_ref={input_ref}
                    placeholder="自动获得焦点..."
                    style="width: 100%; padding: 8px; border: 1px solid #ccc; border-radius: 4px;"
                />
            </section>

            <section style="margin-bottom: 24px;">
                <h4>"2. 焦点陷阱（Focus Trap）"</h4>
                <button on:click={toggle_trap} style="padding: 8px 16px;">
                    {move || if show_trap() { "关闭焦点陷阱" } else { "打开焦点陷阱" }}
                </button>

                {move || show_trap().then(|| {
                    view! {
                        <div
                            node_ref={trap_ref}
                            tabindex="-1"
                            style="border: 2px solid #e74c3c; border-radius: 8px; padding: 20px;
                                   margin-top: 12px; background: #fdf2f2; outline: none;"
                            on:keydown={on_trap_keydown}
                        >
                            <p style="margin-top: 0; font-weight: bold; color: #c0392b;">
                                "⚠ 焦点陷阱已激活（Tab 将在容器内循环）"
                            </p>
                            <div style="display: flex; flex-direction: column; gap: 8px;">
                                <input placeholder="输入框 1"
                                    style="padding: 6px; border: 1px solid #ccc; border-radius: 4px;" />
                                <input placeholder="输入框 2"
                                    style="padding: 6px; border: 1px solid #ccc; border-radius: 4px;" />
                                <button style="padding: 6px 12px; background: #3498db; color: white;
                                    border: none; border-radius: 4px; cursor: pointer;">
                                    "确认按钮"
                                </button>
                                <button style="padding: 6px 12px; background: #95a5a6; color: white;
                                    border: none; border-radius: 4px; cursor: pointer;">
                                    "取消按钮"
                                </button>
                            </div>
                            <p style="margin-bottom: 0; font-size: 0.85em; color: #888; margin-top: 12px;">
                                "提示：按 Esc 或点击外部可关闭陷阱"
                            </p>
                        </div>
                    }
                })}
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
