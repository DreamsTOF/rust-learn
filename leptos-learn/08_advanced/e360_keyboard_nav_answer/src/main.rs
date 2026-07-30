// ============================================================
// 练习 e360: 键盘导航 — 键盘导航模式（Tab 键顺序、箭头键操作）
//
// 核心知识点:
//   - on:keydown 事件处理箭头键上下移动
//   - Tab 键顺序管理（tabindex 属性）
//   - Enter/Space 触发选中
//   - 视觉焦点指示器
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

/// 菜单项数据
const MENU_ITEMS: &[&str] = &[
    "🏠 首页",
    "📁 文件",
    "⚙️ 设置",
    "👤 用户管理",
    "📊 报表",
];

#[component]
fn Exercise() -> impl IntoView {
    let (focused_index, set_focused_index) = signal(0_usize);
    let (selected_index, set_selected_index) = signal::<Option<usize>>(None);

    // 键盘导航处理
    let handle_keydown = move |ev: leptos::ev::KeyboardEvent| {
        let len = MENU_ITEMS.len();
        let current = focused_index.get();

        match ev.key().as_str() {
            "ArrowDown" | "ArrowRight" => {
                ev.prevent_default();
                set_focused_index.set((current + 1) % len);
            }
            "ArrowUp" | "ArrowLeft" => {
                ev.prevent_default();
                set_focused_index.set((current + len - 1) % len);
            }
            "Enter" | " " => {
                ev.prevent_default();
                set_selected_index.set(Some(current));
            }
            "Escape" => {
                ev.prevent_default();
                set_selected_index.set(None);
            }
            _ => {}
        }
    };

    view! {
        <div style="max-width: 500px; margin: 2rem auto; font-family: system-ui, sans-serif; padding: 0 1rem;">
            <h1>"⌨️ 键盘导航菜单"</h1>

            <div style="margin-bottom: 1rem; padding: 1rem; background: #e3f2fd; border-radius: 8px; font-size: 0.9rem;">
                <p><strong>"操作指南："</strong></p>
                <p>"Tab 进入菜单 → 方向键 ↑↓ 移动 → Enter 选中 → Esc 取消"</p>
            </div>

            <div
                role="menu"
                attr:aria-label="导航菜单"
                on:keydown=handle_keydown
                style="background: white; border: 1px solid #e0e0e0; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 8px rgba(0,0,0,0.1);"
            >
                {MENU_ITEMS.iter().enumerate().map(|(idx, item)| {
                    let is_focused = focused_index.get() == idx;
                    view! {
                        <div
                            role="menuitem"
                            tabindex=if is_focused { "0" } else { "-1" }
                            style=format!(
                                "padding: 12px 16px; cursor: pointer; border-bottom: 1px solid #f0f0f0; \
                                 background: {}; font-weight: {}; transition: background 0.15s; outline: none;",
                                if is_focused { "#e3f2fd" } else { "white" },
                                if is_focused { "600" } else { "400" },
                            )
                            on:click={let set = set_selected_index.clone(); move |_| { set.set(Some(idx)); }}
                        >
                            {*item}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            {move || {
                selected_index.get().map(|idx| {
                    view! {
                        <div style="margin-top: 1rem; padding: 1rem; background: #e8f5e9; color: #2e7d32; border-radius: 8px; font-weight: 500;">
                            "✅ 已选中: " {MENU_ITEMS[idx]}
                        </div>
                    }
                })
            }}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
