// ============================================================
// 练习 e368: 动画系统 — CSS transition/animation 受 Signal 控制
//
// 核心知识点:
//   - CSS transition 属性实现平滑动画
//   - 用 Signal 控制动画状态（展开/折叠、淡入/淡出）
//   - class: 语法动态切换 CSS class
//   - 动态 style 绑定控制样式值
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

// TODO: 实现动画系统
// 1. 展开/折叠: 用 expanded signal + class:active 语法控制 CSS class
// 2. 淡入/淡出: 用 visible signal + 动态 style 控制 opacity
// 3. CSS transition 定义在 <style> 标签中
// 4. class:active={move || expanded()} 当 expanded 为 true 时添加 active class

#[component]
fn Exercise() -> impl IntoView {
    let (expanded, set_expanded) = signal(false);
    let (visible, set_visible) = signal(true);

    // TODO: 添加切换函数
    // toggle_expanded = move || set_expanded.update(|v| *v = !*v)
    // toggle_visible = move || set_visible.update(|v| *v = !*v)

    view! {
        <div style="padding: 1rem; font-family: sans-serif; max-width: 480px; margin: 0 auto;">
            <h3>"练习 e368: 动画系统"</h3>

            <div style="margin-bottom: 16px;">
                <button on:click=move |_| set_expanded.update(|v| *v = !*v)>
                    {move || if expanded() { "折叠" } else { "展开" }}
                </button>
            </div>

            <div style="margin-bottom: 16px;">
                <button on:click=move |_| set_visible.update(|v| *v = !*v)>
                    {move || if visible() { "淡出" } else { "淡入" }}
                </button>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
