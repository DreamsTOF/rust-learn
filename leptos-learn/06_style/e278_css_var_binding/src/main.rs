// ============================================================
// 练习 e278: CSS 变量绑定 — style:--var 语法
//
// 核心知识点:
//   - style:--custom-prop 绑定 CSS 自定义属性（变量）
//   - 响应式 CSS 变量更新
//   - CSS 变量与常规 style: 指令结合使用
//
// 难度: ⭐⭐ (补全关键代码)
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

            // TODO: 为 div 添加两个 style: 指令:
            //   - style:--primary 绑定到 primary 信号（声明 CSS 变量）
            //   - style:background-color 也绑定到 primary 信号
            // 提示: style:--primary={move || primary.get()}
            <div>
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
