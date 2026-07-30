use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 创建 HSL 颜色控制信号
    let (hue, set_hue) = signal(0);
    let (saturation, set_saturation) = signal(50);
    let (lightness, set_lightness) = signal(50);

    // TODO: 创建 style 派生信号
    // 使用 move || { format!(...) } 生成 CSS 字符串
    // 格式: "background-color: hsl({hue}, {saturation}%, {lightness}%); width: 200px; height: 200px; border-radius: 8px; transition: background-color 0.3s ease;"
    let style = move || {
        // 补全样式字符串
        String::new()
    };

    view! {
        <div>
            <h2>"CSS in Rust — 动态样式"</h2>

            // TODO: 添加三个 range 输入框
            // - hue: min=0, max=360
            // - saturation: min=0, max=100
            // - lightness: min=0, max=100
            // - 每个输入框绑定 on:input 事件更新对应的信号
            // - 显示当前值

            // TODO: 添加预览 div
            // - style 属性绑定到 {style}
            // - 设置固定宽高

            <p>"提示：完成 style 闭包和控件绑定，预览方块的颜色会随滑块变化。"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
