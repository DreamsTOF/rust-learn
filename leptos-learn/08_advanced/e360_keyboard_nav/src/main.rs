// ============================================================
// 练习 e360: 键盘导航 — 键盘导航模式（Tab 键顺序、箭头键操作）
//
// 核心知识点:
//   - on:keydown 事件处理箭头键上下移动
//   - Tab 键顺序管理（tabindex 属性）
//   - Enter/Space 触发选中
//   - 视觉焦点指示器
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
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
    // TODO: 创建信号 tracking 当前焦点索引
    // let (focused_index, set_focused_index) = signal(0);

    // TODO: 创建信号 tracking 选中项
    // let (selected_item, set_selected_item) = signal::<Option<usize>>(None);

    // TODO: 处理 keydown 事件
    // 箭头下: index += 1 (循环)
    // 箭头上: index -= 1 (循环)
    // Enter/Space: 选中当前项

    view! {
        <div style="max-width: 500px; margin: 2rem auto; font-family: system-ui, sans-serif; padding: 0 1rem;">
            <h1>"⌨️ 键盘导航菜单"</h1>

            <div style="margin-bottom: 1rem; padding: 1rem; background: #e3f2fd; border-radius: 8px; font-size: 0.9rem;">
                <p><strong>"操作指南："</strong></p>
                <p>"Tab 进入菜单 → 方向键 ↑↓ 移动 → Enter 选中 → Esc 取消"</p>
            </div>

            // TODO: 渲染菜单列表
            // - 每个菜单项有 role="menuitem"
            // - 焦点项使用 tabindex="0"，非焦点项使用 tabindex="-1"
            // - 焦点项具有高亮背景
            // - 处理 on:keydown 事件
            // - 显示当前选中项

            <div role="menu" attr:aria-label="导航菜单"
                style="background: white; border: 1px solid #e0e0e0; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                // 第一项示例（硬编码，需改为循环生成）
                <div
                    role="menuitem"
                    tabindex="0"
                    style="padding: 12px 16px; cursor: pointer; border-bottom: 1px solid #f0f0f0; background: #e3f2fd; font-weight: 500;"
                >
                    "🏠 首页"
                </div>
                <div
                    role="menuitem"
                    tabindex="-1"
                    style="padding: 12px 16px; cursor: pointer; border-bottom: 1px solid #f0f0f0;"
                >
                    "📁 文件"
                </div>
            </div>

            // TODO: 显示当前选中项
            // <div style="margin-top: 1rem; padding: 1rem; background: #e8f5e9; border-radius: 8px;">
            //     "已选中: ..."
            // </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
