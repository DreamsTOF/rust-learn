// ============================================================
// 练习 e276: 多 style 指令 — 同时控制多个 CSS 属性
//
// 核心知识点:
//   - style:xxx 指令绑定单个 CSS 属性
//   - 多个 style: 指令组合使用
//   - 响应式样式绑定（信号驱动）
//
// 难度: ⭐⭐ (补全关键代码)
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 声明响应式信号
    let (bg_color, set_bg_color) = signal(String::from("#e0f7fa"));
    let (text_color, set_text_color) = signal(String::from("#006064"));
    let (font_size, set_font_size) = signal(18);

    view! {
        <div>
            <h2>"多 style 指令示例"</h2>

            // TODO: 为 div 添加三个 style: 指令
            //   - style:background-color 绑定 bg_color
            //   - style:color 绑定 text_color
            //   - style:font-size 绑定 font_size（格式: "{size}px"）
            // 提示: style:background-color={move || bg_color.get()}
            <div>
                "这段文本的样式由多个 style: 指令控制。"
            </div>

            <div>
                <label>
                    "背景色: "
                    // TODO: 添加 on:input 事件，通过 set_bg_color 更新背景色
                    <input type="color" />
                </label>
                <label>
                    "文字色: "
                    // TODO: 添加 on:input 事件，通过 set_text_color 更新文字色
                    <input type="color" />
                </label>
                <label>
                    "字号: "
                    // TODO: 添加 on:input 事件，解析值为 i32 后通过 set_font_size 更新字号
                    <input type="range" min="12" max="36" />
                    {move || format!("{}px", font_size.get())}
                </label>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
