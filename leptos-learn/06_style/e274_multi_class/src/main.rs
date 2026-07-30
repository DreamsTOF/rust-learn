use leptos::prelude::*;

// 练习目标：根据多个条件控制多个 CSS 类
//
// 知识点：
// - 在同一个元素上使用多个 class: 指令
// - 每个 class: 独立控制一个 CSS 类

#[component]
fn Exercise() -> impl IntoView {
    // 创建多个信号，控制不同样式
    let (is_bold, set_is_bold) = signal(false);
    let (is_italic, set_is_italic) = signal(false);
    let (is_highlight, set_is_highlight) = signal(false);

    view! {
        <div style="padding: 20px;">
            <h2>"多重 CSS 类控制"</h2>

            <div
                /* TODO: 添加 class:bold 指令，绑定 is_bold */
                /* TODO: 添加 class:italic 指令，绑定 is_italic */
                /* TODO: 添加 class:highlight 指令，绑定 is_highlight */
                style="padding: 20px; border: 1px solid #ccc; border-radius: 8px; transition: all 0.3s; margin: 10px 0; font-size: 18px;"
            >
                <p>"这段文字的样式由下方按钮控制"</p>
                <p>"当前样式："
                    {move || if is_bold.get() { "【粗体】" } else { "" }}
                    {move || if is_italic.get() { "【斜体】" } else { "" }}
                    {move || if is_highlight.get() { "【高亮】" } else { "" }}
                </p>
            </div>

            <div style="display: flex; gap: 8px; margin: 10px 0;">
                <button on:click=move |_| set_is_bold.update(|v| *v = !*v)>
                    {move || if is_bold.get() { "取消粗体" } else { "粗体" }}
                </button>
                <button on:click=move |_| set_is_italic.update(|v| *v = !*v)>
                    {move || if is_italic.get() { "取消斜体" } else { "斜体" }}
                </button>
                <button on:click=move |_| set_is_highlight.update(|v| *v = !*v)>
                    {move || if is_highlight.get() { "取消高亮" } else { "高亮" }}
                </button>
            </div>

            <style>
                ".bold { font-weight: bold; }
                .italic { font-style: italic; }
                .highlight {
                    background-color: #ffeb3b;
                    border-color: #f9a825 !important;
                }"
            </style>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
