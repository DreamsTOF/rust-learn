// ============================================================
// 练习 e278: CSS 变量绑定 — 参考答案
//
// 核心知识点:
//   - style:--custom-prop 绑定 CSS 自定义属性（变量）
//   - 响应式 CSS 变量更新
//   - CSS 变量与常规 style: 指令结合使用
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (primary, set_primary) = signal(String::from("#3498db"));

    view! {
        <div>
            <h2>"CSS 变量绑定"</h2>

            <label>
                "选择主色: "
                <input type="color"
                    on:input=move |ev| set_primary(event_target_value(&ev))
                />
            </label>

            <div
                style:--primary={move || primary.get()}
                style:background-color={move || primary.get()}
                style:color="white"
                style:padding="1rem"
                style:border-radius="8px"
            >
                "带有 CSS 变量绑定的元素"
            </div>

            <p>
                "当前主色: "
                {move || primary.get()}
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
