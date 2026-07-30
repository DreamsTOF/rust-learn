use leptos::prelude::*;

// 练习目标：用信号控制元素的 style 属性（颜色、大小）
//
// 知识点：
// - style:color={move || color.get()}：动态绑定 CSS 属性
// - style:font-size={move || ...}：带单位的属性值绑定
// - event_target_value(&ev)：从事件中获取输入值

#[component]
fn Exercise() -> impl IntoView {
    // 创建颜色信号，初始为 #3498db
    let (color, set_color) = signal(String::from("#3498db"));
    // 创建字号信号，初始为 16
    let (font_size, set_font_size) = signal(16);

    view! {
        <div style="padding: 20px;">
            <h2>"动态样式"</h2>

            { /* 使用 style:color 和 style:font-size 指令动态绑定样式 */ }
            <div
                style:color={move || color.get()}
                style:font-size={move || format!("{}px", font_size.get())}
                style="padding: 20px; border: 1px solid #ccc; border-radius: 8px; margin: 10px 0; transition: all 0.3s;"
            >
                <p>"这段文字的颜色和大小会随下方控制变化"</p>
                <p>"当前颜色：" {move || color.get()}</p>
                <p>"当前字号：" {move || format!("{}px", font_size.get())}</p>
            </div>

            <div style="margin: 10px 0;">
                <label>"选择颜色："</label>
                <input
                    type="color"
                    value={move || color.get()}
                    on:input=move |ev| set_color.set(event_target_value(&ev))
                />
            </div>

            <div style="margin: 10px 0;">
                <label>"字号（12-40px）："</label>
                <input
                    type="range"
                    min="12"
                    max="40"
                    value={move || font_size.get().to_string()}
                    on:input=move |ev| {
                        if let Ok(v) = event_target_value(&ev).parse::<i32>() {
                            set_font_size.set(v);
                        }
                    }
                />
                <span>" " {move || font_size.get()}</span>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
