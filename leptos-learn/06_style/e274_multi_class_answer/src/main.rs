use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (is_bold, set_is_bold) = signal(false);
    let (is_italic, set_is_italic) = signal(false);
    let (is_highlight, set_is_highlight) = signal(false);

    view! {
        <div style="padding: 20px;">
            <h2>"多重 CSS 类控制"</h2>

            <div
                class:bold={move || is_bold.get()}
                class:italic={move || is_italic.get()}
                class:highlight={move || is_highlight.get()}
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
