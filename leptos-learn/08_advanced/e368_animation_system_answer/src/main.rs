// ============================================================
// 参考答案 e368: 动画系统 — CSS transition/animation 受 Signal 控制
//
// 核心知识点:
//   - CSS transition 属性实现平滑动画
//   - 用 Signal 控制动画状态（展开/折叠、淡入/淡出）
//   - class: 语法动态切换 CSS class
//   - 动态 style 绑定控制样式值
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (expanded, set_expanded) = signal(false);
    let (visible, set_visible) = signal(true);

    let toggle_expanded = move |_| set_expanded.update(|v| *v = !*v);
    let toggle_visible = move |_| set_visible.update(|v| *v = !*v);

    view! {
        <div style="padding: 1rem; font-family: sans-serif; max-width: 480px; margin: 0 auto;">
            <h3>"练习 e368: 动画系统"</h3>

            <div style="margin-bottom: 16px;">
                <button on:click=toggle_expanded>
                    {move || if expanded() { "折叠 ▲" } else { "展开 ▼" }}
                </button>
                <div
                    class="animation-box"
                    class:active={move || expanded()}
                    style="max-height: {move || if expanded() { \"200px\" } else { \"0\" }}; overflow: hidden; transition: max-height 0.4s ease; background: #f0f8ff; border-radius: 8px; padding: {move || if expanded() { \"12px\" } else { \"0 12px\" }}; margin-top: 8px;"
                >
                    <p>"这是可展开的内容区域。使用了 max-height 过渡动画。"</p>
                    <p>"当 expanded 为 true 时，max-height 变为 200px。"</p>
                </div>
            </div>

            <div style="margin-bottom: 16px;">
                <button on:click=toggle_visible>
                    {move || if visible() { "淡出 ✖" } else { "淡入 ●" }}
                </button>
                <div
                    class="animation-box"
                    style="opacity: {move || if visible() { \"1\" } else { \"0\" }}; transition: opacity 0.5s ease; background: #fff0f0; border-radius: 8px; padding: 12px; margin-top: 8px;"
                >
                    <p>"这是淡入/淡出的内容区域。使用了 opacity 过渡动画。"</p>
                    <p>"当 visible 为 false 时，opacity 变为 0。"</p>
                </div>
            </div>

            <style>
                ".animation-box {
                    overflow: hidden;
                    transition: max-height 0.4s ease, opacity 0.5s ease;
                    background: #f0f8ff;
                    border-radius: 8px;
                    padding: 12px;
                    margin-top: 8px;
                }
                .animation-box.active {
                    max-height: 200px;
                }"
            </style>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
