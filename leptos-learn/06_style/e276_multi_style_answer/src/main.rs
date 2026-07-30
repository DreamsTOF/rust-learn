// ============================================================
// 练习 e276: 多 style 指令 — 参考答案
//
// 核心知识点:
//   - style:xxx 指令绑定单个 CSS 属性
//   - 多个 style: 指令组合使用
//   - 响应式样式绑定（信号驱动）
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (bg_color, set_bg_color) = signal(String::from("#e0f7fa"));
    let (text_color, set_text_color) = signal(String::from("#006064"));
    let (font_size, set_font_size) = signal(18);

    view! {
        <div>
            <h2>"多 style 指令示例"</h2>

            <div
                style:background-color={move || bg_color.get()}
                style:color={move || text_color.get()}
                style:font-size={move || format!("{}px", font_size.get())}
            >
                "这段文本的样式由多个 style: 指令控制。"
            </div>

            <div>
                <label>
                    "背景色: "
                    <input type="color"
                        on:input=move |ev| set_bg_color(event_target_value(&ev))
                    />
                </label>
                <label>
                    "文字色: "
                    <input type="color"
                        on:input=move |ev| set_text_color(event_target_value(&ev))
                    />
                </label>
                <label>
                    "字号: "
                    <input type="range" min="12" max="36"
                        on:input=move |ev| set_font_size(
                            event_target_value(&ev).parse::<i32>().unwrap_or(18)
                        )
                    />
                    {move || format!("{}px", font_size.get())}
                </label>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
